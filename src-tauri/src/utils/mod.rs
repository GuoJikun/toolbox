use std::{ fs, io, path::Path};
use tauri::{AppHandle};

use walkdir::WalkDir;

pub mod capability;
pub mod config;
pub mod local_server;
pub mod preview;
pub mod shortcut;
pub mod event;
pub mod web_server;

// 获取应用版本号的函数
#[allow(dead_code)]
pub fn get_app_version(app: AppHandle) -> String {
    // 获取 tauri.config.json 配置
    let conf = app.package_info();
    // 获取版本号
    let version = conf.version.clone();

    version.to_string()
}

#[allow(dead_code)]
pub fn print_current_time() {
    let now = chrono::Local::now();
    println!("Current time: {}", now.format("%Y-%m-%d %H:%M:%S%.3f %z"));
}
#[allow(dead_code)]
fn copy_to(src: &Path, dst: &Path) -> io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in WalkDir::new(src).min_depth(1) {
        let entry = entry?;
        let path = entry.path();
        let relative_path = path
            .strip_prefix(src)
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;
        let destination = dst.join(relative_path);

        if path.is_dir() {
            fs::create_dir_all(&destination)?;
        } else if path.is_file() {
            fs::copy(&path, &destination)?;
        }
    }

    Ok(())
}
