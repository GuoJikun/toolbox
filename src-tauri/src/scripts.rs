// src-tauri/src/scripts.rs

use std::process::{Command, Output};
use tauri::command;

#[command]
pub fn run_node_script(script: String, args: Vec<String>) -> Result<String, String> {
    let mut full_args = vec![script];
    full_args.extend(args);

    let output: Output = Command::new("node")
        .args(&full_args)
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(stdout)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        Err(stderr)
    }
}

#[command]
pub fn run_php_script(script: String, args: Vec<String>) -> Result<String, String> {
    run_script("php", script, args)
}

#[command]
pub fn run_python_script(script: String, args: Vec<String>) -> Result<String, String> {
    run_script("python", script, args)
}

fn run_script(command: &str, script: String, args: Vec<String>) -> Result<String, String> {
    let mut full_args = vec!["-c".to_string(), script];
    full_args.extend(args);
    println!("Running script: {} {:?}", command, full_args);
    let output: Output = Command::new(command)
        .args(&full_args)
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(stdout)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        Err(stderr)
    }
}
