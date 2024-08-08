use std::{
    fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Installed {
    pub apps: Vec<App>,
}

impl Default for Installed {
    fn default() -> Self {
        Installed { apps: Vec::new() }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct App {
    pub name: Option<String>,
    pub path: Option<PathBuf>,
    pub icon: Option<PathBuf>,
}

impl Default for App {
    fn default() -> Self {
        App {
            name: None,
            path: None,
            icon: None,
        }
    }
}

impl App {
    pub fn new(name: Option<String>, path: Option<PathBuf>, icon: Option<PathBuf>) -> Self {
        App { name, path, icon }
    }
}

#[cfg(target_os = "windows")]
use lnk::ShellLink;
#[cfg(target_os = "windows")]
impl Installed {
    pub fn new() -> Self {
        let apps = Self::get_apps();
        Installed { apps }
    }
    /// 解析 .lnk 文件的目标路径
    fn resolve_lnk(lnk_path: &Path) -> Option<App> {
        // 过滤掉 Windows PowerShell 的快捷方式, 等待 lnk crate 修复
        if lnk_path
            .display()
            .to_string()
            .contains("Windows PowerShell")
        {
            return None;
        }
        // 使用 ShellLink:: 读取快捷方式
        match ShellLink::open(lnk_path) {
            Ok(shell_link) => match shell_link.link_info() {
                Some(link_info) => {
                    let name = shell_link.name().clone();
                    let path = link_info.local_base_path().clone().map(PathBuf::from);
                    let icon = shell_link.icon_location().clone().map(PathBuf::from);
                    return Option::Some(App::new(name, path, icon));
                }
                _ => {
                    return None;
                }
            },
            Err(_) => return None,
        };
    }
    fn traverse_dir(dir_path: &Path, apps: &mut Vec<App>) {
        if let Ok(entries) = fs::read_dir(dir_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let file_path = entry.path();
                    if file_path.is_dir() {
                        // 如果是目录，则递归调用
                        Self::traverse_dir(&file_path, apps);
                    } else {
                        // 检查文件扩展名
                        if let Some(extension) = file_path.extension() {
                            if extension == "lnk" {
                                // 处理 .lnk 文件
                                match Self::resolve_lnk(&file_path) {
                                    Some(tmp) => {
                                        apps.push(tmp);
                                    }
                                    None => {}
                                }
                            }
                        }
                    }
                }
            }
        } else {
            let entry = fs::read_link(dir_path).expect("Failed to read link");
            let file_path = entry.as_path();
            // 检查文件扩展名
            if let Some(extension) = file_path.extension() {
                if extension == "lnk" {
                    // 处理 .lnk 文件

                    match Self::resolve_lnk(&file_path) {
                        Some(tmp) => {
                            apps.push(tmp);
                        }
                        None => {}
                    }
                }
            }
        }
    }
    fn get_apps() -> Vec<App> {
        let appdata = std::env::var("APPDATA").expect("Failed to get APPDATA environment variable");
        let start_menu_path = Path::new(&appdata).join("Microsoft\\Windows\\Start Menu\\Programs");
        let mut app_paths: Vec<App> = Vec::new();
        Self::traverse_dir(&start_menu_path, &mut app_paths);

        let common_start_menu_path =
            Path::new("C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Programs");
        Self::traverse_dir(&common_start_menu_path, &mut app_paths);
        return app_paths;
    }
}

#[cfg(target_os = "macos")]
use plist;
#[cfg(target_os = "macos")]
use std::{fs, io::BufReader, path::Path};

#[cfg(target_os = "macos")]
impl Installed {
    pub fn new() -> Self {
        let mut apps: Vec<App> = Vec::new();

        // 应用程序安装目录
        let applications_dir = Path::new("/Applications");

        // 获取应用程序列表
        if let Ok(entries) = fs::read_dir(applications_dir) {
            for entry in entries.filter_map(Result::ok) {
                let app_path = entry.path();

                // 确保是应用程序包（以 .app 结尾）
                if app_path.extension().and_then(|e| e.to_str()) == Some("app") {
                    // 获取应用程序名称
                    let app_name = app_path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("Unknown");

                    // 获取 Info.plist 文件路径
                    let info_plist_path = app_path.join("Contents/Info.plist");

                    // 解析 Info.plist 文件
                    if let Ok(plist_data) = Installed::read_plist(&info_plist_path) {
                        // 应用程序名称和版本
                        let _app_version = plist_data
                            .as_dictionary()
                            .and_then(|dict| dict.get("CFBundleShortVersionString"))
                            .and_then(|v| v.as_string())
                            .unwrap_or("Unknown");

                        // 应用程序可执行文件路径
                        let executable_path = app_path.join("Contents/MacOS").join(
                            plist_data
                                .as_dictionary()
                                .and_then(|dict| dict.get("CFBundleExecutable"))
                                .and_then(|v| v.as_string())
                                .unwrap_or(""),
                        );

                        // 应用程序图标路径
                        let icon_path = app_path.join("Contents/Resources/AppIcon.icns");

                        // 输出应用程序信息
                        apps.push(App {
                            name: app_name.to_string(),
                            path: executable_path.to_str().unwrap().to_string(),
                            icon: icon_path.to_str().unwrap().to_string(),
                        });
                    }
                }
            }
        } else {
            eprintln!("Failed to read applications directory.");
        }

        Installed { apps }
    }

    // 读取并解析 plist 文件
    fn read_plist(path: &Path) -> Result<plist::Value, plist::Error> {
        let plist_path = Path::new(path);
        // 打开文件并创建一个读取器
        let file = File::open(&plist_path).expect("Failed to open plist file");
        let reader = BufReader::new(file);
        // 使用 plist crate 解析 plist 文件
        match plist::Value::from_reader(reader) {
            Ok(plist::Value::Dictionary(dict)) => {
                // 在这里处理字典类型的 plist 数据
                return Ok(plist::Value::Dictionary(dict));
            }
            Ok(other) => {
                return Ok(other);
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
}
