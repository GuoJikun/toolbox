use std::{
    fs::{self},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

extern crate winapi;

use std::ffi::OsString;
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::ptr::null_mut;
use winapi::shared::guiddef::GUID;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::combaseapi::{CoCreateInstance, CoInitializeEx, CoUninitialize};
use winapi::um::objbase::COINIT_APARTMENTTHREADED;
use winapi::um::objidl::IPersistFile;
use winapi::um::shobjidl_core::IShellLinkW;
use winapi::um::winnt::HRESULT;
use winapi::Interface;

// 手动定义 CLSID_ShellLink
const CLSID_SHELL_LINK: GUID = GUID {
    Data1: 0x00021401,
    Data2: 0x0000,
    Data3: 0x0000,
    Data4: [0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
};

// 定义一个结构体来存储快捷方式信息
#[derive(Debug, Clone)]
pub struct ShortcutInfo {
    pub target_path: Option<PathBuf>,
    pub description: Option<OsString>,
    pub working_directory: Option<OsString>,
    pub icon_location: Option<OsString>,
    pub icon_index: i32,
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
}

pub fn get_shortcut_info(lnk_path: &str) -> Option<ShortcutInfo> {
    unsafe {
        // 初始化COM库
        let hr = CoInitializeEx(null_mut(), COINIT_APARTMENTTHREADED);
        if !SUCCEEDED(hr) {
            eprintln!("Failed to initialize COM library with HRESULT: 0x{:X}", hr);
            return None;
        }

        // 创建IShellLinkW对象
        let mut shell_link: *mut IShellLinkW = null_mut();
        let hr: HRESULT = CoCreateInstance(
            &CLSID_SHELL_LINK,
            null_mut(),
            winapi::shared::wtypesbase::CLSCTX_INPROC_SERVER,
            &IShellLinkW::uuidof(),
            &mut shell_link as *mut _ as *mut _,
        );

        if SUCCEEDED(hr) {
            // 获取IPersistFile接口
            let mut persist_file: *mut IPersistFile = null_mut();
            let hr = (*shell_link).QueryInterface(
                &IPersistFile::uuidof(),
                &mut persist_file as *mut _ as *mut _,
            );
            if SUCCEEDED(hr) {
                let lnk_path = OsString::from(lnk_path);
                let lnk_path_wide: Vec<u16> = lnk_path.encode_wide().chain(Some(0)).collect();

                // 检查.lnk文件路径
                if !Path::new(&lnk_path).exists() {
                    eprintln!(
                        "The .lnk file does not exist at the specified path: {:?}",
                        lnk_path
                    );
                    (*persist_file).Release();
                    (*shell_link).Release();
                    CoUninitialize();
                    return None;
                }

                // 加载.lnk文件
                let hr = (*persist_file).Load(lnk_path_wide.as_ptr(), 0);
                if SUCCEEDED(hr) {
                    let mut shortcut_info = ShortcutInfo {
                        target_path: None,
                        description: None,
                        working_directory: None,
                        icon_location: None,
                        icon_index: 0,
                    };

                    // 获取目标路径
                    let mut target_path: [u16; winapi::shared::minwindef::MAX_PATH] =
                        [0; winapi::shared::minwindef::MAX_PATH];
                    let hr = (*shell_link).GetPath(
                        target_path.as_mut_ptr(),
                        target_path.len() as i32,
                        null_mut(),
                        0,
                    );
                    if SUCCEEDED(hr) {
                        let len = target_path
                            .iter()
                            .position(|&c| c == 0)
                            .unwrap_or(target_path.len());
                        let target_path_osstring = OsString::from_wide(&target_path[..len]);
                        shortcut_info.target_path = Some(PathBuf::from(target_path_osstring));
                    }

                    // 获取描述信息
                    let mut description: [u16; 512] = [0; 512];
                    let hr = (*shell_link)
                        .GetDescription(description.as_mut_ptr(), description.len() as i32);
                    if SUCCEEDED(hr) {
                        let len = description
                            .iter()
                            .position(|&c| c == 0)
                            .unwrap_or(description.len());
                        let description_osstring = OsString::from_wide(&description[..len]);
                        shortcut_info.description = Some(description_osstring);
                    }

                    // 获取工作目录
                    let mut working_directory: [u16; winapi::shared::minwindef::MAX_PATH] =
                        [0; winapi::shared::minwindef::MAX_PATH];
                    let hr = (*shell_link).GetWorkingDirectory(
                        working_directory.as_mut_ptr(),
                        working_directory.len() as i32,
                    );
                    if SUCCEEDED(hr) {
                        let len = working_directory
                            .iter()
                            .position(|&c| c == 0)
                            .unwrap_or(working_directory.len());
                        let working_directory_osstring =
                            OsString::from_wide(&working_directory[..len]);
                        shortcut_info.working_directory = Some(working_directory_osstring);
                    }

                    // 获取图标位置
                    let mut icon_location: [u16; winapi::shared::minwindef::MAX_PATH] =
                        [0; winapi::shared::minwindef::MAX_PATH];
                    let mut icon_index: i32 = 0;
                    let hr = (*shell_link).GetIconLocation(
                        icon_location.as_mut_ptr(),
                        icon_location.len() as i32,
                        &mut icon_index,
                    );
                    if SUCCEEDED(hr) {
                        let len = icon_location
                            .iter()
                            .position(|&c| c == 0)
                            .unwrap_or(icon_location.len());
                        let icon_location_osstring = OsString::from_wide(&icon_location[..len]);
                        shortcut_info.icon_location = Some(icon_location_osstring);
                        shortcut_info.icon_index = icon_index;
                    }

                    (*persist_file).Release();
                    (*shell_link).Release();
                    CoUninitialize();
                    return Some(shortcut_info);
                }
                (*persist_file).Release();
            }
            (*shell_link).Release();
        }

        // 释放COM库
        CoUninitialize();
    }
    None
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

impl App {
    pub fn new(name: Option<String>, path: Option<PathBuf>, icon: Option<PathBuf>) -> Self {
        App { name, path, icon }
    }
}
// windows 下获取应用程序列表是通过解析开始菜单实现的
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

        let name: Option<String> = lnk_path
            .file_stem()
            .and_then(|s| s.to_str())
            .map(|s| s.to_string());

        let shell_link = get_shortcut_info(lnk_path.as_os_str().to_str().unwrap());
        match shell_link {
            Some(shell_link) => {
                let path = shell_link.target_path().cloned();
                let icon = shell_link.icon_location();
                return Option::Some(App::new(name, path, icon.map(PathBuf::from)));
            }
            None => {
                return None;
            }
        }
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
