use std::error::Error;
use tauri::{AppHandle, Manager};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};


pub fn bind(app: AppHandle) -> Result<(), Box<dyn Error>> {
    let ctrl_n_shortcut = Shortcut::new(Some(Modifiers::ALT), Code::Space);
    let handle = app.clone();
    let global_shortcut = tauri_plugin_global_shortcut::Builder::new().with_handler(move |_, shortcut, event| {
        println!("{:?}", shortcut);
        if shortcut == &ctrl_n_shortcut {
            match event.state() {
                ShortcutState::Pressed => {
                    println!("Ctrl-N Pressed!");
                    let search_window = app.get_window("search");
                    if let Some(search_window) = search_window {
                        let is_visible = search_window.is_visible().unwrap();
                        if !is_visible {
                            search_window.show().unwrap();
                            search_window.set_focus().unwrap();
                        }
                    }

                }
                ShortcutState::Released => {
                    println!("Ctrl-N Released!");
                }
            }
        }
    }).build();
    handle.plugin(global_shortcut)?;
    let global_shortcut_manager = handle.global_shortcut();
    // let is_registered = global_shortcut_manager.is_registered(&ctrl_n_shortcut);
    // if !is_registered {
    //     global_shortcut_manager.register(ctrl_n_shortcut)?;
    // }
    global_shortcut_manager.register(ctrl_n_shortcut)?;
    Ok(())
}

pub fn unbind(app: AppHandle) -> Result<(), Box<dyn Error>> {
    let ctrl_n_shortcut = Shortcut::new(Some(Modifiers::ALT), Code::Space);
    app.global_shortcut().unregister(ctrl_n_shortcut)?;
    Ok(())
}