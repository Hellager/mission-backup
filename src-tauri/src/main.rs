#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
// Import Modules
mod app;
mod logger;
mod plugins;
mod data_types;
mod command;
mod handler;
mod modules;

// Module related
use app::{ get_app_log_dir, create_system_tray, handle_system_tray_event, handle_app_event, initialize_window_shadow };
use handler::{ MissionHandler, MissionHandlerWrapper, initialize_cron_scheduler, AutoStartHandler, AutoStartHandlerWrapper };

// Plugin related
use plugins::initialize_plugin_highlander;

// General
use tauri::async_runtime::block_on;
use std::collections::HashMap;
use std::env::current_exe;
use std::time::Duration;
use hotwatch::Hotwatch;
use std::sync::Mutex;
use auto_launch::*;
use tauri::Manager;
// use log::debug;

fn main() {
  // initialize log
  let app_log_dir = get_app_log_dir();
  logger::initialize_logger(app_log_dir.join("output.log").to_str().unwrap());

  // initialize plugins
  // failed to build on linux and macos 
  // check issue https://github.com/ioneyed/tauri-plugin-highlander/issues/10
  let plugin_highlander = initialize_plugin_highlander("another_instance");

  let context = tauri::generate_context!();
  let _app = tauri::Builder::default()
    .setup(move |app| {
      let app_name = &app.package_info().name;

      // check webview2
      app::check_webview2_available(app.app_handle().clone());
      // app::check_instance(app_name);
      
      // initialize data
      let data = app::load_data_file();

      // initialize MissionJobHandler
      let monitor_delay_time = data.clone().setting.monitor_delay;
      let is_auto_start = data.clone().setting.is_auto_start;
      let monitor = Hotwatch::new_with_custom_delay(Duration::from_secs(monitor_delay_time.into())).expect("failed to initialize monitor_handler");
      let cron = block_on(initialize_cron_scheduler());
      let cron_hashmap = HashMap::new();
      let increment_hashmap = HashMap::new();
      let working_hashmap = HashMap::new();
      let handle = app.handle();

      let mission_store = MissionHandlerWrapper(Mutex::new(MissionHandler {
        cron_handler: cron,
        cron_job_hashmap: cron_hashmap,
        is_initialized: false,
        dir_increment: increment_hashmap,
        working_mission_list: working_hashmap,
        monitor_handler: monitor,
        app_handler: handle,
        window_visiable: true,
        setting: data.setting,
        list: data.list
      }));

      // initialize AutoStartHandler
      // let app_name = &app.package_info().name;
      let current_exe = current_exe().unwrap();

      let auto_start = AutoLaunchBuilder::new()
                                      .set_app_name(&app_name)
                                      .set_app_path(&current_exe.to_str().unwrap())
                                      .set_use_launch_agent(true)
                                      .build()
                                      .unwrap();
      
      if is_auto_start {
        auto_start.enable().unwrap();
      } else {
        if auto_start.is_enabled().unwrap() {
          auto_start.disable().unwrap();
        }
      }

      let auto_start_store = AutoStartHandlerWrapper(Mutex::new(AutoStartHandler {
        handler: auto_start,
      }));

      app.manage(mission_store);
      app.manage(auto_start_store);

      // waiting for rust
      // let splashscreen_window = app.get_window("splashscreen").unwrap();
      let _main_window = app.get_window("main").unwrap();
      // main_window.hide().unwrap();
      let _shadow_window = _main_window.clone();
      
      // // we perform the initialization code on a new task so the app doesn't freeze
      // tauri::async_runtime::spawn(async move {
      //   // initialize your app here instead of sleeping :)
      //   debug!("Waiting for backend initializing...");
      //   std::thread::sleep(std::time::Duration::from_secs(2));
      //   debug!("Backend initializing success, redirect to main page");

      //   // After it's done, close the splashscreen and display the main window
      //   splashscreen_window.close().unwrap();
      //   main_window.show().unwrap();
      // });

      // add window shadows to app, not available on linux now
      #[cfg(not(target_os = "linux"))]
      initialize_window_shadow(&_shadow_window, true);

      // app.emit_all("initialize", APPData { setting: data.setting, list: data.list.clone() });
      Ok(())
    })
    .plugin(plugin_highlander)
    .system_tray(create_system_tray())
    .on_system_tray_event(handle_system_tray_event)
    .invoke_handler(tauri::generate_handler![
        command::show_mainwindow,
        command::close_splashscreen,
        command::initialize_data,
        command::is_program_initialized,
        command::is_password_set,
        command::initialize_program_status,
        command::exit_program,
        command::close_to_tray,
        command::start_timing_save_data,
        command::timing_save_data,
        command::check_path_valid,
        command::get_drop_path_info,
        command::update_list_info,
        command::open_specific_folder_window,
        command::change_setting_is_auto_start,
        command::change_setting_is_light_theme,
        command::change_setting_is_close_to_tray,
        command::change_window_state_by_system_tray,
        command::change_setting_language,
        command::change_setting_is_password_protect,
        command::change_setting_password,
        command::change_monitor_delay,
        command::unlock,
        command::create_mission,
        command::start_mission,
        command::stop_mission,
        command::edit_mission,
        command::delete_mission,
        command::force_delete_mission,
        command::get_mission_backups_status,
        command::open_url,
        command::change_setting_is_notify_when_create_backup_success,
        command::change_setting_is_notify_when_create_backup_failed
    ])
    .build(context)
    .expect("error while running tauri application");

    _app.run(handle_app_event)
}
