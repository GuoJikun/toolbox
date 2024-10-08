use std::{ffi::OsString, os::windows::ffi::OsStringExt, ptr::null_mut};
use tauri::{AppHandle, Manager};
use windows::{
    core::{IUnknown, Interface, GUID, PCWSTR, PWSTR, VARIANT},
    Win32::{
        Foundation::{BOOL, HWND, LPARAM, LRESULT, WPARAM},
        System::
            Com::{
                CoCreateInstance, CoInitializeEx, CoTaskMemFree, CoUninitialize, IDispatch,
                IServiceProvider, CLSCTX_ALL, CLSCTX_INPROC_SERVER,
                COINIT_APARTMENTTHREADED, COINIT_DISABLE_OLE1DDE,
            }
        ,
        UI::{
            Input::KeyboardAndMouse,
            Shell::{
                Common::{ITEMIDLIST, STRRET},
                IEnumIDList, IInitializeWithItem, IPreviewHandler, IShellBrowser,
                IShellFolder, IShellItem, IShellView, IShellWindows,
                IWebBrowser2, SHCreateItemFromParsingName, SHGetDesktopFolder, ShellWindows,
                SHGDN_FORPARSING, SVGIO_SELECTION,
            },
            WindowsAndMessaging,
        },
    },
};

#[derive(Debug)]
pub struct PreviewFile {
    selected_file: Option<String>,                   // 存储选中的文件路径
    hook_handle: Option<WindowsAndMessaging::HHOOK>, // 钩子的句柄
    app_handle: Option<AppHandle>,
}

fn get_lpsz_class(str: &str) -> PCWSTR {
    return PCWSTR::from_raw(str.encode_utf16().collect::<Vec<u16>>().as_ptr());
}

fn utf8_string_to_utf16_string(s: String) -> String {
    let u16 = s.encode_utf16().collect::<Vec<u16>>();
    return String::from_utf16_lossy(&u16.as_slice());
}

fn get_window_class_name(hwnd: HWND) -> Option<String> {
    let mut buffer = [0u16; 256];
    let len = unsafe { WindowsAndMessaging::GetClassNameW(hwnd, &mut buffer) };
    if len > 0 {
        Some(String::from_utf16_lossy(&buffer[..len as usize]))
    } else {
        None
    }
}

struct Explorer {}

impl Explorer {
    unsafe extern "system" fn enum_child_shell_tab_windows_proc(
        child_hwnd: HWND,
        l_param: LPARAM,
    ) -> BOOL {
        let class_name = utf8_string_to_utf16_string(String::from("ShellTabWindowClass"));

        let hwnd_class = get_window_class_name(child_hwnd);

        match hwnd_class {
            Some(target) => {
                if target.contains(&class_name) {
                    *(l_param.0 as *mut HWND) = child_hwnd;
                    return false.into();
                }
            }
            None => {
                return true.into();
            }
        }

        true.into()
    }
    fn get_shell_tab_window_hwnd(hwnd: HWND) -> Option<HWND> {
        unsafe {
            let mut shell_tab_window_hwnd: HWND = HWND::default();
            let _result = WindowsAndMessaging::EnumChildWindows(
                hwnd,
                Some(Self::enum_child_shell_tab_windows_proc),
                LPARAM(&mut shell_tab_window_hwnd as *mut _ as isize),
            );
            println!("shell_tab_window_hwnd is {:?}", shell_tab_window_hwnd);
            return Some(shell_tab_window_hwnd);
        }
        None
    }

    unsafe extern "system" fn enum_shell_tab_window_child_windows_proc(
        child_hwnd: HWND,
        l_param: LPARAM,
    ) -> BOOL {
        let class_name = utf8_string_to_utf16_string(String::from("ShellTabWindowClass"));

        let hwnd_class = get_window_class_name(child_hwnd);

        match hwnd_class {
            Some(target) => {
                println!("class name is {:?}; hwnd is {:?}", target, child_hwnd.0);
                if target.contains(&class_name) {
                    return false.into();
                }
            }
            None => {
                return true.into();
            }
        }

        true.into()
    }
    fn get_shell_tab_window_child(hwnd: HWND) -> Option<HWND> {
        unsafe {
            let mut child_hwnd: HWND = HWND::default();
            let result = WindowsAndMessaging::EnumChildWindows(
                hwnd,
                Some(Self::enum_shell_tab_window_child_windows_proc),
                LPARAM(&mut child_hwnd as *mut _ as isize),
            );
            println!("child_hwnd is {:?}", child_hwnd);
            return Some(child_hwnd);
        }
        None
    }

