use std::convert::Into;
use std::path::PathBuf;
use image::ImageReader;
use webp::Encoder;
use std::fs::File;
use std::io::{BufWriter, Write};
use serde::Serialize;
use tauri::{AppHandle, path::BaseDirectory, Manager, Emitter, EventTarget};

#[derive(Clone, Serialize)]
pub enum ConvertImagesPayloadStatus {
    Success,
    Error,
}

#[derive(Clone, Serialize)]
pub struct ConvertImagesPayload {
    status: ConvertImagesPayloadStatus,
    message: String,
    path: String,
    dest: String,
}
#[tauri::command]
pub fn convert_images(app: AppHandle,paths: Vec<String>) -> Result<(), String> {
    let label = "toolbox-plugin-convertToWebp".to_string();
    let event_name = "plugin_convert_images_notify";
    for path_str in paths {
        let input_path = PathBuf::from(&path_str);
        let _file_name = input_path.file_name().unwrap().to_string_lossy().to_string();
        // 安全性检查：验证文件是否存在且可读
        if !input_path.is_file() {
            app.emit_to(EventTarget::WebviewWindow { label: label.clone()}, event_name, ConvertImagesPayload {
                status: ConvertImagesPayloadStatus::Error,
                path: path_str,
                message: "文件不存在或无法读取".into(),
                dest: "".into()
            }).unwrap();
            continue
        }

        // 防止目录遍历攻击，确保路径安全
        let canonical_input = input_path.canonicalize();
        if canonical_input.is_err() {
            app.emit_to(EventTarget::WebviewWindow { label: label.clone()}, event_name, ConvertImagesPayload {
                status: ConvertImagesPayloadStatus::Error,
                path: path_str,
                message: "文件路径无效".into(),
                dest: "".into()
            }).unwrap();
            continue
        }

        let uuid = uuid::Uuid::new_v4();
        let output_path = app.path().resolve(format!("{}.webp", uuid.to_string()), BaseDirectory::Temp);
        if output_path.is_err() {
            app.emit_to(EventTarget::WebviewWindow { label: label.clone()}, event_name, ConvertImagesPayload {
                status: ConvertImagesPayloadStatus::Error,
                path: path_str,
                message: "无法解析暂存文件路径".into(),
                dest: "".into()
            }).unwrap();
            continue
        }
        let mut output_path = output_path.unwrap();
        // 执行转换
        let result = convert_to_webp(&canonical_input.unwrap(), &mut output_path);
        if result.is_err() {
            app.emit_to(EventTarget::WebviewWindow { label: label.clone()}, event_name, ConvertImagesPayload {
                status: ConvertImagesPayloadStatus::Error,
                path: path_str,
                message: result.err().unwrap().to_string(),
                dest: "".into()
            }).unwrap();
            continue
        }

        app.emit_to(EventTarget::WebviewWindow { label: label.clone()}, event_name, ConvertImagesPayload {
            status: ConvertImagesPayloadStatus::Success,
            path: path_str,
            message: "格式转化成功".into(),
            dest: output_path.to_string_lossy().into(),
        }).unwrap();
    }
    Ok(())
}

fn convert_to_webp(input_path: &PathBuf, output_path: &mut PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    // 读取输入图像
    let img = ImageReader::open(&input_path)?.decode()?;

    // 将图像转换为 RGBA8 格式
    let rgba = img.to_rgba8();

    // 使用 webp crate 进行编码
    let encoder = Encoder::from_rgba(&rgba, img.width(), img.height());
    let webp_data = encoder.encode_lossless();

    // 设置为 .webp 扩展名
    output_path.set_extension("webp");

    // 构建输出文件并写入数据
    let output_file = File::create(&output_path)?;
    let mut writer = BufWriter::new(output_file);
    writer.write_all(&webp_data)?;

    Ok(())
}

