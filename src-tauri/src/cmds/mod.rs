use memmap2::MmapMut;
use tauri::State;
use tauri::{command as TCMD, path::BaseDirectory, AppHandle, Manager};
use std::process::{Command, Stdio};
use std::fs::OpenOptions;

pub mod pkg;
pub mod dylib;
pub mod scripts;

// 执行外部程序
#[TCMD]
pub fn run_external_program(executable_path: String, args: Vec<String>) -> Result<String, String> {
    if let Ok(_child) = Command::new(executable_path)
        .args(&args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(String::from("命令执行成功"))
    } else {
        return Err(String::from("命令执行失败"));
    }
}

#[path = "../platform/mod.rs"]
mod platform;
use platform::{App, Installed, Screenshot};

#[path = "../utils/mod.rs"]
mod utils;
use utils::local_server;

// 获取本机安装的 app 列表
#[TCMD]
pub fn get_installed_apps() -> Vec<App> {
    let result = Installed::new();
    let apps = result.apps;
    return apps;
}

// 获取屏幕截图
#[TCMD]
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

#[TCMD]
pub fn preview_file(path: String) -> Result<utils::preview::File, String> {
    utils::preview::preview_file(path)
}

#[TCMD]
pub fn local_shared_server(state: State<'_, local_server::ServerState>) -> Result<String, String> {
    println!("local_server: {:?}", state.lock().unwrap());
    // if stop {
    //     return local_server::stop_file_server(state);
    // }
    // local_server::start_file_server(state, path)
    Ok("".to_string())
}