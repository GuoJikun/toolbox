use std::process::Command;

/// 获取已安装的软件列表
#[tauri::command]
pub fn list_installed_software() -> Result<String, String> {
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


