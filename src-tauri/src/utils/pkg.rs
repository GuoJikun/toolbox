use std::process::Command;
use tauri::{Manager};

/// 获取已安装的软件列表
#[tauri::command]
fn list_installed_software() -> Result<String, String> {
    let output = Command::new("winget")
        .args(&["list"])
        .output()
        .map_err(|e| format!("Failed to run winget: {}", e))?;

    let installed_software = String::from_utf8_lossy(&output.stdout).to_string();
    if installed_software.is_empty() {
        return Err("没有找到已安装的软件。".to_string());
    }

    Ok(installed_software)
}

/// 更新选定的软件
#[tauri::command]
fn upgrade_software(software_name: String) -> Result<String, String> {
    let output = Command::new("winget")
        .args(&["upgrade", &software_name, "--accept-source-agreements", "--accept-package-agreements"])
        .output()
        .map_err(|e| format!("Failed to upgrade software: {}", e))?;

    let result = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(result)
}

/// 安装新软件
#[tauri::command]
fn install_software(software_name: String) -> Result<String, String> {
    let output = Command::new("winget")
        .args(&["install", &software_name])
        .output()
        .map_err(|e| format!("Failed to install software: {}", e))?;

    let result = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(result)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![list_installed_software, upgrade_software, install_software])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
