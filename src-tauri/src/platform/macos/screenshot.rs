use image::{ImageBuffer, Rgba};
use std::{io::ErrorKind::WouldBlock, thread, time::Duration};

#[derive(Debug)]
pub struct Screenshot {
    data: ImageBuffer<Rgba<u8>, Vec<u8>>,
}

use scrap::{Capturer, Display};

impl Screenshot {
    pub fn new() -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, String> {
        let display = Display::primary().map_err(|e| e.to_string())?;
        let (w, h) = (display.width(), display.height());
        let mut capturer = Capturer::new(display).map_err(|e| e.to_string())?;

        loop {
            match capturer.frame() {
                Ok(frame) => {
                    // 将 BGR 转换为 RGB
                    let buffer: Vec<u8> = frame
                        .chunks(4)
                        .flat_map(|pixel| vec![pixel[2], pixel[1], pixel[0], pixel[3]])
                        .collect();
                    let img =
                        ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(w as u32, h as u32, buffer)
                            .ok_or("Failed to create image buffer.")?;
                    Self { data: img.clone() };
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

    pub fn save(&self, path: &str) -> Result<(), String> {
        self.data.save(path).map_err(|e| e.to_string())
    }
}
