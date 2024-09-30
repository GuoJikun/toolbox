use serde_json::json;
use std::{io, path::PathBuf};
use tauri::{path::BaseDirectory, App, Manager};

#[path = "config.rs"]
mod config;
pub use config::Config;

#[allow(dead_code)]
pub fn generate(app: &mut App) -> io::Result<()> {
    let dist_path = app
        .path()
        .resolve("capabilities", BaseDirectory::Resource)
        .unwrap();
    let _src_path = app
        .path()
        .resolve("plugins", BaseDirectory::Resource)
        .unwrap();
    let _ = match fs_extra::dir::get_dir_content(&_src_path) {
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
                let config_data = config.get_data().as_object().unwrap();
                let id = config_data.get("id").unwrap().as_str().unwrap();
                let path = dist_path.join(format!("{}.json", id));
                let _ = match config_data.get("permissions") {
                    Some(permissions) => {
                        let content = json!({
                            "identifier": format!("toolbox-plugin-{}", id),
                            "description": format!("Capability for toolbox-plugin-{}", id),
                            "windows": vec![format!("toolbox-plugin-{}-window", id)],
                            "webviews": vec![format!("toolbox-plugin-{}-webview", id)],
                            "permissions": permissions,
                        });
                        let _ = fs_extra::file::write_all(&path, &content.to_string())
                            .expect("Failed to write capability file");
                    }
                    None => {}
                };
            }
        }
        Err(err) => {
            println!("Error: {:?}", err);
        }
    };

    Ok(())
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
