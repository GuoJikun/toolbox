// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use tauri::{command, path::BaseDirectory, AppHandle, Manager};
use tauri_plugin_cli::CliExt;

// 插件相关
mod plugins;
use plugins::{run_node_script, run_php_script, run_python_script};
// 动态库相关/ffi
mod dylib;
use dylib::dynamic_command;

mod apps;
use apps::Installed;

#[command]
fn run_external_program(executable_path: String, args: Vec<String>) -> Result<String, String> {
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
fn add_acl() {
    let capability = tauri::ipc::CapabilityBuilder::new("plugin-b");
    capability
        .window("toolbox-plugin-window-plugin-b")
        .webview("toolbox-plugin-webview-plugin-b")
        .permission("window:allow-is-fullscreen");
}

#[command]
fn add_capabilities(window: String, webview: String, permissions: Vec<String>) {
    let mut capability = tauri::ipc::CapabilityBuilder::new(window);
    if webview != "" {
        capability = capability.webview(webview);
    }
    for permission in permissions {
        capability = capability.permission(permission);
    }
}

#[command]
fn get_installed_list() -> Vec<apps::App> {
    let result = Installed::new();
    let apps = result.apps;
    return apps;
}

#[command]
fn screenshot_desktop(app: AppHandle) -> Option<String> {
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

#[cfg(desktop)]
mod tray;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(all(desktop))]
            {
                let handle = app.handle();
                tray::create_tray(handle)?;
            }
            Ok(())
        })
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_cli::init())
        .setup(|app| {
            match app.cli().matches() {
                // `matches` here is a Struct with { args, subcommand }.
                // `args` is `HashMap<String, ArgData>` where `ArgData` is a struct with { value, occurrences }.
                // `subcommand` is `Option<Box<SubcommandMatches>>` where `SubcommandMatches` is a struct with { name, matches }.
                Ok(matches) => {
                    matches.args.iter().for_each(|(key, value)| {
                        println!("{}: {:?}", key, value);
                    });
                }
                Err(_) => {}
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            run_external_program,
            run_node_script,
            run_php_script,
            run_python_script,
            dynamic_command,
            add_acl,
            add_capabilities,
            get_installed_list,
            screenshot_desktop
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
