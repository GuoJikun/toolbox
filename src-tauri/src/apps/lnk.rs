extern crate winapi;

use std::ffi::OsString;
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::path::{Path, PathBuf};
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
