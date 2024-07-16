// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use libloading::{Library, Symbol};
// use serde_json::to_string;
// use std::collections::HashMap;
// use std::ffi::CStr;

use std::process::Command;
use tauri::{App, Manager};
use tauri_plugin_cli::CliExt;
mod scripts;
mod utils;

// struct DynamicLib {
//     lib: Library,
//     command: String,
// }

// impl DynamicLib {
//     fn get_lib_path(path: &str) -> String {
//         let mut lib_path = "".into();
//         if cfg!(target_os = "windows") {
//             lib_path = format!("{}/screen_capture_lib.dll", path);
//         } else if cfg!(target_os = "macos") {
//             lib_path = format!("{}/libscreen_capture_lib.dylib", path);
//         } else {
//             lib_path = format!("{}/libscreen_capture_lib.so", path);
//         }
//         Ok(PathResolver::join(lib_path).unwrap())
//     }
//     fn new(lib_path: &str, command: &str) -> Result<Self, String> {
//         let lib = Library::new(lib_path).map_err(|e| e.to_string())?;
//         Ok(Self { lib, command })
//     }

//     fn run(&self) -> Result<String, String> {
//         unsafe {
//             let func: Symbol<unsafe extern "C" fn() -> *const i8> = self
//                 .lib
//                 .get(self.command.as_bytes())
//                 .map_err(|e| e.to_string())?;
//             let result_c_str = CStr::from_ptr(func());
//             let result_str = result_c_str.to_str().map_err(|e| e.to_string())?;
//             Ok(result_str.to_string())
//         }
//     }
// }

// fn loading_dynamic_lib() {
//     let lib_path = if cfg!(target_os = "windows") {
//         "path/to/screen_capture_lib.dll"
//     } else if cfg!(target_os = "macos") {
//         "path/to/libscreen_capture_lib.dylib"
//     } else {
//         "path/to/libscreen_capture_lib.so"
//     };

//     let lib = Library::new(lib_path).map_err(|e| e.to_string())?;
//     unsafe {
//         let func: Symbol<unsafe extern "C" fn() -> *const i8> = lib
//             .get(b"capture_screenshot\0")
//             .map_err(|e| e.to_string())?;
//         let result_c_str = CStr::from_ptr(func());
//         let result_str = result_c_str.to_str().map_err(|e| e.to_string())?;
//         Ok(result_str.to_string())
//     }
// }

// type CommandHandler = fn(String) -> Result<String, String>;

// #[tauri::command]
// fn dynamic_command(command: String, input: String) -> Result<String, String> {
//     let mut handlers: HashMap<String, CommandHandler> = HashMap::new();

//     handlers.insert("capture_screenshot".into(), capture_screenshot);

//     if let Some(handler) = handlers.get(&command) {
//         handler(input)
//     } else {
//         Err("Unknown command".into())
//     }
// }

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
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

// fn read_config(app_path: &str) -> Result<utils::Config, Box<dyn std::error::Error>> {
//     let config_path_str = format!("{}/vtools/config.json", app_path);

//     let config_obj = utils::read_json_file(config_path_str);
//     match config_obj {
//         Ok(config) => {
//             println!("{:?}", config);
//             Ok(config)
//         }
//         Err(e) => {
//             eprintln!("Failed to read config file: {}", e);
//             Err(e.into())
//         }
//     }
// }

// fn init_plugins(app: &mut App, app_path: &str) {
//     let resource_dir_tmp = app.path().resource_dir();
//     match resource_dir_tmp {
//         Ok(resource_dir) => {
//             let resource_dir = resource_dir.to_str().unwrap();
//             std::fs::read_dir(resource_dir).unwrap().for_each(|entry| {
//                 let entry = entry.unwrap();
//                 let path = entry.path();
//                 if path.is_dir() {
//                     let path_str = path.to_str().unwrap();
//                     if path_str.contains("resource") {
//                         println!("Adding plugin: {}", path_str);
//                         let _ = utils::copy_dir_all(
//                             std::path::Path::new(path_str),
//                             std::path::Path::new(app_path),
//                         );
//                     }
//                 }
//             });
//         }
//         Err(e) => {
//             eprintln!("Failed to get resource directory: {}", e);
//         }
//     }
// }

// fn init_app_dir() -> String {
//     let home_dir = dirs_next::home_dir().expect("Cannot find home directory");
//     let target_directory = home_dir.join(".vtools");

//     if !target_directory.exists() {
//         std::fs::create_dir_all(&target_directory).expect("Cannot create directory");
//     }

//     return target_directory.to_str().unwrap().to_string();
// }

fn main() {
    // let _app_dir = init_app_dir();

    tauri::Builder::default()
        // .setup(move |app| {
        //     init_plugins(app, _app_dir.clone().as_str());
        //     read_config(_app_dir.clone().as_str()).unwrap();
        //     Ok(())
        // })
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
                    println!("{:?}", matches)
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
            // dynamic_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
