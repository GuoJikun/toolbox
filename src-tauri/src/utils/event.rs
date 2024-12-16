use tauri::{AppHandle, Emitter, EventTarget};

pub fn event_to_webview_window(app: &AppHandle, label: &str, event: &str, data: String) -> Result<(), String> {
    app.emit_to(EventTarget::WebviewWindow { label: label.into() }, event, data).map_err(|e| {
        format!("emit_to label: {} error: {}", label, e)
    })?;
    Ok(())
}