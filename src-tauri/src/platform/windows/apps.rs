use std::{
    fs::{self},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use std::ffi::OsString;
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use windows::core::{Interface, GUID, HRESULT, PCWSTR};
use windows::Win32::Foundation::{MAX_PATH, S_OK};
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CoUninitialize, IPersistFile, CLSCTX_INPROC_SERVER,
    COINIT_MULTITHREADED, STGM_READ,
};
use windows::Win32::UI::Shell::IShellLinkW;

// 手动定义 CLSID_ShellLink
const CLSID_SHELL_LINK: GUID = GUID::from_u128(0x00021401_0000_0000_C000_000000000046);

// 定义一个结构体来存储快捷方式信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortcutInfo {
    pub target_path: Option<PathBuf>,
    pub description: Option<OsString>,
    pub working_directory: Option<OsString>,
    pub icon_location: Option<OsString>,
    pub icon_index: i32,
    pub name: Option<String>,
}

#[allow(dead_code)]
impl ShortcutInfo {
    pub fn target_path(&self) -> Option<&PathBuf> {
        self.target_path.as_ref()
    }

    pub fn description(&self) -> Option<&OsString> {
        self.description.as_ref()
    }

    pub fn working_directory(&self) -> Option<&OsString> {
        self.working_directory.as_ref()
    }

    pub fn icon_location(&self) -> Option<&OsString> {
        self.icon_location.as_ref()
    }

    pub fn icon_index(&self) -> i32 {
        self.icon_index
    }

    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }
}

// pub fn get_shortcut_info(lnk_path: &str) -> Option<ShortcutInfo> {
//     unsafe {
//         // 初始化COM库
//         let hr: HRESULT = CoInitializeEx(None, COINIT_MULTITHREADED);
//         if hr != S_OK {
//             eprintln!(
//                 "Failed to initialize COM library with HRESULT: 0x{:X}",
//                 hr.0
//             );
//             CoUninitialize(); // 添加此行以确保在错误时释放COM库
//             return None;
//         }

//         // 创建IShellLinkW对象
//         let shell_link: Result<IShellLinkW, _> =
//             CoCreateInstance(&CLSID_SHELL_LINK, None, CLSCTX_INPROC_SERVER);

//         if let Ok(shell_link) = shell_link {
//             // 获取IPersistFile接口
//             let persist_file: IPersistFile = shell_link.cast().unwrap();
//             let lnk_path = OsString::from(lnk_path);
//             let lnk_path_wide: Vec<u16> = lnk_path.encode_wide().chain(Some(0)).collect();

//             // 检查.lnk文件路径
//             if !Path::new(&lnk_path).exists() {
//                 eprintln!(
//                     "The .lnk file does not exist at the specified path: {:?}",
//                     lnk_path
//                 );
//                 CoUninitialize();
//                 return None;
//             }

//             // 加载.lnk文件
//             let hr = persist_file.Load(PCWSTR(lnk_path_wide.as_ptr()), STGM_READ);

//             if hr.is_ok() {
//                 let mut shortcut_info = ShortcutInfo {
//                     target_path: None,
//                     description: None,
//                     working_directory: None,
//                     icon_location: None,
//                     icon_index: 0,
//                 };

//                 // 获取目标路径
//                 let mut target_path: [u16; MAX_PATH as usize] = [0; MAX_PATH as usize];
//                 let mut find_data: windows::Win32::Storage::FileSystem::WIN32_FIND_DATAW =
//                     std::mem::zeroed();
//                 let hr = shell_link.GetPath(&mut target_path, &mut find_data, 0);
//                 if hr.is_ok() {
//                     let len = target_path
//                         .iter()
//                         .position(|&c| c == 0)
//                         .unwrap_or(target_path.len());
//                     let target_path_osstring = OsString::from_wide(&target_path[..len]);
//                     shortcut_info.target_path = Some(PathBuf::from(target_path_osstring));
//                 }

