use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::GetClassNameW;

pub fn get_window_class_name(hwnd: HWND) -> String {
    let mut buffer = [0u16; 256];
    let len = unsafe {
        GetClassNameW(hwnd, &mut buffer)
    };
    String::from_utf16_lossy(&buffer[..len as usize])
}