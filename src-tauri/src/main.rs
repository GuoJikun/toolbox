// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::command;
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_cli::CliExt;

// 插件相关
mod plugins;
use plugins::{run_node_script, run_php_script, run_python_script};
// 动态库相关/ffi
mod dylib;
use dylib::dynamic_command;

mod command;
use command::{get_installed_apps, run_external_program, screenshot_desktop};

mod utils;

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

#[cfg(desktop)]
mod tray;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec![]),
        ))
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_cli::init())
        .setup(|app| {
            tray::create_tray(app)?;
            utils::generate_capabilities_file(app)?;
            utils::add_capability(app);
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
            get_installed_apps,
            screenshot_desktop
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
