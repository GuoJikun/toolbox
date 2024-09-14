use std::ptr::null_mut;
use windows::{
    core::{Interface, PWSTR},
    Win32::Foundation::HWND,
    Win32::System::Com::{
        CoCreateInstance, CoInitializeEx, CLSCTX_INPROC_SERVER, COINIT_APARTMENTTHREADED,
    },
    Win32::UI::Shell::{
        IInitializeWithItem, IPreviewHandler, IShellItem, SHCreateItemFromParsingName,
    },
};

// 初始化 COM 和窗口
#[allow(dead_code)]
pub fn preview_file(file_path: &str, hwnd: HWND) -> windows::core::Result<()> {
    unsafe {
        // 初始化 COM
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);

        // 将文件路径转为宽字符串（Windows API 要求）
        let path: PWSTR = PWSTR(file_path.encode_utf16().collect::<Vec<u16>>().as_mut_ptr());

        // 创建文件的 ShellItem
        // let mut shell_item: = None;
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
