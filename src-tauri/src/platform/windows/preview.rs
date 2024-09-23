use std::{ffi::OsString, os::windows::ffi::OsStringExt, ptr::null_mut};
use windows::{
    core::{Interface, PWSTR},
    Win32::{
        Foundation::{HWND, LPARAM, LRESULT, WPARAM},
        System::Com::{
            CoCreateInstance, CoInitializeEx, CoTaskMemFree, CoUninitialize, IDispatch, CLSCTX_ALL,
            CLSCTX_INPROC_SERVER, COINIT_APARTMENTTHREADED,
        },
        UI::{
            Input::KeyboardAndMouse,
            Shell::{
                CLSID_ShellWindows,
                Common::{ITEMIDLIST, STRRET},
                IEnumIDList, IInitializeWithItem, IPreviewHandler, IShellBrowser, IShellFolder,
                IShellItem, IShellView, IShellWindows, SHCreateItemFromParsingName,
                SHGetDesktopFolder, SHCONTF_FOLDERS, SHCONTF_INCLUDEHIDDEN, SHCONTF_NONFOLDERS,
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
}

#[allow(dead_code)]
impl PreviewFile {
    pub fn new() -> Self {
        Self {
            selected_file: None,
            hook_handle: None,
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
                "selected_file is {:?} in PrviewFile Struct",
                &self.selected_file()
            );
            if let Some(ref file_path) = self.selected_file {
                println!("file path is {:?}", file_path);
                self.preview_file(file_path);
            }
        }
    }

    // 文件预览逻辑
    fn preview_file(&self, file_path: &str) {
        println!("Previewing file: {}", file_path);
        // 调用 IFilePreviewHandler 或其他预览逻辑
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

    fn get_selected_file() -> Option<String> {
        unsafe {
            let hwnd = WindowsAndMessaging::GetForegroundWindow(); // 获取当前活动窗口句柄
            println!("当前窗口的 HWND {:?}", hwnd);
            let mut class_name = [0u16; 256];
            WindowsAndMessaging::GetClassNameW(hwnd, &mut class_name);

            let class_name_str = String::from_utf16_lossy(&class_name);
            println!("className is {}", class_name_str);
            if class_name_str.contains("CabinetWClass") || class_name_str.contains("Progman") {
                // 窗口是文件管理器或桌面，开始获取选中的文件
                if let Some(shell_view) = Self::get_shell_view(hwnd) {
                    println!("shell_view is {:?}", shell_view);
                    // 获取当前选中的文件路径
                    let selected_file = Self::get_selected_file_from_shell_view(shell_view);
                    return selected_file;
                }
            }
        }
        None
    }

    fn get_shell_view(hwnd: HWND) -> Option<IShellView> {
        unsafe {
            // 初始化 COM 库
            let com = CoInitializeEx(None, COINIT_APARTMENTTHREADED);

            if com.is_err() {
                return None;
            }

            let hr = CoCreateInstance(&CLSID_ShellWindows, None, CLSCTX_ALL);

            if hr.is_err() {
                CoUninitialize(); // 清理 COM
                return None; // 创建 IShellWindows 失败
            }

            let shell_windows: IShellWindows = hr.unwrap();

            let count = shell_windows.Count().unwrap_or(0);
            let mut found_shell_view = None;

            for i in 0..count {
                let item = shell_windows.Item(i.into());
                if item.is_ok() {
                    let shell_browser: *mut IShellBrowser = null_mut();
                    if item.cast(&shell_browser).is_ok() {
                        let hwnd_browser = (*shell_browser).GetWindow();
                        if hwnd_browser == hwnd {
                            let mut shell_view: *mut IShellView = null_mut();
                            if (*shell_browser)
                                .QueryActiveShellView(&mut shell_view)
                                .is_ok()
                            {
                                found_shell_view = Some(shell_view); // 找到 IShellView
                                break; // 找到后可以退出循环
                            }
                        }
                    }
                    // 释放 IDispatch 接口
                    (*item).Release();
                }
            }

            // 释放 IShellWindows 接口
            (*shell_windows).Release();
            // 清理 COM
            CoUninitialize();

            found_shell_view // 返回找到的 IShellView 或 None
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
                strret_type_str => {
                    let p_str = &strret.Anonymous.cStr as *const _ as *const u16;
                    let wide_str = std::slice::from_raw_parts(p_str, strret.uType as usize / 2);
                    Some(OsString::from_wide(wide_str).to_string_lossy().into_owned())
                }
                _ => None,
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
    // pub fn preview_file(file_path: &str, hwnd: HWND) -> windows::core::Result<()> {
    //     println!("Previewing file: {}", file_path);

    //     unsafe {
    //         // 初始化 COM
    //         let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);

    //         // 将文件路径转为宽字符串（Windows API 要求）
    //         let path: PWSTR = PWSTR(file_path.encode_utf16().collect::<Vec<u16>>().as_mut_ptr());

    //         // 创建文件的 ShellItem
    //         // let mut shell_item: = None;
    //         let shell_item: IShellItem = SHCreateItemFromParsingName(path, None)?;

    //         // 获取文件的预览处理器
    //         // let mut preview_handler: Option<IPreviewHandler> = None;
    //         let preview_handler: Result<IPreviewHandler, windows::core::Error> =
    //             CoCreateInstance(&IPreviewHandler::IID, None, CLSCTX_INPROC_SERVER);

    //         if let Ok(handler) = preview_handler {
    //             // 初始化文件
    //             let initialize_with_file = handler.cast::<IInitializeWithItem>()?;
    //             let _ = initialize_with_file.Initialize(&shell_item, 0);
    //             // 将预览窗口绑定到我们的窗口句柄
    //             let _ = handler.SetWindow(hwnd, null_mut());

    //             // 触发文件预览
    //             handler.DoPreview()?;
    //         }

    //         Ok(())
    //     }
    // }
}

// 全局静态 APP 实例
static mut APP_INSTANCE: Option<PreviewFile> = None;

// 公开一个全局函数来初始化 PreviewFile
pub fn init_preview_file() {
    let mut preview_file = PreviewFile::new();
    preview_file.set_keyboard_hook();

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
