use image::{ImageBuffer, Rgba};
use std::fs::File;
use std::io::BufWriter;
#[cfg(feature = "wayland")]
use wayland_client::{protocol::wl_output::WlOutput, Display, GlobalManager};
#[cfg(feature = "wayland")]
use wayland_screenshot::{Screenshot, ScreenshotHandler};
#[cfg(feature = "x11")]
use x11::xlib::*;

pub struct Capture {
    data: Vec<u8>,
    width: u32,
    height: u32,
}

impl Capture {
    // 创建新的 Capture 实例并捕获屏幕
    #[cfg(feature = "x11")]
    pub fn new_x11() -> Result<Self, Box<dyn std::error::Error>> {
        let display = XOpenDisplay(std::ptr::null());
        let screen = DefaultScreen(display);
        let width = XDisplayWidth(display, screen);
        let height = XDisplayHeight(display, screen);

        let root = RootWindow(display, screen);
        let mut image = XGetImage(display, root, 0, 0, width, height, AllPlanes, ZPixmap);

        let data = unsafe {
            std::slice::from_raw_parts(image.data as *const u8, (width * height * 4) as usize)
                .to_vec()
        };

        XDestroyImage(image);
        XCloseDisplay(display);

        Ok(Capture {
            data,
            width: width as u32,
            height: height as u32,
        })
    }

    #[cfg(feature = "wayland")]
    pub fn new_wayland() -> Result<Self, Box<dyn std::error::Error>> {
        let display = Display::connect_to_env()?;
        let mut global_manager = GlobalManager::new(&display);
        global_manager.sync()?;

        // 使用 wayland-screenshot crate 捕获屏幕
        let screenshot = Screenshot::new(&display)?;
        let (width, height, data) = screenshot.take_screenshot()?;

        Ok(Capture {
            data,
            width,
            height,
        })
    }

    // 保存为 PNG 文件
    pub fn save(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let img_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> =
            ImageBuffer::from_raw(self.width, self.height, self.data.clone())
                .ok_or("Failed to create image buffer")?;

        let file = File::create(file_path)?;
        let ref mut w = BufWriter::new(file);
        img_buffer.save(w)?;

        Ok(())
    }

    // 获取帧数据
    pub fn frame(&self) -> &[u8] {
        &self.data
    }
}
