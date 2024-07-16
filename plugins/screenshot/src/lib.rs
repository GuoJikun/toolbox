use image::{ImageBuffer, Rgba};
use std::ffi::CString;
use std::io::ErrorKind::WouldBlock;
use std::os::raw::c_char;
use std::thread;
use std::time::Duration;

#[cfg(target_os = "windows")]
use scrap::{Capturer, Display};

#[cfg(target_os = "linux")]
use scrap::{Capturer as LinuxCapturer, Display as LinuxDisplay};

#[cfg(target_os = "macos")]
use coregraphics::display::CGDisplay;

#[no_mangle]
pub extern "C" fn capture_screenshot() -> *const c_char {
    cfg_if::cfg_if! {
        if #[cfg(target_os = "windows")] {
            match capture_with_scrap_windows() {
                Ok(path) => string_to_c_char(path),
                Err(err) => string_to_c_char(format!("Error: {}", err)),
            }
        } else if #[cfg(target_os = "linux")] {
            match capture_with_scrap_linux() {
                Ok(path) => string_to_c_char(path),
                Err(err) => string_to_c_char(format!("Error: {}", err)),
            }
        } else if #[cfg(target_os = "macos")] {
            match capture_with_coregraphics() {
                Ok(path) => string_to_c_char(path),
                Err(err) => string_to_c_char(format!("Error: {}", err)),
            }
        } else {
            string_to_c_char("Unsupported platform".into())
        }
    }
}

#[cfg(target_os = "windows")]
fn capture_with_scrap_windows() -> Result<String, String> {
    let display = Display::primary().map_err(|e| e.to_string())?;
    let (w, h) = (display.width(), display.height());
    let mut capturer = Capturer::new(display).map_err(|e| e.to_string())?;

    loop {
        match capturer.frame() {
            Ok(frame) => {
                let buffer = frame.iter().cloned().collect::<Vec<u8>>();
                let img = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(w as u32, h as u32, buffer)
                    .ok_or("Failed to create image buffer.")?;
                img.save("screenshot.png").map_err(|e| e.to_string())?;
                return Ok("screenshot.png".into());
            }
            Err(error) => {
                if error.kind() == WouldBlock {
                    thread::sleep(Duration::from_millis(10));
                    continue;
                } else {
                    return Err(error.to_string());
                }
            }
        }
    }
}

#[cfg(target_os = "linux")]
fn capture_with_scrap_linux() -> Result<String, String> {
    let display = LinuxDisplay::primary().map_err(|e| e.to_string())?;
    let (w, h) = (display.width(), display.height());
    let mut capturer = LinuxCapturer::new(display).map_err(|e| e.to_string())?;

    loop {
        match capturer.frame() {
            Ok(frame) => {
                let buffer = frame.iter().cloned().collect::<Vec<u8>>();
                let img = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(w as u32, h as u32, buffer)
                    .ok_or("Failed to create image buffer.")?;
                img.save("screenshot.png").map_err(|e| e.to_string())?;
                return Ok("screenshot.png".into());
            }
            Err(error) => {
                if error.kind() == WouldBlock {
                    thread::sleep(Duration::from_millis(10));
                    continue;
                } else {
                    return Err(error.to_string());
                }
            }
        }
    }
}

#[cfg(target_os = "macos")]
fn capture_with_coregraphics() -> Result<String, String> {
    let display_id = CGDisplay::main().id();
    let image = CGDisplay::main()
        .image()
        .ok_or("Failed to capture screenshot")?;
    let width = image.width() as u32;
    let height = image.height() as u32;

    let mut buffer: Vec<u8> = vec![0; (width * height * 4) as usize];
    image.data().copy_into_slice(&mut buffer);

    let img = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(width, height, buffer)
        .ok_or("Failed to create image buffer.")?;
    img.save("screenshot.png").map_err(|e| e.to_string())?;
    Ok("screenshot.png".into())
}

fn string_to_c_char(s: String) -> *const c_char {
    CString::new(s).unwrap().into_raw()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
