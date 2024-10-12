use std::error::Error;
use tauri::{AppHandle, Manager};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};


use crate::platform;

pub fn bind(app: AppHandle) -> Result<(), Box<dyn Error>> {
    println!("Binding shortcut");
    let alt_n_space = Shortcut::new(Some(Modifiers::ALT), Code::Space);
    // let space = Shortcut::new(None, Code::Space);
    let shortcuts = vec![alt_n_space];
    let handle = app.clone();
    app.clone().global_shortcut().on_shortcuts(shortcuts, move |_, shortcut, event| {
        if shortcut == &alt_n_space {
            match event.state() {
                ShortcutState::Pressed => {
                    println!("ALT-SPACE Pressed!");
                    let search_window = app.clone().get_window("search");
                    if let Some(search_window) = search_window {
                        let is_visible = search_window.is_visible().unwrap();
                        if !is_visible {
                            search_window.show().unwrap();
                            search_window.set_focus().unwrap();
                        }
                    }
                }
                ShortcutState::Released => {
                    println!("ALT-SPACE Released!");
                }
            }
        }
    }).unwrap();
    platform::init_preview_file(handle);
    Ok(())
}

pub fn unbind(app: AppHandle) -> Result<(), Box<dyn Error>> {
    let alt_n_space = Shortcut::new(Some(Modifiers::ALT), Code::Space);
    app.global_shortcut().unregister(alt_n_space)?;
    Ok(())
}
