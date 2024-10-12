#[cfg(target_os = "linux")]
#[path = "linux.rs"]
pub mod platform;

#[cfg(target_os = "windows")]
#[path = "windows/mod.rs"]
pub mod platform;

#[cfg(target_os = "macos")]
#[path = "macos/mod.rs"]
pub mod platform;

#[allow(unused)]
pub use platform::{
    PreviewFile, App, Installed, Screenshot, init_preview_file,
};
