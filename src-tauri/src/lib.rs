mod core;
mod plugins;
mod utils;
mod config;
mod error;
mod schema;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

pub type AppResult<T> = std::result::Result<T, error::AppError>;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(plugins::initialize_plugin_os())
        .plugin(plugins::initialize_plugin_notification())
        .plugin(plugins::initialize_plugin_opener())
        .plugin(plugins::initialize_plugin_single_instance())
        .plugin(plugins::initialize_plugin_autostart())
        .plugin(plugins::initialize_plugin_window_state())

        .plugin(plugins::initialize_plugin_log())
        .setup(core::setup::setup_handler)
        .on_window_event(core::window::on_window_event)
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
