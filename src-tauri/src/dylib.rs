use libloading::{Library, Symbol};
use std::collections::HashMap;
use std::ffi::{CStr, CString};

use std::path::PathBuf;

fn get_lib_ext() -> String {
    if cfg!(target_os = "windows") {
        return String::from(".dll");
    } else if cfg!(target_os = "macos") {
        return String::from(".dylib");
    } else {
        return String::from(".so");
    }
}
fn collect_dylib() -> HashMap<String, PathBuf> {
    let ext = get_lib_ext();
    let mut handlers: HashMap<String, PathBuf> = HashMap::new();
    let plugins_dir = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join("plugins");

    plugins_dir
        .read_dir()
        .unwrap()
        .for_each(|entry| match entry {
            Ok(entry) => {
                let path = entry.path();

                if path.is_dir() {
                    // let tmp_path = plugins_dir.clone();
                    let lib_path = path.join("lib").join(format!("index{}", ext));
                    if lib_path.exists() {
                        handlers.insert(
                            path.file_name().unwrap().to_str().unwrap().to_string(),
                            lib_path.clone(),
                        );
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to read directory: {}", e);
            }
        });
    return handlers;
}
#[tauri::command]
pub fn dynamic_command(plugin: String, fn_name: String) -> Result<String, String> {
    let lib_path_map = collect_dylib();

    let lib_path = lib_path_map.get(&plugin).unwrap();
    // 动态生成函数名并转换为以空终止符结尾的 CString
    let func_name_cstr = CString::new(fn_name).map_err(|e| e.to_string())?;
    unsafe {
        let lib = Library::new(lib_path).map_err(|e| e.to_string())?;
        let func: Symbol<unsafe extern "C" fn() -> *const i8> = lib
            .get(func_name_cstr.as_bytes_with_nul())
            .map_err(|e| e.to_string())?;
        let result_c_str = CStr::from_ptr(func());
        let result_str = result_c_str.to_str().map_err(|e| e.to_string())?;
        Ok(result_str.to_string())
    }
}
