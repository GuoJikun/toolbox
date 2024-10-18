use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use tauri::{command, path::BaseDirectory, AppHandle, Manager};

use memmap2::MmapMut;
use std::fs::OpenOptions;
use serde_json::json;
use tauri_plugin_store::JsonValue;

#[path = "platform/mod.rs"]
mod platform;
use platform::{App, Installed, Screenshot};

#[path = "utils/mod.rs"]
mod utils;

// 获取本机安装的 app 列表
#[command]
pub fn get_installed_apps() -> Vec<App> {
    let result = Installed::new();
    let apps = result.apps;
    return apps;
}

// 获取屏幕截图
#[command]
pub fn screenshot_desktop(app: AppHandle) -> Result<String, String> {
    utils::print_current_time();
    let tmp = Screenshot::new().map_err(|e| e.to_string());
    let result = tmp.unwrap();

    let file_path = app
        .path()
        .resolve("image_data.bin", BaseDirectory::Temp)
        .unwrap();

    // 创建一个临时文件用于存储共享内存
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path.clone())
        .map_err(|e| e.to_string())?;

    // 图片数据
    let image_data: Vec<u8> = result.to_vec();
    file.set_len(image_data.len() as u64)
        .map_err(|e| e.to_string())?;

    // 创建一个内存映射，并将数据写入其中
    let mut mmap = unsafe { MmapMut::map_mut(&file).map_err(|e| e.to_string())? };
    mmap.copy_from_slice(&image_data);
    mmap.flush().map_err(|e| e.to_string())?;
    utils::print_current_time();
    // 返回内存映射文件的路径，让前端能够访问
    Ok(file_path.to_string_lossy().to_string())
}
// 执行外部程序
#[command]
pub fn run_external_program(executable_path: String, args: Vec<String>) -> Result<String, String> {
    let result = Arc::new(Mutex::new(String::new()));
    let status = Arc::new(Mutex::new("success".to_string()));

    let result_clone = Arc::clone(&result);
    let status_clone = Arc::clone(&status);

    let handle = thread::spawn(move || {
        let child = Command::new(executable_path)
            .args(&args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start process");

        let output = child.wait_with_output().expect("Failed to wait on child");

        if output.status.success() {
            println!("Output: {}", String::from_utf8_lossy(&output.stdout));
            let mut result = result_clone.lock().unwrap();
            *result = String::from_utf8_lossy(&output.stdout).to_string();
            let mut status = status_clone.lock().unwrap();
            *status = "success".to_string();
        } else {
            eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
            let mut result = result_clone.lock().unwrap();
            *result = String::from_utf8_lossy(&output.stderr).to_string();
            let mut status = status_clone.lock().unwrap();
            *status = "error".to_string();
        }
    });

    handle.join().expect("Thread panicked");

    let status = status.lock().unwrap();
    let result = result.lock().unwrap();

    if *status == "success" {
        Ok(result.clone())
    } else {
        Err(result.clone())
    }
}

#[command]
pub fn preview_file(path: String) -> Result<utils::preview::File, String> {
    utils::preview::preview_file(path)
    // let result = utils::preview::preview_file(path);
    // if result.is_err() {
    //     return Err(result.err().unwrap());
    // }
    // let file = result.unwrap();
    // return Ok(json!({
    //     "matcher_type": file.matcher_type(),
    //     "path": file.path(),
    //     "extension": file.extension()
    // }));
}
