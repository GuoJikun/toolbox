use serde::Deserialize;
use serde_json;
use serde_json::json;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use tauri::{path::BaseDirectory, App, Manager};
use walkdir::WalkDir;

fn copy_to(src: &Path, dst: &Path) -> io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in WalkDir::new(src).min_depth(1) {
        let entry = entry?;
        let path = entry.path();
        let relative_path = path
            .strip_prefix(src)
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;
        let destination = dst.join(relative_path);

        if path.is_dir() {
            fs::create_dir_all(&destination)?;
        } else if path.is_file() {
            fs::copy(&path, &destination)?;
        }
    }

    Ok(())
}

// 读取 JSON 配置文件，返回一个 json::Value
#[derive(Debug, Deserialize)]
pub struct Config {
    config: serde_json::Value,
}
impl Default for Config {
    fn default() -> Self {
        Config {
            config: serde_json::Value::Null,
        }
    }
}

impl Config {
    fn new<P: AsRef<Path>>(path: P) -> Self {
        // 打开文件
        let file = fs::File::open(path).expect("file not found");
        // 创建一个缓冲读取器
        let reader = io::BufReader::new(file);
        // 解析JSON文件
        let config = serde_json::from_reader(reader).expect("error while reading json file");
        Config { config }
    }
}

pub fn generate_capabilities_file(app: &mut App) -> io::Result<()> {
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
                println!("file_path: {:?}", file_path);

                let config = Config::new(&file_path);
                let config = config.config.as_object().unwrap();
                let id = config.get("id").unwrap().as_str().unwrap();
                let path = dist_path.join(format!("{}.json", id));
                let _ = match config.get("primission") {
                    Some(permissions) => {
                        println!("permissions: {:?}", permissions);
                        let content = json!({
                            "identifier": format!("toolbox-plugin-{}", id),
                            "description": format!("Capability for toolbox-plugin-{}", id),
                            "window": format!("toolbox-plugin-{}-window", id),
                            "webview": format!("toolbox-plugin-{}-webview", id),
                            "permissions": permissions,
                        });
                        println!("Writing capability file: {:?}", path);
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

pub fn add_capability(app: &mut App) {
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