//                 // 获取描述信息
//                 let mut description: [u16; 512] = [0; 512];
//                 let hr = shell_link.GetDescription(&mut description);
//                 if hr.is_ok() {
//                     let len = description
//                         .iter()
//                         .position(|&c| c == 0)
//                         .unwrap_or(description.len());
//                     let description_osstring = OsString::from_wide(&description[..len]);
//                     shortcut_info.description = Some(description_osstring);
//                 }

//                 // 获取工作目录
//                 let mut working_directory: [u16; MAX_PATH as usize] = [0; MAX_PATH as usize];
//                 let hr = shell_link.GetWorkingDirectory(&mut working_directory);
//                 if hr.is_ok() {
//                     let len = working_directory
//                         .iter()
//                         .position(|&c| c == 0)
//                         .unwrap_or(working_directory.len());
//                     let working_directory_osstring = OsString::from_wide(&working_directory[..len]);
//                     shortcut_info.working_directory = Some(working_directory_osstring);
//                 }

//                 // 获取图标位置
//                 let mut icon_location: [u16; MAX_PATH as usize] = [0; MAX_PATH as usize];
//                 let mut icon_index: i32 = 0;
//                 let hr = shell_link.GetIconLocation(&mut icon_location, &mut icon_index);
//                 if hr.is_ok() {
//                     let len = icon_location
//                         .iter()
//                         .position(|&c| c == 0)
//                         .unwrap_or(icon_location.len());
//                     let icon_location_osstring = OsString::from_wide(&icon_location[..len]);
//                     shortcut_info.icon_location = Some(icon_location_osstring);
//                     shortcut_info.icon_index = icon_index;
//                 }

//                 CoUninitialize();
//                 return Some(shortcut_info);
//             }
//         }

//         // 释放COM库
//         CoUninitialize();
//     }
//     None
// }

