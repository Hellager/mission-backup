use chrono::Local;
// Tauri related
use tauri::{ Wry, Manager, AppHandle, Window as TauriWindow,
    SystemTray, CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem, SystemTrayEvent,
    RunEvent, WindowEvent
  };

// Data file related
use std::fs::{ OpenOptions, read_to_string };
use std::io::Write;
use std::path::PathBuf;

// base64 related
use base64::{ encode, decode };
use std::string::String;

// // JSON serialize
// use serde::{Serialize, Deserialize};

// Webview version related
use wry::webview::webview_version;
use native_dialog::{ MessageDialog, MessageType };

// Open folder related
use std::process::Command;
use std::env::consts;

// Add window shadows
// use wry::application::window::Window;
use window_shadows::set_shadow;

// Single instance
// use single_instance::SingleInstance;

// General
use crate::data_types::APPData;
use log::{ debug, error };

// Using absolute path
use std::env::current_dir;
#[allow(unused_imports)]
use tauri::api::path::home_dir;

// Const varibles
static APP_VERSION: &str = "v1.0.0";

#[allow(unused)]
static APP_DIR: &str = "mission-backup";

pub fn get_app_home_dir() -> PathBuf {
  #[cfg(target_os = "windows")]
  if let Err(e) = current_dir() {
    error!("Failed to get app home dir. Errmsg: {}", e);
    std::process::exit(-1);
  } else {
    return current_dir().unwrap();
  }

  #[cfg(not(target_os = "windows"))]
  match home_dir() {
    None => {
      error!("Failed to get app home dir");
      std::process::exit(-1);      
    },
    Some(path) => {
      return path.join(APP_DIR);
    }
  }
}

pub fn get_app_log_dir() -> PathBuf {
  get_app_home_dir().join("logs")
}

/**
 * Load data from ./config.dat file, if not exists, create file
 */
pub fn load_data_file() -> APPData {
    let app_home_dir = get_app_home_dir();
    let data_file_path = app_home_dir.join("config.dat");
  
    #[allow(unused_mut)]
    let mut data_file = OpenOptions::new().read(true).write(true).create(true).open(data_file_path.to_str().unwrap()).unwrap();
  
    let local_time_offset = Local::now().offset().local_minus_utc();
    let local_language = if local_time_offset == (8 * 3600) {"zh-CN"} else {"en-US"};

    if data_file.metadata().unwrap().len() == 0 {
      debug!("Create config.dat file");
      let default_config = format!(
        r#"{{
          "setting": {{
            "is_auto_start": true,
            "is_light_theme": true,
            "is_password_protected": true,
            "password": "not set yet",
            "is_close_to_tray": true,
            "language": "{}",
            "monitor_delay": 5,
            "software_version": "{}",
            "is_notify_when_create_backup_success": true,
            "is_notify_when_create_backup_failed": true
          }},
          "list": []
        }}
      "#, local_language, APP_VERSION);
  
      let _create_data_file = data_file.write(encode(&default_config.as_bytes()).as_bytes());
    }
  
    let encoded_data = read_to_string(data_file_path).unwrap();
    let decoded_data = decode(&encoded_data).unwrap();
    let decoded_string = String::from_utf8_lossy(&decoded_data);

    let program_data: APPData = serde_json::from_str(&decoded_string).unwrap();
    debug!("Current software version {}", program_data.setting.software_version);

    program_data
}

/**
 * Save data to file
 */
pub fn save_data_to_file(data: APPData) -> bool {
  let app_home_dir = get_app_home_dir();
  let data_file_path = app_home_dir.join("config.dat");

  let mut data_file = OpenOptions::new().write(true).create(true).truncate(true).open(data_file_path.to_str().unwrap()).unwrap();

  let data_to_string = serde_json::to_string(&data).unwrap();

  let encoded_data = encode(&data_to_string.as_bytes());

  if let Err(e) = data_file.write(encoded_data.as_bytes()) {
    error!("Save data to file {} failed, errMsg: {}", data_file_path.display(), e);
    return false;    
  }

  true
}

/**
 * Listen to app event
 */
