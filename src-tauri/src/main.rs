// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::command;

use libloading::{Library, Symbol};
use std::collections::HashMap;
use std::ffi::{CStr, CString};

use serde_json::Value;
use std::path::PathBuf;
use std::process::Command;
use tauri_plugin_cli::CliExt;
mod scripts;
mod utils;

#[command]
fn run_external_program(executable_path: String, args: Vec<String>) -> Result<String, String> {
    let output = Command::new(executable_path)
        .args(&args)
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[command]
fn dynamic_command(plugin: String, fn_name: String) -> Result<String, String> {
    let lib_path_map = collect_dylib();

    let lib_path = lib_path_map.get(&plugin).unwrap();
    // 动态生成函数名并转换为以空终止符结尾的 CString
    let func_name_cstr = CString::new(fn_name).map_err(|e| e.to_string())?;
    unsafe {
        let lib = Library::new(lib_path).map_err(|e| e.to_string())?;
        let func: Symbol<unsafe extern "C" fn() -> *const i8> = lib
            .get(func_name_cstr.as_bytes_with_nul())
            .map_err(|e| e.to_string())?;
        let result_c_str = CStr::from_ptr(func());
        let result_str = result_c_str.to_str().map_err(|e| e.to_string())?;
        Ok(result_str.to_string())
    }
}

fn value_to_map(value: &Value) -> Result<HashMap<String, Value>, String> {
    match value {
        Value::Object(map) => {
            let mut new_map = HashMap::with_capacity(map.len());
            for (k, v) in map {
                new_map.insert(k.clone(), v.clone());
            }
            Ok(new_map)
        }
        _ => Err(String::from("Expected a JSON object")),
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
fn add_capabilities() {
    let capability = tauri::ipc::CapabilityBuilder::new("plugin-b");
    capability
        .window("toolbox-plugin-window-plugin-b")
        .webview("toolbox-plugin-webview-plugin-b")
        .permission("window:allow-is-fullscreen");
}

fn get_lib_ext() -> String {
    if cfg!(target_os = "windows") {
        return String::from(".dll");
    } else if cfg!(target_os = "macos") {
        return String::from(".dylib");
    } else {
        return String::from(".so");
    }
}
fn collect_dylib() -> HashMap<String, PathBuf> {
    let ext = get_lib_ext();
    let mut handlers: HashMap<String, PathBuf> = HashMap::new();
    let plugins_dir = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join("plugins");

    plugins_dir
        .read_dir()
        .unwrap()
        .for_each(|entry| match entry {
            Ok(entry) => {
                let path = entry.path();

                if path.is_dir() {
                    // let tmp_path = plugins_dir.clone();
                    let lib_path = path.join("lib").join(format!("index{}", ext));
                    if lib_path.exists() {
                        handlers.insert(
                            path.file_name().unwrap().to_str().unwrap().to_string(),
                            lib_path.clone(),
                        );
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to read directory: {}", e);
            }
        });
    return handlers;
}

fn main() {
    tauri::Builder::default()
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
            scripts::run_node_script,
            scripts::run_php_script,
            scripts::run_python_script,
            dynamic_command,
            add_acl,
            add_capabilities
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
