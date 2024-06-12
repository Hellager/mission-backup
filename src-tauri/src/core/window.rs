//! # Window
//! 
//! `window` module contains functions about tauri app window.

use tauri::{ Wry, GlobalWindowEvent, WindowEvent, FileDropEvent, Theme };

/// Handle app window event.
/// 
/// # Arguments
/// 
/// - `event` - An event from app window
/// 
/// # Examples
/// 
/// ```
/// use core::window::on_window_event;
/// 
/// fn main() {
///     tauri::Builder::default()
///         .on_window_event(on_window_event)
///         .run(tauri::generate_context!())
///         .expect("error while running tauri application");
/// }
/// ```
pub fn on_window_event(event: GlobalWindowEvent<Wry>) {
    match event.event() {
        WindowEvent::CloseRequested { api, .. } => {
            let window = event.window();
            window.hide().unwrap();
            api.prevent_close();
            println!("window {} try to close", window.label());

            std::process::exit(0);
        },
        WindowEvent::FileDrop(drop_event) => {
            match drop_event {
                FileDropEvent::Hovered(files) => {
                    println!("files: {:?} hovered", files);
                },
                FileDropEvent::Dropped(files) => {
                    println!("files: {:?} dropped", files);
                },
                FileDropEvent::Cancelled => {

                },
                _ => {}
            }
        },
        WindowEvent::ThemeChanged(theme) => {
            match theme {
                Theme::Light => {
                    println!("system switch to light theme");
                },
                Theme::Dark => {
                    println!("system switch to dark theme");
                },
                _ => {}
            }
        }
        _ => {}
    }
  }