use std::sync::Mutex;
use serde_json::json;
use tauri::{command, path::BaseDirectory, Manager};
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_cli::CliExt;
use tauri_plugin_store::StoreExt;

mod utils;
use utils::{
    capability,
    local_server::{ServerState, ServerStateInner},
    shortcut,
};

mod cmds;
use crate::cmds::{pkg, dylib, scripts, run_external_program, get_installed_apps, local_shared_server, preview_file, screenshot_desktop, get_uuid, webp};

mod platform;

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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    utils::kill_server_by_name("caddy");
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec![]),
        ))
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_cli::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            app.manage(Mutex::new(ServerStateInner::default()));
            println!(
                "is_running: {:?}",
                app.state::<ServerState>().lock().unwrap().is_running
            );
            let store_path = app
                .path()
                .resolve("config/store.bin", BaseDirectory::Resource)?;
            // 初始化 store
            let store = app.handle().store_builder(store_path).build()?;

            let cur_version = utils::get_app_version(app.handle().clone());
            // 获取版本信息，以便初始化一些参数
            let _ = match store.get("version".to_string()) {
                Some(tmp) => {
                    if cur_version != tmp.to_string() {
                        store.set("version".to_string(), json!(cur_version));
                    }
                }
                None => {
                    store.set("version".to_string(), json!(cur_version));
                }
            };
            println!("store version: {}", cur_version);
            let _ = match store.get("local_http_server_pid".to_string()) {
                Some(tmp) => {
                    let pid: u32 = tmp.as_u64().unwrap() as u32;
                    utils::kill_local_http_server(app.handle().clone(), pid);
                    // 初始化本地 HTTP 服务
                    let pid = utils::init_local_http_server(app.handle().clone());
                    store.set("local_http_server_pid".to_string(), json!(pid));
                }
                None => {
                    let pid = utils::init_local_http_server(app.handle().clone());
                    store.set("local_http_server_pid".to_string(), json!(pid));
                }
            };

            let _ = store.save()?;

            // 创建托盘
            tray::create_tray(app)?;
            // 生成插件的权限文件
            capability::generate(app);
            // 添加插件的权限
            capability::add(app);
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
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { .. } => {
                let label = window.label();
                if label == "preview" {
                    let _ = window.close();
                }
                utils::kill_server_by_name("caddy");
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            run_external_program,
            scripts::run_node,
            scripts::run_php,
            scripts::run_python,
            dylib::dynamic_command,
            add_capabilities,
            get_installed_apps,
            screenshot_desktop,
            preview_file,
            local_shared_server,
            pkg::installed_list,
            pkg::upgrade_software,
            pkg::install_software,
            pkg::check_updates,
            get_uuid,
            webp::convert_images,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
