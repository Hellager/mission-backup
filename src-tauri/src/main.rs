// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod utils;
mod plugins;
mod config;
mod core;
mod db;

use tauri_plugin_autostart::MacosLauncher;
use plugins::on_another_instance;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, Some(vec![])))
        .plugin(tauri_plugin_single_instance::init(on_another_instance))
        .setup(crate::core::setup::setup_handler)
        .system_tray(core::tray::create_system_tray())
        .on_system_tray_event(core::tray::on_system_tray_event)
        .on_window_event(core::window::on_window_event)
        .invoke_handler(crate::core::setup::setup_command())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
