use tauri_plugin_highlander::*;
use tauri::Wry;

/**
 * Restrict only one instance of program can run at a same time
 */
pub fn initialize_plugin_highlander(event_name: &str) -> Highlander<Wry> {
    HighlanderBuilder::default().event(event_name.to_string()).build()
}