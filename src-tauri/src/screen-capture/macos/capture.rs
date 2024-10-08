use core_graphics::display::{CGDisplay, CGDisplayBounds};
use core_graphics::image::CGImage;
use image::{ImageBuffer, Rgba};
use std::fs::File;
use std::io::BufWriter;

pub struct Capture {
    data: Vec<u8>,
    width: u32,
    height: u32,
}

impl Capture {
    // 创建新的 Capture 实例并捕获屏幕
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let display_id = CGDisplay::main().id();
        let bounds = CGDisplayBounds(display_id);
        let width = bounds.size.width as u32;
        let height = bounds.size.height as u32;

        // 捕获屏幕
        let image: CGImage = CGDisplay::create_image(display_id).ok_or("Failed to create image")?;
        let data = image.data().to_vec();

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
