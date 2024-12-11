use std::process::{Command, Stdio};
use serde_json::{json, Value};

fn is_winget_client_module_installed() -> bool {
    let output = Command::new("powershell")
        .arg("-Command")
        .arg("Get-Module -ListAvailable -Name Microsoft.WinGet.Client")
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                let result = String::from_utf8_lossy(&output.stdout);
                if result.contains("Microsoft.WinGet.Client") {
                    println!("Microsoft.WinGet.Client module is installed.");
                    return true;
                }
            }
            false
        },
        Err(_) => false,
    }
}
fn install_winget_client() -> Result<(), String> {
    // 使用 PowerShell 安装 Microsoft.WinGet.Client 模块
    let install_result = Command::new("powershell")
        .arg("-Command")
        .arg("Install-Module -Name Microsoft.WinGet.Client -Force -AllowClobber -Scope CurrentUser")
        .output();

    match install_result {
        Ok(output) => {
            if output.status.success() {
                println!("Microsoft.WinGet.Client module installed successfully.");
                return Ok(());
            } else {
                return Err("Failed to install Microsoft.WinGet.Client module.".to_string());
            }
        },
        Err(_) => Err("Failed to execute installation command.".to_string()),
    }
}

/// 获取已安装的软件列表
#[tauri::command]
pub fn installed_list() -> Result<Value, String> {
    // 检查 Microsoft.WinGet.Client 模块是否已安装
    if !is_winget_client_module_installed() {
        // 安装 Microsoft.WinGet.Client 模块
        match install_winget_client() {
            Ok(_) => {},
            Err(e) => return Err(e),
        }
    }


    let args = [
        "-Command", 
        "[Console]::OutputEncoding = [System.Text.Encoding]::UTF8; Get-WinGetPackage | ConvertTo-Json -Compress"
    ];
    let output = Command::new("powershell")
        .args(&args)
        .stdout(Stdio::piped())
        .output()
        .map_err(|e| format!("Failed to run winget: {}", e))?;
    
    let output_str = String::from_utf8_lossy(&output.stdout).to_string();
    let installed_software = json!(output_str);

    if output_str.is_empty() {
        return Err("没有找到已安装的软件。".to_string());
    }

    Ok(installed_software)
}

/// 更新选定的软件
#[tauri::command]
pub fn upgrade_software(software_name: String) -> Result<String, String> {
    let output = Command::new("winget")
        .args(&["upgrade", &software_name, "--accept-source-agreements", "--accept-package-agreements"])
        .output()
        .map_err(|e| format!("Failed to upgrade software: {}", e))?;

    let result = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(result)
}

#[tauri::command]
pub fn check_updates() -> Result<Vec<String>, String> {
    let output = Command::new("winget")
        .args(&["upgrade", "--accept-source-agreements", "--accept-package-agreements"])
        .output()
        .map_err(|e| format!("Failed to run winget: {}", e))?;

    let updates = String::from_utf8_lossy(&output.stdout).to_string();

    // 解析更新的软件列表
    let mut software_list = Vec::new();
    for line in updates.lines() {
        // 只匹配包含软件名的行
        if line.contains("  ") {
            let software_name = line.split_whitespace().next().unwrap().to_string();
            software_list.push(software_name);
        }
    }

    if software_list.is_empty() {
        return Ok(vec!["没有可更新的软件.".to_string()]);
    }

    Ok(software_list)
}

/// 安装新软件
#[tauri::command]
pub fn install_software(software_name: String) -> Result<String, String> {
    let output = Command::new("winget")
        .args(&["install", &software_name])
        .output()
        .map_err(|e| format!("Failed to install software: {}", e))?;

    let result = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(result)
}


