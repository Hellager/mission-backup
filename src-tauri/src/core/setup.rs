use tauri::{App, Manager};
use tauri_plugin_window_state::{StateFlags, WindowExt};
use log::debug;
use super::tray;

pub fn setup_handler(app: &mut App) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let window = app.get_webview_window("main").unwrap();
    let state_flags: StateFlags = StateFlags::POSITION | StateFlags::VISIBLE;
    let _ = window.restore_state(state_flags);
    debug!("Window state restored");

    let _ = tray::create_system_tray(app.handle())?;

    Ok(())
}
