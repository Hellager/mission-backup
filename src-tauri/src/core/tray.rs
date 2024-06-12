//! # Tray
//! 
//! `tray` module contains functions about tauri system tray.

use tauri::{ Wry, Manager, AppHandle,
    SystemTray, CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem, SystemTrayEvent
};

/// Create app system tray.
/// 
/// # Arguments
/// 
/// # Examples
/// 
/// ```
/// use core::tray::create_system_tray;
/// 
/// fn main() {
///     tauri::Builder::default()
///         .system_tray(create_system_tray())
///         .run(tauri::generate_context!())
///         .expect("error while running tauri application");
/// }
/// ```
pub fn create_system_tray() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit Program");
    let hide = CustomMenuItem::new("hide".to_string(), "Close to tray");
    let tray_menu = SystemTrayMenu::new()
      .add_item(hide)
      .add_native_item(SystemTrayMenuItem::Separator)
      .add_item(quit);
    SystemTray::new().with_menu(tray_menu)
}

#[allow(dead_code)]
/// Handle tauri app system tray event.
/// 
/// # Arguments
/// 
/// * `app` - A handle for current tauri app
/// * `event` - An event from system tray
/// 
/// # Examples
/// 
/// ```
/// use core::tray::{create_system_tray, on_system_tray_event};
/// 
/// fn main() {
///     tauri::Builder::default()
///         .system_tray(create_system_tray())
///         .on_system_tray_event(on_system_tray_event)
///         .run(tauri::generate_context!())
///         .expect("error while running tauri application");
/// }
/// ```
pub fn on_system_tray_event(app: &AppHandle<Wry>, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::DoubleClick { position: _ , size: _, .. } => {
            let window = app.get_window("main").unwrap();
            window.unminimize().unwrap();
            window.show().unwrap(); 
            window.set_focus().unwrap();
        },
        SystemTrayEvent::MenuItemClick { id, ..} => {
            match id.as_str() {
              "quit" => {
                app.get_window("main").unwrap().hide().unwrap();
                
                std::process::exit(0);
              }
              "hide" => {
                app.get_window("main").unwrap().hide().unwrap();
              }
              _ => {}
            }
        }
        _ => {}
    }
}
