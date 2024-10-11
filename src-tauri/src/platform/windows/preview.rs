use std::{ffi::OsString, os::windows::ffi::OsStringExt, ptr::null_mut};
use tauri::{AppHandle, Manager};
use windows::Win32::UI::Controls;
use windows::Win32::UI::Controls::LVITEMW;
use windows::Win32::UI::Shell::IFolderView;
use windows::{
    core::{IUnknown, Interface, GUID, PCWSTR, PWSTR, VARIANT},
    Win32::{
        Foundation::{BOOL, HWND, LPARAM, LRESULT, S_OK, WPARAM},
        System::{
            Com::{
                CoCreateInstance, CoInitializeEx, CoTaskMemFree, CoUninitialize, IDispatch,
                IServiceProvider, CLSCTX_ALL, CLSCTX_INPROC_SERVER, CLSCTX_LOCAL_SERVER,
                COINIT_APARTMENTTHREADED, COINIT_DISABLE_OLE1DDE,
            },
            Memory,
        },
        UI::{
            Input::KeyboardAndMouse,
            Shell::{
                Common::{ITEMIDLIST, STRRET},
                IEnumIDList, IInitializeWithItem, IPreviewHandler, IShellBrowser, IShellFolder,
                IShellFolderViewDual, IShellItem, IShellView, IShellWindows, IUnknown_QueryService,
                IWebBrowser2, SHCreateItemFromParsingName, SHGetDesktopFolder,
                SHGetSpecialFolderLocation, SID_STopLevelBrowser, ShellWindows, SHGDN_FORPARSING,
                SVGIO_SELECTION, SWC_DESKTOP, SWC_EXPLORER, SWFO_NEEDDISPATCH,
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
            let result = WindowsAndMessaging::SendMessageW(
                hwnd,
                WindowsAndMessaging::WM_COPYDATA,
                WPARAM(0x00000060),
                LPARAM(&mut cds as *const _ as isize),
            );
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
            let class_name_str = Self::get_window_name(hwnd);
            println!("className is {}", class_name_str);
            if class_name_str.contains("CabinetWClass") || class_name_str.contains("Progman") {
                // 窗口是文件管理器或桌面，开始获取选中的文件
                // Self::get_selected_file_path(hwnd);
                // Explorer::get_select_file_path(hwnd);
                // return Self::get_selected_file_path(hwnd);
                // 获取选中的文件路径

                let path = Self::get_select_file_path(hwnd);
                println!("路径 is {:?}", path);
            }
        };
        None
    }
    fn get_window_name(hwnd: HWND) -> String {
        let mut buffer = [0u16; 256];
        unsafe {
            WindowsAndMessaging::GetClassNameW(hwnd, &mut buffer);
        }
        String::from_utf16_lossy(&buffer)
    }

    // 检查 IDispatch 对象是否包含特定接口
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

    fn get_select_file_path(hwnd: HWND) -> Option<String> {
        unsafe {
            // 初始化 COM 库
            let com = CoInitializeEx(None, COINIT_DISABLE_OLE1DDE);
            if com.is_err() {
                return None;
            }

            let hr: Result<IShellWindows, windows::core::Error> =
                CoCreateInstance(&ShellWindows, None, CLSCTX_LOCAL_SERVER);
            // let hr = CoCreateInstance(&ShellWindows, None, CLSCTX_INPROC_HANDLER);
            if hr.is_err() {
                println!("创建 IShellWindows 失败");
                CoUninitialize(); // 清理 COM
                return None; // 创建 IShellWindows 失败
            }
            let shell_windows = hr.unwrap();

            let mut target_path = None;
            let count = shell_windows.Count().unwrap_or_default();

            for i in 0..count {
                let variant = VARIANT::from(i);
                let window: IDispatch = shell_windows.Item(&variant).ok()?;
                let web_browser: IWebBrowser2 = window.cast().ok()?;
                // 通过IWebBrowser2获取文件夹视图并获取选中的项目
                let document = web_browser.Document().ok()?;
                let folder_view: IShellFolderViewDual = document.cast().ok()?;
                let target_hwnd = web_browser.HWND().ok()?;
                println!("target_hwnd:{:?}", target_hwnd);

                let selected_items = folder_view.SelectedItems().ok()?;
                let count = selected_items.Count().ok()?;
                if count > 0 {
                    let item = selected_items.Item(&VARIANT::from(0)).ok()?;
                    let path = item.Path().ok()?;
                    target_path = Some(path.to_string());
                }
            }
            // 清理 COM
            CoUninitialize();
            // 返回找到的 IShellView 或 None
            target_path
        }
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
