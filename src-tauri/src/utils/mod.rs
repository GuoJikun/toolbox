use std::{env, fs, io, path::Path, process::Command as StdCommand};
use tauri::{path::BaseDirectory, AppHandle, Manager};

use tauri_plugin_shell::{process::Command, ShellExt};
use walkdir::WalkDir;

pub mod capability;
pub mod config;
pub mod shortcut;

// 获取应用版本号的函数
#[allow(dead_code)]
pub fn get_app_version(app: AppHandle) -> String {
    // 获取 tauri.config.json 配置
    let conf = app.package_info();
    // 获取版本号
    let version = conf.version.clone();

    version.to_string()
}

#[allow(dead_code)]
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
#[allow(dead_code)]
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
#[allow(dead_code)]
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
