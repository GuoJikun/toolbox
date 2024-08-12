#[cfg(target_os = "windows")]
#[path = "windows.rs"]
pub mod platform;

#[cfg(target_os = "macos")]
#[path = "macos.rs"]
pub mod platform;

pub use platform::App;
pub use platform::Installed;
