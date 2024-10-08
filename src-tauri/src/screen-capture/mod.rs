#[cfg(target_os = "windows")]
#[path = "windows"]
mod platform;

#[cfg(target_os = "macos")]
#[path = "macos"]
mod platform;
