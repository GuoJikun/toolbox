// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde_json::json;

use tauri::{command, path::BaseDirectory, Manager, Wry};
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_cli::CliExt;
use tauri_plugin_store::{with_store, StoreCollection};

// 插件相关
mod plugins;
use plugins::{run_node_script, run_php_script, run_python_script};
// 动态库相关/ffi
mod dylib;
use dylib::dynamic_command;

mod command;
use command::{get_installed_apps, run_external_program, screenshot_desktop};

mod utils;
use utils::{capability, shortcut};

// mod platform;
// use platform::{cleanup_preview_file, init_preview_file};

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
    utils::kill_server_by_name("caddy");
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec![]),
        ))
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_cli::init())
        .setup(|app| {
            // 初始化 store
            let stores = app
                .handle()
                .try_state::<StoreCollection<Wry>>()
                .ok_or("Store not found")?;
            let store_path = app
                .path()
                .resolve("config/store.bin", BaseDirectory::Resource)?;
            let _ = with_store(app.handle().clone(), stores, store_path, |store| {
                // 获取版本信息，以便初始化一些参数
                let mut version = utils::get_app_version(app.handle().clone());
                let _ = match store.get("version".to_string()) {
                    Some(tmp) => version = tmp.to_string(),
                    None => {
                        store.insert("version".to_string(), json!(version))?;
                    }
                };
                println!("store version: {}", version);
                let _ = match store.get("local_http_server_pid".to_string()) {
                    Some(tmp) => {
                        let pid: u32 = tmp.as_u64().unwrap() as u32;
                        utils::kill_local_http_server(app.handle().clone(), pid);
                        // 初始化本地 HTTP 服务
                        let pid = utils::init_local_http_server(app.handle().clone());
                        store.insert("local_http_server_pid".to_string(), json!(pid))?;
                    }
                    None => {
                        let pid = utils::init_local_http_server(app.handle().clone());
                        store.insert("local_http_server_pid".to_string(), json!(pid))?;
                    }
                };

                store.save()?;

                Ok(())
            });
            // 创建托盘
            tray::create_tray(app)?;
            // 生成插件的权限文件
            capability::generate(app)?;
            // 添加插件的权限
            capability::add(app);
            // init_preview_file(app.handle().clone());
            // cli
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
            // 绑定全局快捷键
            shortcut::bind(app.handle().clone())?;
            Ok(())
        })
        // .on_window_event(|window, event| match event {
        //     tauri::WindowEvent::CloseRequested { .. } => {
        //         cleanup_preview_file();
        //         let label = window.label();
        //         if label == "previewFile" {
        //             let _ = window.close();
        //         }
        //     }
        //     _ => {}
        // })
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
