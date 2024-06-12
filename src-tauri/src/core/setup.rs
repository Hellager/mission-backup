//! # Setup
//! 
//! `setup` module contains functions about setup tauri app itself and its' commands.
use tauri::{App, Manager};

/// Setup tauri app.
/// 
/// # Arguments
/// 
/// * `app` - An app instance
/// 
/// # Examples
/// 
/// ```
/// use core::setup::setup_handler;
/// 
/// fn main() {
///     tauri::Builder::default()
///         .setup(crate::core::setup::setup_handler)
///         .run(tauri::generate_context!())
///         .expect("error while running tauri application");
/// }
/// ```
pub fn setup_handler(app: &mut App) -> Result<(), Box<dyn std::error::Error + 'static>> {
    use super::window;

    let main_window = app.get_window("main").unwrap();

    // add window shadows to app, not available on linux now
    #[cfg(not(target_os = "linux"))]
    window::init_window_shadow(&main_window, true);

    Ok(())
}