    fn get_select_file_path(hwnd: HWND) {
        let child_hwnd = Self::get_shell_tab_window_hwnd(hwnd);

        match child_hwnd {
            Some(target) => {
                Self::get_shell_tab_window_child(target);
            }
            None => {}
        }
    }
}

#[allow(dead_code)]
impl PreviewFile {
    pub fn new() -> Self {
        Self {
            selected_file: None,
            hook_handle: None,
            app_handle: None,
        }
    }

    // 注册键盘钩子
    pub fn set_keyboard_hook(&mut self) {
        unsafe {
            let hook_ex = WindowsAndMessaging::SetWindowsHookExW(
                WindowsAndMessaging::WH_KEYBOARD_LL,
                Some(Self::keyboard_proc), // 使用结构体的键盘回调
                None,                      // 当前进程实例句柄
                0,
            );
            match hook_ex {
                Ok(result) => {
                    self.hook_handle = Some(result);
                }
                Err(_) => {
                    self.hook_handle = None;
                }
            }
        }
    }

    // 取消键盘钩子
    pub fn remove_keyboard_hook(&mut self) {
        if let Some(hook) = self.hook_handle {
            unsafe {
                let _ = WindowsAndMessaging::UnhookWindowsHookEx(hook);
            }
            self.hook_handle = None;
        }
    }

    // 按键处理逻辑
    pub fn handle_key_down(&self, vk_code: u32) {
        if vk_code == KeyboardAndMouse::VK_SPACE.0 as u32 {
            Self::get_selected_file();
            println!(
                "selected_file is {:?} in PreviewFile Struct",
                &self.selected_file()
            );
            if let Some(ref file_path) = self.selected_file {
                println!("file path is {:?}", file_path);
                let target_window = self
                    .app_handle
                    .clone()
                    .unwrap()
                    .get_webview_window("previewFile")
                    .unwrap();
                let target_hwnd = target_window.hwnd().unwrap();
                let _ = Self::preview_file(file_path, target_hwnd);
            }
        }
    }

    // 全局键盘钩子的回调函数
    extern "system" fn keyboard_proc(ncode: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        if ncode == 0 && wparam.0 == WindowsAndMessaging::WM_KEYDOWN as usize {
            let kb_struct = unsafe { *(lparam.0 as *const WindowsAndMessaging::KBDLLHOOKSTRUCT) };
            let vk_code = kb_struct.vkCode;
            // 获取 PreviewFile 实例并处理按键事件
            if let Some(app) = unsafe { APP_INSTANCE.as_ref() } {
                app.handle_key_down(vk_code);
            }
        }
        unsafe { WindowsAndMessaging::CallNextHookEx(None, ncode, wparam, lparam) }
    }

    fn utf8_string_to_utf16_string(s: String) -> String {
        let mut u16 = s.encode_utf16().collect::<Vec<u16>>();
        while u16.len() < 256 {
            u16.push(0)
        }
        return String::from_utf16_lossy(&u16.as_slice());
    }

