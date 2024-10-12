use std::io::Error;
use tauri::{AppHandle, Emitter, Error as TauriError, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, Code, ShortcutState};
use windows::{
    core::{Interface, GUID, VARIANT},
    Win32::{
        Foundation::{ HWND, LPARAM, LRESULT, WPARAM },
        System::{
            Com::{
                CoCreateInstance, CoInitializeEx, CoUninitialize, IDispatch,
                 CLSCTX_LOCAL_SERVER,
                COINIT_DISABLE_OLE1DDE,
            },
        },
        UI::{
            Shell::{
                IShellFolderViewDual, IShellWindows,
                IWebBrowser2, ShellWindows,
            },
            WindowsAndMessaging,
            Input::KeyboardAndMouse
        },
    },
};

#[derive(Debug)]
pub struct PreviewFile {
    selected_file: Option<String>,                   // 存储选中的文件路径
    hook_handle: Option<WindowsAndMessaging::HHOOK>, // 钩子的句柄
    app_handle: Option<AppHandle>,
}

struct Explorer;

impl Explorer {
    pub fn new () -> Option<String> {
        Self::get_selected_file()
    }

    fn get_selected_file() -> Option<String> {
        let mut path = None;
        unsafe {
            let hwnd = WindowsAndMessaging::GetForegroundWindow(); // 获取当前活动窗口句柄
            let class_name_str = Self::get_window_name(hwnd);
            if class_name_str.contains("CabinetWClass") || class_name_str.contains("Progman") {
                // 窗口是文件管理器或桌面，开始获取选中的文件
                path = Self::get_select_file_path(hwnd);
            }
        };
        path
    }
    fn get_window_name(hwnd: HWND) -> String {
        let mut buffer = [0u16; 256];
        unsafe {
            WindowsAndMessaging::GetClassNameW(hwnd, &mut buffer);
        }
        String::from_utf16_lossy(&buffer)
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
                // 检查窗口是否与当前活动窗口匹配
                let item_hwnd = web_browser.HWND().ok()?;
                if item_hwnd.0 != hwnd.0 as isize {
                    continue;
                }
                // 通过IWebBrowser2获取文件夹视图并获取选中的项目
                let document = web_browser.Document().ok()?;
                let folder_view: IShellFolderViewDual = document.cast().ok()?;


                let selected_items = folder_view.SelectedItems().ok()?;
                let count = selected_items.Count().ok()?;
                // for i in 0..count {
                //     let variant = VARIANT::from(i);
                //     let item: FolderItem = selected_items.Item(&variant).ok()?;
                //     let path = item.Path().ok()?;
                //     println!("Selected item path: {}", path.to_string());
                // }
                if count > 0 {
                    let item = selected_items.Item(&VARIANT::from(0)).ok()?;
                    let path = item.Path().ok()?;
                    target_path = Some(path.to_string());
                    break
                }
            }
            // 清理 COM
            CoUninitialize();
            target_path
        }
    }
}


#[allow(dead_code)]
impl PreviewFile {
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
            let _ = Self::preview_file(self.app_handle.clone().unwrap());
        }
    }

    // 全局键盘钩子的回调函数
    extern "system" fn keyboard_proc(ncode: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        if ncode == 0 && wparam.0 == WindowsAndMessaging::WM_KEYDOWN as usize {
            let kb_struct = unsafe { *(lparam.0 as *const WindowsAndMessaging::KBDLLHOOKSTRUCT) };
            let vk_code = kb_struct.vkCode;
            let is_explorer_or_desktop=unsafe{
                let hwnd = WindowsAndMessaging::GetForegroundWindow(); // 获取当前活动窗口句柄
                let class_name_str = Explorer::get_window_name(hwnd);
                class_name_str.contains("CabinetWClass") || class_name_str.contains("Progman")
            };
            if is_explorer_or_desktop{
                // 获取 PreviewFile 实例并处理按键事件
                if let Some(app) = unsafe { APP_INSTANCE.as_ref() } {
                    app.handle_key_down(vk_code);
                }
            }
            
        }
        unsafe { WindowsAndMessaging::CallNextHookEx(None, ncode, wparam, lparam) }
    }



    pub fn preview_file(app: AppHandle) -> Result<(), TauriError> {
        let file_path = Explorer::new();
        if file_path.is_none() {
            println!("No file selected")
        }
        let window = app
            .get_webview_window("preview")
            .unwrap();
        window.show()?;
        window.set_focus()?;
        window.emit("file-preview", file_path)?;
        Ok(())
    }

    pub fn new() -> Self {
        Self {
            selected_file: None,
            hook_handle: None,
            app_handle: None,
        }
    }

}

static mut APP_INSTANCE: Option<PreviewFile> = None;
impl Drop for PreviewFile {
    fn drop(&mut self) {
        println!("Dropping PreviewFile instance");
        self.remove_keyboard_hook();
    }
}

impl Default for PreviewFile {
    fn default() -> Self {
        PreviewFile::new()
    }
}

// 公开一个全局函数来初始化 PreviewFile
pub fn init_preview_file(handle: AppHandle) {
    let mut preview_file = PreviewFile::default();
    preview_file.set_keyboard_hook();
    preview_file.app_handle = Some(handle);
    // 将实例存储在全局变量中
    unsafe {
        APP_INSTANCE = Some(preview_file);
    }
}
