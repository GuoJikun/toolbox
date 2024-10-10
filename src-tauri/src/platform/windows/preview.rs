use std::{ffi::OsString, os::windows::ffi::OsStringExt, ptr::null_mut};
use tauri::{AppHandle, Manager};
use windows::{
    core::{IUnknown, Interface, GUID, PCWSTR, PWSTR, VARIANT},
    Win32::{
        Foundation::{BOOL, HWND, LPARAM, LRESULT, WPARAM,S_OK},
        System::{
            Com::{
                CoCreateInstance, CoInitializeEx,CoInitialize, CoTaskMemFree, CoUninitialize, IDispatch,
                IServiceProvider, CLSCTX_ALL, CLSCTX_INPROC_SERVER,
                COINIT_APARTMENTTHREADED, COINIT_DISABLE_OLE1DDE,
            },
            Memory,
        },
        UI::{
            Input::KeyboardAndMouse,
            Shell::{
                Common::{ITEMIDLIST, STRRET},
                IEnumIDList, IInitializeWithItem, IPreviewHandler, IShellBrowser,
                IShellFolder, IShellItem, IShellView, IShellWindows,
                IWebBrowser2, SHCreateItemFromParsingName, SHGetDesktopFolder, ShellWindows,
                SHGDN_FORPARSING, SVGIO_SELECTION, SWC_DESKTOP, SWC_EXPLORER, SWFO_NEEDDISPATCH,
                IUnknown_QueryService, SID_STopLevelBrowser

            },
            WindowsAndMessaging,
        },
    },
};
use windows::Win32::UI::Shell::IFolderView;

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


    fn get_lpsz_class(str: &str) -> PCWSTR {
        return PCWSTR::from_raw(str.encode_utf16().collect::<Vec<u16>>().as_ptr());
    }
    fn get_selected_file_path(hwnd: HWND) -> Option<String> {
        unsafe {
            // 准备发送 WM_COPYDATA 消息
            let mut buffer = [0u16; 256];
            let mut cds = windows::Win32::System::DataExchange::COPYDATASTRUCT {
                dwData: 0,
                cbData: buffer.len() as u32,
                lpData: buffer.as_mut_ptr() as *mut _,
            };

            // 发送消息
            let result = WindowsAndMessaging::SendMessageW(hwnd, WindowsAndMessaging::WM_COPYDATA, WPARAM(0x00000060), LPARAM(&mut cds as *const _ as isize));
            println!("Received path: {}", result.0);
            // 处理结果
            if result.0 != 0 {
                // 解析返回的数据
                // 这里需要根据实际情况解析路径
                // 假设获取的路径在 lpData 指向的内存中

                // let path_ptr = Memory::GlobalLock(cds.lpData);
                // let path = windows::core::HSTRING::from_wide(path_ptr);
                // let file_path: String = path.unwrap().to_string_lossy().into();
                return Some("file_path".into());

            }
        }
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
                // Self::get_selected_file_path(hwnd);
                // Explorer::get_select_file_path(hwnd);
                // return Self::get_selected_file_path(hwnd);
                let path = Self::get_explorer_show_path();
                println!("explorer_show_path is {:?}", path);
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
    fn get_explorer_show_path() -> String {
        // debug!("enter get_explorer_show_path");
        let mut folder_cur_path = "none".to_string();
        unsafe {
            let foreground_window = WindowsAndMessaging::GetForegroundWindow();
            let mut class_name = [0; 260];
            WindowsAndMessaging::GetClassNameW(foreground_window, &mut class_name);
            let pos = class_name.iter().position(|c| *c == 0).unwrap();
            let class_name = String::from_utf16_lossy(&class_name[0..pos]);
            if class_name != "CabinetWClass" {
                // debug!("not explorer window：{}", class_name);
                return folder_cur_path;
            }
            // debug!("find explorer windows");
            let _ = CoInitializeEx(None, COINIT_DISABLE_OLE1DDE);
            let psh_windows = CoCreateInstance(&ShellWindows, None, windows::Win32::System::Com::CLSCTX_LOCAL_SERVER);
            if psh_windows.is_err() {
                CoUninitialize();
                return folder_cur_path;
            }
            let psh_windows: IShellWindows = psh_windows.unwrap();
            let count = psh_windows.Count();
            if count.is_err() {
                CoUninitialize();
                return folder_cur_path;
            }
            let count: i32 = count.unwrap();
            for i in 0..count {
                let i = VARIANT::from(i);

                let disp = psh_windows.Item(&i);
                if disp.is_err() {
                    // error!("{}", disp.unwrap_err());
                    continue;
                }
                let disp = disp.unwrap();
                let mut p_app = std::ptr::null_mut();
                let ret = disp.query(&IWebBrowser2::IID, &mut p_app);
                if ret != windows::Win32::Foundation::S_OK {
                    continue;
                }
                let p_app = IWebBrowser2::from_raw(p_app);
                let win_hwnd = p_app.HWND();
                if win_hwnd.is_err() {
                    continue;
                }
                let win_hwnd = win_hwnd.unwrap();
                if win_hwnd.0 != foreground_window.0 as isize {
                    continue;
                }
                let mut psp = std::ptr::null_mut();
                let ret = p_app.query(&IServiceProvider::IID, &mut psp);
                if ret != windows::Win32::Foundation::S_OK {
                    continue;
                }
                let psp = IServiceProvider::from_raw(psp);
                let browser = psp.QueryService(&IShellBrowser::IID);
                if browser.is_err() {
                    continue;
                }
                let browser: IShellBrowser = browser.unwrap();
                let shell_view = browser.QueryActiveShellView();
                if shell_view.is_err() {
                    continue;
                }
                let shell_view = shell_view.unwrap();
                let mut p_folder_view = std::ptr::null_mut();
                let ret = shell_view.query(&windows::Win32::UI::Shell::IFolderView::IID, &mut p_folder_view);
                if ret != windows::Win32::Foundation::S_OK {
                    continue;
                }
                let p_folder_view = windows::Win32::UI::Shell::IFolderView::from_raw(p_folder_view);
                let folder = p_folder_view.GetFolder();
                if folder.is_err() {
                    continue;
                }
                let folder: windows::Win32::UI::Shell::IPersistFolder2 = folder.unwrap();
                let pidl = folder.GetCurFolder();
                if pidl.is_err() {
                    continue;
                }
                let pidl = pidl.unwrap();
                let path = windows::Win32::UI::Shell::SHGetNameFromIDList(pidl, windows::Win32::UI::Shell::SIGDN_FILESYSPATH);
                if path.is_err() {
                    continue;
                }
                let path = path.unwrap();
                let path = path.as_wide();
                folder_cur_path = String::from_utf16_lossy(path);
                CoTaskMemFree(Some(path.as_ptr() as *const std::ffi::c_void));
                break;
            }
            CoUninitialize();
        };
        return folder_cur_path;
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

            let mut tmp = HWND::default();
              let dispatch = shell_windows.FindWindowSW(
                &VARIANT::default(),
                &VARIANT::default(),
                SWC_DESKTOP,
                std::ptr::addr_of_mut!(tmp) as _,
                SWFO_NEEDDISPATCH,
            );

            if dispatch.is_err() {
                CoUninitialize(); // 清理 COM
                return None;
            }

            let shell_browser: Result<IShellBrowser, _> = IUnknown_QueryService(&dispatch.unwrap(), &SID_STopLevelBrowser);
            if shell_browser.is_err() {
                CoUninitialize(); // 清理 COM
                return None;
            }
            let shell_view = shell_browser.unwrap().QueryActiveShellView();
            if shell_view.is_ok() {
                let shell_view = shell_view.unwrap();
                // let shell_view_window = shell_view.GetWindow();
                // println!("shell_view_window is {:?}", shell_view_window.clone().unwrap());
                // if shell_view_window.is_ok() && shell_view_window.unwrap() == hwnd {
                //     println!("shell_view is {:?}", &shell_view);
                //     found_shell_view = Some(shell_view);
                // }
                println!("shell_view is {:?}", &shell_view);
                found_shell_view = Some(shell_view);
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
            let _ = CoInitializeEx(None, COINIT_DISABLE_OLE1DDE);
            let p_folder_view = std::ptr::null_mut();
            let ret = shell_view.query(&IFolderView::IID, p_folder_view);

            if ret != S_OK {
                CoUninitialize();
                return None;
            }

            println!("p_folder_view is {:?}", p_folder_view);

            // let p_folder_view = IFolderView::from_raw(p_folder_view);
            // let folder = p_folder_view.GetFolder();
            // if folder.is_err() {
            //     continue;
            // }
            // let folder: IPersistFolder2 = folder.unwrap();
            // let pidl = folder.GetCurFolder();
            // if pidl.is_err() {
            //     continue;
            // }
            // let pidl = pidl.unwrap();
            // let path = SHGetNameFromIDList(pidl, SIGDN_FILESYSPATH);
            // if path.is_err() {
            //     continue;
            // }
            // let path = path.unwrap();
            // let path = path.as_wide();
            // folder_cur_path = String::from_utf16_lossy(path);

            let item  = shell_view.GetItemObject::<IShellItem>(SVGIO_SELECTION);

            if item.is_err() {
                CoUninitialize();
                return None;
            }

            let shell_folder = SHGetDesktopFolder();
            if shell_folder.is_err() {
                CoUninitialize();
                return None;
            }

            let shell_folder = shell_folder.unwrap();

            // 获取路径
            let display_name_ptr  = item.unwrap().GetDisplayName(windows::Win32::UI::Shell::SIGDN_FILESYSPATH);
            if display_name_ptr.is_err() {
                println!("Error: {:?}", display_name_ptr.err());
                CoUninitialize();
                return None;
            }
            let display_name_ptr  = display_name_ptr .unwrap();
            // 将路径转换为字符串并打印
            let path_str = String::from_utf16_lossy(std::slice::from_raw_parts(display_name_ptr.0, (0..).take_while(|&i| *display_name_ptr.0.offset(i) != 0).count()));
            println!("选中的文件路径: {}", path_str.trim_matches(char::from(0)));
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