    // unsafe extern "system" fn enum_child_windows_proc(child_hwnd: HWND, l_param: LPARAM) -> BOOL {
    //     let class_name = {
    //         let mut buf: [u16; 256] = [0; 256];
    //         WindowsAndMessaging::GetClassNameW(child_hwnd, &mut buf);
    //         String::from_utf16_lossy(&buf)
    //     };
    //
    //     let sys_tree_view_32 = Self::utf8_string_to_utf16_string(String::from("SysTreeView32"));
    //
    //     if class_name == "SysListView32" {
    //         let item_count = WindowsAndMessaging::SendMessageW(child_hwnd, Controls::LVM_GETITEMCOUNT, WPARAM(0), LPARAM(0));
    //         let mut text_buffer: [u16; 256] = [0; 256];
    //         for i in 0..item_count.0 {
    //             let mut lvi: Controls::LVITEMW = Controls::LVITEMW {
    //                 mask: Controls::LVIF_TEXT,
    //                 iItem: i as i32,
    //                 iSubItem: 0,
    //                 cchTextMax: text_buffer.len() as i32,
    //                 ..Default::default()
    //             };
    //             let psz_text = text_buffer.as_mut_ptr();
    //             lvi.pszText = *psz_text.cast::<PWSTR>();
    //             lvi.cchTextMax = text_buffer.len() as i32;
    //
    //             WindowsAndMessaging::SendMessageW(child_hwnd, Controls::LVM_GETITEMTEXT, WPARAM(i as usize), LPARAM(&lvi as *const _ as isize));
    //
    //             if lvi.iItem == 1 { // 假设你只想获取第一个选中的文件
    //                 let file_name = String::from_utf16_lossy(&text_buffer);
    //                 println!("Selected file: {}", file_name);
    //
    //             }
    //         }
    //     } else if class_name.eq(&sys_tree_view_32) {
    //         let first_item = WindowsAndMessaging::SendMessageW(child_hwnd, Controls::TVM_GETNEXTITEM, WPARAM(Controls::TVGN_FIRSTVISIBLE as usize), LPARAM(0));
    //
    //         if first_item.0 != 0 {
    //             let mut tmp_text_buffer: [u16; 256] = [0; 256];
    //             // tmp_text_buffer[0] = 0;
    //
    //             let mut tvi_item = Controls::TVITEMW {
    //                 mask: Controls::TVIF_HANDLE | Controls::TVIF_TEXT | Controls::TVIF_IMAGE | Controls::TVIF_SELECTEDIMAGE,
    //                 hItem: Controls::HTREEITEM(first_item.0),
    //                 cchTextMax: tmp_text_buffer.len() as i32,
    //                 pszText: PWSTR(tmp_text_buffer.as_mut_ptr()),
    //                 ..Default::default()
    //             };
    //             println!("child_hwnd is {:?}", &child_hwnd);
    //             println!("tvi_item is {:?}", &tvi_item);
    //             let result = WindowsAndMessaging::PostMessageW(child_hwnd, Controls::TVM_GETITEM, WPARAM(0), LPARAM(&mut tvi_item as *const _ as isize));
    //             println!("result is {:?}", result);
    //             if result.is_ok() {
    //                 // 获取成功，处理获取到的文件名
    //                 println!("run success, result is {:?}", result.unwrap());
    //             }
    //
    //             let text_slice = std::slice::from_raw_parts(
    //                 tvi_item.pszText.as_ptr(),
    //                 tvi_item.cchTextMax as usize
    //             );
    //             let file_name = String::from_utf16_lossy(text_slice);
    //             println!("Selected item: {}", file_name)
    //         }
    //     }
    //     true.into() // 继续枚举
    // }
    fn get_lpsz_class(str: &str) -> PCWSTR {
        return PCWSTR::from_raw(str.encode_utf16().collect::<Vec<u16>>().as_ptr());
    }
    fn get_selected_file_path(hwnd: HWND) -> Option<String> {
        None
    }
    fn get_selected_file() -> Option<String> {
        unsafe {
            let hwnd = WindowsAndMessaging::GetForegroundWindow(); // 获取当前活动窗口句柄
            println!("hwnd is {:?}", hwnd);
            let mut class_name = [0u16; 256];
            WindowsAndMessaging::GetClassNameW(hwnd, &mut class_name);

            let class_name_str = String::from_utf16_lossy(&class_name);
            println!("className is {}", class_name_str);
            if class_name_str.contains("CabinetWClass") || class_name_str.contains("Progman") {
                // 窗口是文件管理器或桌面，开始获取选中的文件
                Explorer::get_select_file_path(hwnd);
                // return Self::get_selected_file_path(hwnd);
                if let Some(shell_view) = Self::get_shell_view(hwnd) {
                    println!("shell_view is {:?}", shell_view);
                    // 获取当前选中的文件路径
                    let selected_file = Self::get_selected_file_from_shell_view(shell_view);
                    return selected_file;
                }
            }
        };
        None
    }
    fn check_interface(dispatch: &IDispatch, interface_guid: &GUID) -> bool {
        unsafe {
            // 获取类型信息的数量
            let count = dispatch.GetTypeInfoCount().unwrap_or(0);
            for i in 0..count {
                let type_info = dispatch.GetTypeInfo(i, 1);
                if type_info.is_ok() {
                    let type_attr = type_info.unwrap().GetTypeAttr();
                    if type_attr.is_ok() {
                        let type_attr = type_attr.unwrap();

                        // 检查是否包含特定接口的 GUID
                        if (*type_attr).guid == *interface_guid {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    fn get_shell_view(hwnd: HWND) -> Option<IShellView> {
        unsafe {
            // 初始化 COM 库
            let com = CoInitializeEx(None, COINIT_DISABLE_OLE1DDE);
            if com.is_err() {
                return None;
            }

            let hr = CoCreateInstance(&ShellWindows, None, CLSCTX_ALL);
            // let hr = CoCreateInstance(&ShellWindows, None, CLSCTX_INPROC_HANDLER);
            if hr.is_err() {
                CoUninitialize(); // 清理 COM
                return None; // 创建 IShellWindows 失败
            }
            let mut found_shell_view = None;
            let shell_windows: IShellWindows = hr.unwrap();
            let count = shell_windows.Count().unwrap_or(0);

            for i in 0..count {
                let index = VARIANT::from(i);
                let item = shell_windows.Item(&index);

                if item.is_ok() {
                    let dispatch = item.unwrap();
                    let typeinfo_count = dispatch.GetTypeInfoCount().unwrap_or(0);
                    for t in 0..typeinfo_count {
                        let type_info = dispatch.GetTypeInfo(t, 1);
                        if type_info.is_ok() {
                            let type_info = type_info.unwrap();
                            let type_attr = type_info.GetTypeAttr();
                            if type_attr.is_ok() {
                                let type_attr = type_attr.unwrap();
                                let guid = (*type_attr).guid;
                                println!("guid is {:?}", guid);
                                println!("guid1 is {:?}", IShellBrowser::IID);

                                if guid == IShellBrowser::IID {
                                    let shell_browser = dispatch.cast::<IShellBrowser>();
                                    println!("found IShellBrowser: {:?}", shell_browser.unwrap());
                                } else if guid == IShellView::IID {
                                    println!("found IShellView")
                                } else if guid == IUnknown::IID {
                                    println!("found IUnknown")
                                } else if guid == IShellFolder::IID {
                                    println!("found IShellFolder")
                                } else if guid == IShellBrowser::IID {
                                    println!("found IShellBrowser")
                                } else if guid == IWebBrowser2::IID {
                                    println!("found IWebBrowser2")
                                } else if guid == IServiceProvider::IID {
                                    println!("found IServiceProvider")
                                } else {
                                    println!("found other interface: {:?}", guid);
                                }
                            }
                        }
                    }

                    let web_browser2 = dispatch.cast::<IWebBrowser2>();
                    match web_browser2 {
                        Ok(web_browser2) => {
                            let web_browser_hwnd = web_browser2.HWND();
                            let web_browser_document = web_browser2.Document();
                            let name = web_browser2.FullName();
                            let app = web_browser2.Application();
                            if web_browser_document.is_ok() && web_browser_hwnd.is_ok() {
                                let document = web_browser_document.unwrap();
                                // if document_window.unwrap() == hwnd {
                                //     let shell_view = web_browser2.query_active_shell_view();
                                //     println!("shell_viewis {:?}", shell_view);
                                // }
                            }
                        }
                        Err(e) => {
                            println!("Failed to cast IWebBrowser2, Err info {:?}", e);
                        }
                    }
                    // let shell_browser = dispatch.cast::<IShellBrowser>();
                    // match shell_browser {
                    //     Ok(shell_browser) => {
                    //         // 直接从 IShellBrowser 获取 IShellView
                    //         let shell_view_hwnd = shell_browser.GetWindow();
                    //         if shell_view_hwnd.unwrap() == hwnd {
                    //             let shell_view = shell_browser.QueryActiveShellView();
                    //             if shell_view.is_ok() {
                    //                 found_shell_view = shell_view.ok(); // 找到 IShellView，返回
                    //                 break;
                    //             }
                    //         }
                    //     }
                    //     Err(e) => {
                    //         println!("Failed to cast IShellBrowser, Err info {:?}", e);
                    //     }
                    // }
                }
            }
            // 清理 COM
            CoUninitialize();
            // 返回找到的 IShellView 或 None
            found_shell_view
        }
    }

    // This function converts a STRRET to a Rust String.
    fn strret_to_string(
        shell_folder: &IShellFolder,
        pidl: *mut ITEMIDLIST,
        strret: &mut STRRET,
    ) -> Option<String> {
        unsafe {
            let hresult = shell_folder.GetDisplayNameOf(pidl, SHGDN_FORPARSING, strret);
            if hresult.is_err() {
                return None;
            }

            match strret.uType {
                _strret_type_str => {
                    let p_str = &strret.Anonymous.cStr as *const _ as *const u16;
                    let wide_str = std::slice::from_raw_parts(p_str, strret.uType as usize / 2);
                    Some(OsString::from_wide(wide_str).to_string_lossy().into_owned())
                } // _ => None,
            }
        }
    }

    // This function retrieves the selected file from the IShellView.
    fn get_selected_file_from_shell_view(shell_view: IShellView) -> Option<String> {
        unsafe {
            let enum_id_list: Result<IEnumIDList, windows::core::Error> =
                shell_view.GetItemObject(SVGIO_SELECTION);
            if enum_id_list.is_err() {
                return None;
            }

            let shell_folder = SHGetDesktopFolder();
            if shell_folder.is_err() {
                return None;
            }

            let shell_folder = shell_folder.unwrap();

            // Enumerate through the selected items
            let mut pidl_array: [*mut ITEMIDLIST; 1] = [null_mut()];
            let mut fetched: u32 = 0;

            while enum_id_list
                .as_ref()
                .unwrap()
                .Next(&mut pidl_array, Some(&mut fetched))
                .is_ok()
                && fetched > 0
            {
                let mut strret = STRRET {
                    uType: 0,
                    Anonymous: Default::default(),
                };

                if let Some(file_path) =
                    Self::strret_to_string(&shell_folder, pidl_array[0], &mut strret)
                {
                    // 在返回之前释放 pidl_array[0] 内存
                    let _ = CoTaskMemFree(Some(pidl_array[0] as *mut std::ffi::c_void));
                    return Some(file_path);
                }

                // 释放每次获取的 ITEMIDLIST 内存
                let _ = CoTaskMemFree(Some(pidl_array[0] as *mut std::ffi::c_void));
            }
        }
        None
    }
    pub fn selected_file(&self) -> Option<&String> {
        self.selected_file.as_ref()
    }
    pub fn preview_file(file_path: &str, hwnd: HWND) -> windows::core::Result<()> {
        println!("Previewing file: {}", file_path);

        unsafe {
            // 初始化 COM
            let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);

            // 将文件路径转为宽字符串（Windows API 要求）
            let path: PWSTR = PWSTR(file_path.encode_utf16().collect::<Vec<u16>>().as_mut_ptr());

            // 创建文件的 ShellItem
            let shell_item: IShellItem = SHCreateItemFromParsingName(path, None)?;

            // 获取文件的预览处理器
            // let mut preview_handler: Option<IPreviewHandler> = None;
            let preview_handler: Result<IPreviewHandler, windows::core::Error> =
                CoCreateInstance(&IPreviewHandler::IID, None, CLSCTX_INPROC_SERVER);

            if let Ok(handler) = preview_handler {
                // 初始化文件
                let initialize_with_file = handler.cast::<IInitializeWithItem>()?;
                let _ = initialize_with_file.Initialize(&shell_item, 0);
                // 将预览窗口绑定到我们的窗口句柄
                let _ = handler.SetWindow(hwnd, null_mut());

                // 触发文件预览
                handler.DoPreview()?;
            }

            Ok(())
        }
    }
}

// 全局静态 APP 实例
static mut APP_INSTANCE: Option<PreviewFile> = None;

// 公开一个全局函数来初始化 PreviewFile
pub fn init_preview_file(handle: AppHandle) {
    let mut preview_file = PreviewFile::new();
    preview_file.set_keyboard_hook();
    preview_file.app_handle = Some(handle);
    // 将实例存储在全局变量中
    unsafe {
        APP_INSTANCE = Some(preview_file);
    }
}

// 公开一个函数来清理钩子
pub fn cleanup_preview_file() {
    unsafe {
        if let Some(app) = APP_INSTANCE.as_mut() {
            app.remove_keyboard_hook();
        }
    }
}
