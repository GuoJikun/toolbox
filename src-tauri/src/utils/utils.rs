use serde::Deserialize;
use serde_json;
use serde_json::json;
use std::{
    env, fs, io,
    path::{Path, PathBuf},
    process::Command as StdCommand,
};
use tauri::{path::BaseDirectory, App, AppHandle, Manager};

use tauri_plugin_shell::{process::Command, ShellExt};
use walkdir::WalkDir;

#[allow(dead_code)]
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

                let config = Config::new(&file_path);
                let config = config.config.as_object().unwrap();
                let id = config.get("id").unwrap().as_str().unwrap();
                let path = dist_path.join(format!("{}.json", id));
                let _ = match config.get("primissions") {
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

// 获取应用版本号的函数
pub fn get_app_version(app: AppHandle) -> String {
    // 获取 tauri.config.json 配置
    let conf = app.package_info();
    // 获取版本号
    let version = conf.version.clone();

    version.to_string()
}

pub fn init_local_http_server(app: AppHandle) -> u32 {
    let binding = app
        .path()
        .resolve("plugins", BaseDirectory::Resource)
        .unwrap();

    let static_path = binding.to_str().unwrap();
    let shell = app.shell();
    let caddy: Command = shell.sidecar("caddy").unwrap();
    let args = vec![
        "file-server",
        "--listen",
        "localhost:6543",
        "--root",
        static_path,
    ];
    let _ = match caddy.args(args).spawn() {
        Ok((_rx, child)) => {
            let pid = child.pid();
            return pid;
        }
        Err(_) => 0,
    };
    0
}

pub fn kill_local_http_server(app: AppHandle, pid: u32) {
    let shell = app.shell();
    let os = env::consts::OS;

    match os {
        "windows" => {
            shell
                .command("taskkill")
                .arg("/F")
                .arg("/PID")
                .arg(pid.to_string())
                .spawn()
                .unwrap();
        }
        "linux" | "macos" => {
            shell
                .command("kill")
                .arg("-9")
                .arg(pid.to_string())
                .spawn()
                .unwrap();
        }
        _ => {
            panic!("Unsupported operating system");
        }
    }
}

pub fn kill_server_by_name(process_name: &str) {
    let os = env::consts::OS;

    match os {
        "windows" => {
            // 获取 PID
            let output = StdCommand::new("tasklist")
                .arg("/FI")
                .arg(format!("IMAGENAME eq {}", process_name))
                .output()
                .expect("Failed to execute command");

            let output_str = String::from_utf8_lossy(&output.stdout);
            for line in output_str.lines() {
                if line.contains(process_name) {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if let Some(pid) = parts.get(1) {
                        let pid: u32 = pid.parse().expect("Failed to parse PID");
                        // 杀死进程
                        StdCommand::new("taskkill")
                            .arg("/F")
                            .arg("/PID")
                            .arg(pid.to_string())
                            .spawn()
                            .expect("Failed to execute command");
                    }
                }
            }
        }
        "linux" | "macos" => {
            // 获取 PID
            let output = StdCommand::new("pgrep")
                .arg(process_name)
                .output()
                .expect("Failed to execute command");

            let output_str = String::from_utf8_lossy(&output.stdout);
            for line in output_str.lines() {
                let pid: u32 = line.parse().expect("Failed to parse PID");
                // 杀死进程
                StdCommand::new("kill")
                    .arg("-9")
                    .arg(pid.to_string())
                    .spawn()
                    .expect("Failed to execute command");
            }
        }
        _ => {
            panic!("Unsupported operating system");
        }
    }
}

#[allow(dead_code)]
pub fn print_current_time() {
    let now = chrono::Local::now();
    println!("Current time: {}", now.format("%Y-%m-%d %H:%M:%S%.3f %z"));
}