pub fn handle_app_event(_app_handle: &AppHandle<Wry>, event: RunEvent) {
  match event {
      RunEvent::Exit => {}
      RunEvent::ExitRequested { .. } => {}
      RunEvent::WindowEvent { label, event, .. } => {
          match event {
              WindowEvent::CloseRequested { api, .. } => {
                  if label == "main" {
                      let _ = _app_handle.get_window("main").unwrap().hide();
                      _app_handle.emit_all("close", {}).unwrap();
                      debug!("Rust try to close");
                      api.prevent_close()
                  }
              }
              _ => {}
          }
      }
      RunEvent::Ready => {}
      RunEvent::Resumed => {}
      RunEvent::MainEventsCleared => {}
      _ => {}
  }
}

/**
 * Create system tray
 */
pub fn create_system_tray() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit Program");
    let hide = CustomMenuItem::new("hide".to_string(), "Close to tray");
    let tray_menu = SystemTrayMenu::new()
      .add_item(hide)
      .add_native_item(SystemTrayMenuItem::Separator)
      .add_item(quit);
    SystemTray::new().with_menu(tray_menu)
}

/**
 * Listen and handle system tray event
 */
pub fn handle_system_tray_event(app: &AppHandle<Wry>, event: SystemTrayEvent) {
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
                app.emit_all("close", {}).unwrap();
                debug!("System tray try to close");
              }
              "hide" => {
                app.get_window("main").unwrap().hide().unwrap();
                app.emit_all("hide", {}).unwrap();
                debug!("System tray try to hide/show");
              }
              _ => {}
            }
        }
        _ => {}
    }
}

/**
 * listen and handle app event
 */
// pub fn handle_app_event(_app_handle: &AppHandle<Wry>, event: RunEvent) {
//     match event {
//         RunEvent::Exit => {}
//         RunEvent::ExitRequested { .. } => {}
//         RunEvent::WindowEvent { label, event, .. } => {
//             match event {
//                 WindowEvent::CloseRequested { api, .. } => {
//                     if label == "main" {
//                         let _ = _app_handle.get_window("main").unwrap().hide();
//                         api.prevent_close()
//                     }
//                 }
//                 _ => {}
//             }
//         }
//         RunEvent::Ready => {}
//         RunEvent::Resumed => {}
//         RunEvent::MainEventsCleared => {}
//         _ => {}
//     }
// }

/**
 * Check WebView2 available
 */
pub fn check_webview2_available(scope: AppHandle) {
    let system_webview2_version = webview_version().unwrap();
    debug!("System webview2 version: {}", system_webview2_version);

    if system_webview2_version == ""  {
        error!("System missing webview2 bundle");

        let result = MessageDialog::new()
            .set_type(MessageType::Error)
            .set_title("Error")
            .set_text("Missing Webview Bundle")
            .show_confirm()
            .unwrap();
        
        if result {
            let _jump_to_download_page = tauri::scope::ShellScope::open(
                &scope.shell_scope(),
                "https://developer.microsoft.com/zh-cn/microsoft-edge/webview2/#download-section",
                None,
            );
            std::process::exit(0);
        }
    }
}

/**
 * Directly open a folder window
 * https://stackoverflow.com/questions/66485945/with-rust-open-explorer-on-a-file
 */
pub fn open_folder_window(path: &str) {
  debug!("try to open {} window", path);
  if consts::OS == "linux" {
    Command::new("xdg-open")
            .arg(path)
            .spawn()
            .unwrap();
  } else if consts::OS == "macos" {
    Command::new("open")
            .arg(path)
            .spawn()
            .unwrap();
  } else if consts::OS == "windows" {
    Command::new("open")
            .arg(path)
            .spawn()
            .unwrap();
  }
}

/**
 * Add window shadows to app, not available on linux now
 * https://github.com/tauri-apps/window-shadows
 */
pub fn initialize_window_shadow(window: &TauriWindow, is_shadow_enable: bool) {
  if let Err(e) = set_shadow(window, is_shadow_enable) {
    error!("Failed to add native window shadow, errMsg: {:?}", e);
  }
}

// /**
//  * Make sure there is only one instance running
//  * Not working now
//  */
// pub fn check_instance(name: &str) {
  
//   let instance = SingleInstance::new(name).unwrap();
//   debug!("Check instance {} whether single {}", name, instance.is_single());
//   if !instance.is_single() {
//     MessageDialog::new()
//     .set_type(MessageType::Error)
//     .set_title("Error")
//     .set_text("Existing instance")
//     .show_confirm()
//     .unwrap();

//     debug!("{} is not single", name);
//     std::process::exit(0);

//   } else {
//       debug!("{} is single", name);
//   }
// }
