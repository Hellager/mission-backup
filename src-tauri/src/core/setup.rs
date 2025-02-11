use tauri::{App, Manager};
use tauri_plugin_window_state::{StateFlags, WindowExt};
use crate::config::{load_app_config, save_app_config, AppConfig};
use log::{debug, info, error};
use super::tray;

pub fn setup_handler(app: &mut App) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let window = app.get_webview_window("main").unwrap();
    let state_flags: StateFlags = StateFlags::POSITION | StateFlags::VISIBLE;
    let _ = window.restore_state(state_flags);
    debug!("Window state restored");

    let _ = tray::create_system_tray(app.handle())?;

    let _ = load_app_config().unwrap_or_else(|e| {
        error!("Failed to load config: {}", e);
        let default_config = AppConfig::default();
        if let Err(e) = save_app_config(&default_config) {
            error!("Failed to save default config: {}", e);
        } else {
            info!("Created and saved default config");
        }
        default_config
    });

    Ok(())
}
