use serde_json::{json, from_reader, Value};
use std::{io, path::PathBuf, fs::File};
use tauri::{path::BaseDirectory, App, Manager};

#[path = "config.rs"]
mod config;
pub use config::Config;

fn generate_capability_file_from_config(config: &Config, dist_path: &PathBuf) {
    let config_data = config.get_data().as_object().unwrap();
    let id = config_data.get("id").unwrap().as_str().unwrap();
    let path = dist_path.join(format!("{}.json", id));
    let permissions = config_data.get("permissions");
    let _ = match permissions {
        Some(permissions) => {
            let content = json!({
                "identifier": format!("toolbox-plugin-{}", id),
                "description": format!("Capability for toolbox-plugin-{}", id),
                "windows": vec![format!("toolbox-plugin-{}", id)],
                "webviews": vec![format!("toolbox-plugin-{}", id)],
                "permissions": permissions,
            });
            let _ = fs_extra::file::write_all(&path, &content.to_string())
                .expect("Failed to write capability file");
        }
        None => {log::info!("Get permissions error")}
    };
}
fn generate_capability_file_from_internal_plugins(app: &mut App) -> Result<(), String> {
    let dist_path = app
        .path()
        .resolve("capabilities", BaseDirectory::Resource)
        .unwrap();
    let dir_path = app
        .path()
        .resolve("config", BaseDirectory::Resource)
        .unwrap();
    let src_path = dir_path.join("internal-plugins.json");

    // 打开 JSON 文件
    let file = File::open(src_path).map_err(|_| "Failed to open JSON file".to_string())?;
    let reader = io::BufReader::new(file);

    // 解析 JSON 文件成结构体
    let configs: Vec<Value>  = from_reader(reader).map_err(|e| e.to_string())?;

    for config in configs {
        let conf = Config {
            config,
        };
        generate_capability_file_from_config(&conf, &dist_path);
    }

    Ok(())
}

fn generate_capability_file_from_external_plugins(app: &mut App) -> Result<(), String> {
    let dist_path = app
        .path()
        .resolve("capabilities", BaseDirectory::Resource)
        .unwrap();
    let src_path = app
        .path()
        .resolve("plugins", BaseDirectory::Resource)
        .unwrap();
    let _ = match fs_extra::dir::get_dir_content(&src_path) {
        Ok(dir_content) => {
            if dir_content.files.is_empty() {
                return Ok(());
            }
            let files = dir_content.files.clone();
            for file in files {
                if !file.ends_with("config.json") {
                    continue;
                }
                let file_path = PathBuf::from(file);

                let config = Config::new(&file_path);
                generate_capability_file_from_config(&config, &dist_path);
            }
            Ok(())
        },
        Err(err) => {
            Err(format!("Error reading directory: {}", err))
        }
    };
    Ok(())
}

#[allow(dead_code)]
pub fn generate(app: &mut App) {
    let a = generate_capability_file_from_internal_plugins(app);
    log::info!("Internal Plugins: {:?}", a);
    let _ = generate_capability_file_from_external_plugins(app);
}
#[allow(dead_code)]
pub fn add(app: &mut App) {
    let dir = app
        .path()
        .resolve("capabilities", BaseDirectory::Resource)
        .unwrap();

    let _ = match fs_extra::dir::get_dir_content(dir) {
        Ok(files) => {
            let files = files.files.clone();
            for file in files {
                let file_path = std::path::PathBuf::from(file);
                if file_path.extension().unwrap() != "json" {
                    continue;
                }
                let content = fs_extra::file::read_to_string(&file_path)
                    .expect("Failed to read capability file");
                let _ = app.add_capability(&content);
            }
        }
        Err(_) => {}
    };
}