fn get_shortcut_info(lnk_paths: &Vec<PathBuf>) -> Vec<ShortcutInfo> {
    unsafe {
        // 初始化COM库
        let hr: HRESULT = CoInitializeEx(None, COINIT_MULTITHREADED);
        if hr != S_OK {
            eprintln!(
                "Failed to initialize COM library with HRESULT: 0x{:X}",
                hr.0
            );
            return vec![]; // 返回空的结果
        }
        let mut results: Vec<ShortcutInfo> = Vec::new();
        // 创建IShellLinkW对象
        let shell_link: Result<IShellLinkW, _> =
            CoCreateInstance(&CLSID_SHELL_LINK, None, CLSCTX_INPROC_SERVER).map_err(|_| {
                return results.clone();
            });

        let shell_link = shell_link.unwrap();

        for lnk_path in lnk_paths.iter() {
            // 检查.lnk文件路径
            if !lnk_path.exists() {
                eprintln!(
                    "The .lnk file does not exist at the specified path: {:?}",
                    lnk_path
                );
                continue;
            }

            // 获取IPersistFile接口
            let persist_file: IPersistFile = shell_link.cast().unwrap();
            let lnk_path_wide: Vec<u16> = OsString::from(lnk_path)
                .encode_wide()
                .chain(Some(0))
                .collect();

            // 加载.lnk文件
            let hr = persist_file.Load(PCWSTR(lnk_path_wide.as_ptr()), STGM_READ);
            if hr.is_ok() {
                let name: Option<String> = lnk_path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .map(|s| s.to_string());
                let mut shortcut_info = ShortcutInfo {
                    name,
                    target_path: None,
                    description: None,
                    working_directory: None,
                    icon_location: None,
                    icon_index: 0,
                };

                // 获取目标路径
                let mut target_path: [u16; MAX_PATH as usize] = [0; MAX_PATH as usize];
                let mut find_data: windows::Win32::Storage::FileSystem::WIN32_FIND_DATAW =
                    std::mem::zeroed();
                let hr = shell_link.GetPath(&mut target_path, &mut find_data, 0);
                if hr.is_ok() {
                    let len = target_path
                        .iter()
                        .position(|&c| c == 0)
                        .unwrap_or(target_path.len());
                    let target_path_osstring = OsString::from_wide(&target_path[..len]);
                    shortcut_info.target_path = Some(PathBuf::from(target_path_osstring));
                }

                // 获取描述信息
                let mut description: [u16; 512] = [0; 512];
                let hr = shell_link.GetDescription(&mut description);
                if hr.is_ok() {
                    let len = description
                        .iter()
                        .position(|&c| c == 0)
                        .unwrap_or(description.len());
                    let description_osstring = OsString::from_wide(&description[..len]);
                    shortcut_info.description = Some(description_osstring);
                }

                // 获取工作目录
                let mut working_directory: [u16; MAX_PATH as usize] = [0; MAX_PATH as usize];
                let hr = shell_link.GetWorkingDirectory(&mut working_directory);
                if hr.is_ok() {
                    let len = working_directory
                        .iter()
                        .position(|&c| c == 0)
                        .unwrap_or(working_directory.len());
                    let working_directory_osstring = OsString::from_wide(&working_directory[..len]);
                    shortcut_info.working_directory = Some(working_directory_osstring);
                }

                // 获取图标位置
                let mut icon_location: [u16; MAX_PATH as usize] = [0; MAX_PATH as usize];
                let mut icon_index: i32 = 0;
                let hr = shell_link.GetIconLocation(&mut icon_location, &mut icon_index);
                if hr.is_ok() {
                    let len = icon_location
                        .iter()
                        .position(|&c| c == 0)
                        .unwrap_or(icon_location.len());
                    let icon_location_osstring = OsString::from_wide(&icon_location[..len]);
                    shortcut_info.icon_location = Some(icon_location_osstring);
                    shortcut_info.icon_index = icon_index;
                }

                results.push(shortcut_info);
            }
        }

        // 释放COM库
        CoUninitialize();
        results
    }
}

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

// windows 下获取应用程序列表是通过解析开始菜单实现的
impl Installed {
    pub fn new() -> Self {
        let mut installed = Self::default();
        installed.apps = Self::get_apps();
        installed
    }

    fn traverse_dir(dir_path: &Path, lnks: &mut Vec<PathBuf>) {
        if let Ok(entries) = fs::read_dir(dir_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let file_path = entry.path();
                    if file_path.is_dir() {
                        // 如果是目录，则递归调用
                        Self::traverse_dir(dir_path, lnks)
                    } else {
                        // 检查文件扩展名
                        if let Some(extension) = file_path.extension() {
                            if extension == "lnk" {
                                // 处理 .lnk 文件
                                lnks.push(file_path.clone())
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

                    lnks.push(entry.clone())
                }
            }
        }
    }
    fn get_apps() -> Vec<App> {
        let mut apps = Vec::new();
        let lnks = Self::get_lnks();
        let shortcut_info_list = get_shortcut_info(&lnks);
        for info in shortcut_info_list {
            apps.push(App {
                name: info.name().cloned(),
                path: info.target_path().cloned(),
                icon: info.icon_location().map(|os_str| PathBuf::from(os_str)),
            })
        }
        return apps;
    }
    fn get_lnks() -> Vec<PathBuf> {
        let appdata = std::env::var("APPDATA").expect("Failed to get APPDATA environment variable");
        let start_menu_path = Path::new(&appdata).join("Microsoft\\Windows\\Start Menu\\Programs");
        let mut lnks: Vec<PathBuf> = Vec::new();
        Self::traverse_dir(&start_menu_path, &mut lnks);

        let common_start_menu_path =
            Path::new("C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Programs");
        Self::traverse_dir(&common_start_menu_path, &mut lnks);
        return lnks;
    }
}
