use tauri::{AppHandle, Manager};
use log::error;

pub fn on_another_instance(app: &AppHandle, _argv: Vec<String>, _cwd: String) {
    let windows = app.windows();
            
    if let Some(_) = windows.get("main") {
        if let Err(error) = app.emit_to("main", "instance", {}) {
            error!("Failed to send event about another instance, errMsg: {:?}", error);
        }
    }
}