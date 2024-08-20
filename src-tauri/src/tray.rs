use tauri::{
    menu::{MenuBuilder, MenuItem, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    App, Manager,
};
pub fn create_tray(app: &mut App) -> tauri::Result<()> {
    let quit = MenuItemBuilder::with_id("quit", "退出").build(app)?;
    let upgrade = MenuItem::with_id(app, "upgrade", "检查更新", true, None::<&str>)?;
    let auto_start = MenuItem::with_id(app, "auto_start", "开机自启", true, None::<&str>)?;
    let menu = MenuBuilder::new(app)
        .items(&[&auto_start, &upgrade, &quit])
        .build()?;

    let _ = TrayIconBuilder::with_id("tray")
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .menu_on_left_click(false)
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "quit" => {
                app.exit(0);
            }
            "upgrade" => {
                println!("Upgrade");
            }
            // Add more events here
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .build(app);

    Ok(())
}
