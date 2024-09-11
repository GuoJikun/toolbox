use windows::{
    core::*,
    Win32::{Foundation::*, Graphics::Gdi::*, System::Library::*},
};

use image::{ImageBuffer, Rgba};
use std::fs::File;
use std::io::BufWriter;
use std::ptr;

pub struct Capture {
    data: Vec<u8>,
    width: u32,
    height: u32,
}

impl Capture {
    // 创建新的 Capture 实例并捕获屏幕
    pub fn new() -> Result<Self, HRESULT> {
        // 创建设备上下文
        let hdc_screen = GetDC(HWND(0));
        let hdc_mem = CreateCompatibleDC(hdc_screen);

        // 获取屏幕尺寸
        let width = GetDeviceCaps(hdc_screen, HORZRES);
        let height = GetDeviceCaps(hdc_screen, VERTRES);

        // 创建位图
        let hbitmap = CreateCompatibleBitmap(hdc_screen, width, height);
        SelectObject(hdc_mem, hbitmap);

        // 捕获屏幕
        BitBlt(hdc_mem, 0, 0, width, height, hdc_screen, 0, 0, SRCCOPY);

        // 创建 BGRA 数据缓冲区
        let mut bmp_info = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: width,
                biHeight: -height, // 负值表示从上到下填充
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB,
                biSizeImage: 0,
                biXPelsPerMeter: 0,
                biYPelsPerMeter: 0,
                biClrUsed: 0,
                biClrImportant: 0,
            },
            bmiColors: [0; 1],
        };

        let mut data: Vec<u8> = vec![0; (width * height * 4) as usize]; // BGRA 每个像素 4 字节
        GetDIBits(
            hdc_mem,
            hbitmap,
            0,
            height as u32,
            data.as_mut_ptr() as *mut _,
            &mut bmp_info,
            DIB_RGB_COLORS,
        );

        // 清理资源
        DeleteObject(hbitmap);
        DeleteDC(hdc_mem);
        ReleaseDC(HWND(0), hdc_screen);

        Ok(Capture {
            data,
            width: width as u32,
            height: height as u32,
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
