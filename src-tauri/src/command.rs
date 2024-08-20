use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use tauri::{command, path::BaseDirectory, AppHandle, Manager};

#[path = "apps/mod.rs"]
mod apps;
use apps::{App, Installed};

// 获取本机安装的 app 列表
#[command]
pub fn get_installed_apps() -> Vec<App> {
    let result = Installed::new();
    let apps = result.apps;
    return apps;
}

// 获取屏幕截图
#[command]
pub fn screenshot_desktop(app: AppHandle) -> Option<String> {
    let _ = match screenshot_desktop::Screenshot::new() {
        Ok(result) => {
            let path = app
                .path()
                .resolve("screenshot.png", BaseDirectory::Temp)
                .unwrap();
            println!("path: {:?}", path);
            result.save(&path).unwrap();
            return Some(path.display().to_string());
        }
        Err(_) => {
            return None;
        }
    };
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
