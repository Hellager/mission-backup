// System related
use tauri::{ command, State, Manager };
use tauri::async_runtime::block_on;

// Module related
use crate::data_types::{ Setting, Mission, APPData, MissionConfig, Response, MissionInfo, DropPathInfo, SavePathInfo };
use crate::handler::{ MissionHandlerWrapper, AutoStartHandlerWrapper };
use crate::app::{save_data_to_file, open_folder_window};
use crate::modules::collector::count_size_with_path;

// General
use std::{fs::metadata, process, path::Path};
use log::{ info, debug, warn, error};
use std::collections::HashMap;
use std::path::PathBuf;
use uuid::Uuid;

#[command]
pub fn check_path_valid(path: &str, expected: &str) -> bool {
  let mut res: bool = false;
  if !metadata(path).is_ok() {
    debug!("path {} invalid", &path);
    return res.into();
  }

  match expected {
      "file" => {
        res = metadata(path).unwrap().is_file();
      },
      "directory" => {
        res = metadata(path).unwrap().is_dir();
      },
      _ => {},
  }

  // debug!("Path {} whether valid: {}", &path, &res);

  res.into()
}

#[tauri::command]
pub async fn show_mainwindow(window: tauri::Window) {
  // Show main window
  window.get_window("main").unwrap().show().unwrap();
}

#[tauri::command]
pub async fn close_splashscreen(window: tauri::Window) {
  // Close splashscreen
  if let Some(splashscreen) = window.get_window("splashscreen") {
    splashscreen.close().unwrap();
  }
  // Show main window
  window.get_window("main").unwrap().show().unwrap();
}

#[command]
pub fn get_drop_path_info(path: &str) -> Response<DropPathInfo> {
  let meta_type: String = if metadata(path).unwrap().is_file() {"file".to_string()} else {"directory".to_string()};
  let name: String = Path::new(path).file_name().unwrap().to_str().unwrap().to_string();
  let parent_path = Path::new(path).parent().unwrap();
  
  let save_name = name.clone() + "_BU";
  let save_path = parent_path.join(save_name).to_str().unwrap().to_string();

  Response { code: 200, data: DropPathInfo { path: path.to_string(), meta_type: meta_type, name: name, save_path: save_path }, msg: "".to_string() }
}

#[command]
pub fn is_program_initialized(state_handler: State<MissionHandlerWrapper>) -> bool {
  let current_status = state_handler.0.lock().unwrap().is_initialized;

  current_status
}

#[command]
pub fn is_password_set(state_handler: State<MissionHandlerWrapper>) -> bool {
  let current_password = state_handler.0.lock().unwrap().setting.password.clone();

  if current_password == "not set yet" {
    return false;
  } else {
    return true;
  }
}

#[command]
pub fn initialize_program_status(state_handler: State<MissionHandlerWrapper>) -> bool {
  state_handler.0.lock().unwrap().is_initialized = true;

  true
}

#[command]
pub fn initialize_data(state_handler: State<MissionHandlerWrapper>) -> APPData {
  let current_state = state_handler.0.lock().unwrap();

  let setting_data: Setting = current_state.setting.clone();
  let mission_list: Vec<Mission> = current_state.list.clone();

  debug!("Initialize setting data and mission list");

  APPData { setting: setting_data, list: mission_list }
}

#[command]
pub fn start_timing_save_data(state_handler: State<MissionHandlerWrapper>) {
  state_handler.0.lock().unwrap().start_timing_save_data();

  debug!("Start timing save data in 30 minute");
}

#[command]
pub fn timing_save_data(state_handler: State<MissionHandlerWrapper>) {
  let current_state = state_handler.0.lock().unwrap();

  let setting_data: Setting = current_state.setting.clone();
  let mission_list: Vec<Mission> = current_state.list.clone();
  
  let data = APPData { setting: setting_data, list: mission_list };

  save_data_to_file(data);
  
  debug!("Save data to file");
}

#[command]
pub fn update_list_info(list: Vec<Mission>, state_handler: State<MissionHandlerWrapper>) {
  let mut current_list = state_handler.0.lock().unwrap().list.clone();
  let mut info_map = HashMap::new();

  for item in list.iter() {
    info_map.insert(item.config.id.clone(), item.info.last_trigger_time.clone());
  }

  for item in current_list.iter_mut() {
    item.info.last_trigger_time = info_map.get(&item.config.id).unwrap().to_string();
  }

  state_handler.0.lock().unwrap().list = current_list;
}

#[command]
pub fn exit_program(state_handler: State<MissionHandlerWrapper>) {
  let current_state = state_handler.0.lock().unwrap();

  let setting_data: Setting = current_state.setting.clone();
  let mission_list: Vec<Mission> = current_state.list.clone();
  
  let data = APPData { setting: setting_data, list: mission_list };

  save_data_to_file(data);

  // block_on(state_handler.0.lock().unwrap().cron_handler.shutdown()).unwrap();

  process::exit(0)
}

#[command]
pub fn close_to_tray(app_handle: tauri::AppHandle) {
  app_handle.get_window("main").unwrap().hide().unwrap();
  debug!("Window close to tray");
}

#[command]
pub fn open_specific_folder_window(path: &str) {
  open_folder_window(path);
}

#[command]
pub fn change_setting_is_auto_start(auto_start: bool, state_handler: State<MissionHandlerWrapper>, start_handler: State<AutoStartHandlerWrapper>) -> bool {
  let mut _toggle_res: bool = false;
  if auto_start {
    _toggle_res = start_handler.0.lock().unwrap().enable_auto_start();
  } else {
    _toggle_res = start_handler.0.lock().unwrap().disable_auto_start();
  }

  if _toggle_res {
    state_handler.0.lock().unwrap().setting.is_auto_start = auto_start;    
    debug!("Change setting is_auto_start to {}", auto_start);
  } else {
    error!("Failed to change setting is_auto_start to {}", auto_start);
  }
  
  _toggle_res
}

#[command]
pub fn change_setting_is_light_theme(is_light: bool, state_handler: State<MissionHandlerWrapper>) -> bool {
  state_handler.0.lock().unwrap().setting.is_light_theme = is_light;
  debug!("Change setting is_light_theme to {}", is_light);
  true
}

#[command]
pub fn change_setting_is_close_to_tray(close_to_tray: bool, state_handler: State<MissionHandlerWrapper>) -> bool {
  state_handler.0.lock().unwrap().setting.is_close_to_tray = close_to_tray;
  debug!("Change setting is_close_to_tray to {}", close_to_tray);
  true
}

#[command]
pub fn change_window_state_by_system_tray(state_handler: State<MissionHandlerWrapper>, app_handle: tauri::AppHandle) -> bool {
  let window = app_handle.get_window("main").unwrap();
  let current_lang = state_handler.0.lock().unwrap().setting.language.clone();
  let window_visiable = state_handler.0.lock().unwrap().window_visiable.clone();
  
  let hide_item = app_handle.tray_handle().get_item("hide");
  let quit_item = app_handle.tray_handle().get_item("quit");
  
  let debug_visiable = window_visiable.clone();

  if &current_lang == &"zh-CN" {
    let hide_title = if !window_visiable {"最小化至托盘"} else {"显示主界面"};
    let quit_title = r"退出程序";
    
    hide_item.set_title(format!("{}", hide_title)).unwrap();
    quit_item.set_title(format!("{}", quit_title)).unwrap();
  } else if &current_lang == &"en-US" {
    let hide_title = if !window_visiable {"Close to tray"} else {"Show MainPage"};
    let quit_title = r"Quit Program";

    hide_item.set_title(format!("{}", hide_title)).unwrap();
    quit_item.set_title(format!("{}", quit_title)).unwrap();  
  }

  if window_visiable {
    window.hide().unwrap();
    state_handler.0.lock().unwrap().window_visiable = false;
  } else {
    window.show().unwrap();
    state_handler.0.lock().unwrap().window_visiable = true;
  }

  debug!("Change window visiable to {} by system tray", !debug_visiable);
  true
}

#[command]
pub fn change_setting_language(lang: &str, state_handler: State<MissionHandlerWrapper>, app_handle: tauri::AppHandle) -> bool {
  state_handler.0.lock().unwrap().setting.language = lang.to_string();
  
  let hide_item = app_handle.tray_handle().get_item("hide");
  let quit_item = app_handle.tray_handle().get_item("quit");

  let window = app_handle.get_window("main").unwrap();
  let window_visiable = state_handler.0.lock().unwrap().window_visiable.clone();

  if &lang == &"zh-CN" {
    let hide_title = if window_visiable {"最小化至托盘"} else {"显示主界面"};
    let quit_title = r"退出程序";
    let window_title = r"有备";
    
    hide_item.set_title(format!("{}", hide_title)).unwrap();
    quit_item.set_title(format!("{}", quit_title)).unwrap();

    window.set_title(format!("{}", window_title).as_str()).unwrap();
  } else if &lang == &"en-US" {
    let hide_title = if window_visiable {"Close to tray"} else {"Show MainPage"};
    let quit_title = r"Quit Program";
    let window_title = r"Mission Backup";

    hide_item.set_title(format!("{}", hide_title)).unwrap();
    quit_item.set_title(format!("{}", quit_title)).unwrap(); 
    
    window.set_title(format!("{}", window_title).as_str()).unwrap();
  }

  debug!("Change setting language to {}", lang);
  true
}

#[command]
pub fn change_setting_is_password_protect(password_protect: bool, state_handler: State<MissionHandlerWrapper>) -> bool {
  state_handler.0.lock().unwrap().setting.is_password_protected = password_protect;
  
  
  debug!("Change setting is_password_protect to {}", password_protect);
  true
}

#[command]
pub fn change_setting_password(old_password: &str, new_password: &str, state_handler: State<MissionHandlerWrapper>) -> bool {
  if state_handler.0.lock().unwrap().setting.password == old_password.to_string() {
    state_handler.0.lock().unwrap().setting.password = new_password.to_string();
    debug!("Change setting password from {} to {}", old_password, new_password);
    return true;
  } else {
    debug!("Incorrect old password {}", old_password);
    return false;
  }
}

#[command]
pub fn change_monitor_delay(delay: u16, state_handler: State<MissionHandlerWrapper>) -> bool {
  state_handler.0.lock().unwrap().setting.monitor_delay = delay;
  debug!("Change setting monitor delay to {}", &delay);

  true
}

#[command]
pub fn unlock(password: &str, state_handler: State<MissionHandlerWrapper>) -> bool {
  if state_handler.0.lock().unwrap().setting.password == password.to_string() {
    debug!("Detect unlock with correct password");
    return true;
  } else {
    warn!("Detect unlock with incorrect password");
    return false;
  }  
}

#[command]
pub fn create_mission(config: MissionConfig, state_handler: State<MissionHandlerWrapper>) -> Response<Mission> {
  let mut with_uuid_config = config.clone();
  with_uuid_config.id = Uuid::new_v4().to_string();

  let res = block_on(state_handler.0.lock().unwrap().add_job_to_handler(with_uuid_config));
  let mission = res.data.clone();

  if &res.code == &200 {
    info!("Create mission {} success", &res.data.config.name);    
    state_handler.0.lock().unwrap().list.push(mission);
  }

  res
}

#[command]
pub fn start_mission(id: String, state_handler: State<MissionHandlerWrapper>) -> Response<bool> {
  let mut response: Response<bool> = Response { code: 200, data: true, msg: "".to_string() };
  let mut _is_id_in_list: bool = false;
  let compare_id = id.clone();
  let mut current_list = state_handler.0.lock().unwrap().list.clone();

  for mission in &mut current_list {
    if mission.config.id == compare_id {
      // #[allow(unused_assignments)]
      _is_id_in_list = true;
      let start_config = mission.config.clone();

      let from_path = &start_config.from_path;
      let save_path = &start_config.to_path;
      if !PathBuf::from(from_path).exists() {
        mission.info.status = "unavailable".to_string();  
        response = Response { code: 400, data: false, msg: "Unavailable from path".to_string() }; 
        break;
      } else if !PathBuf::from(save_path).exists() {
        mission.info.status = "unavailable".to_string();  
        response = Response { code: 400, data: false, msg: "Unavailable save path".to_string() }; 
        break;
      }

      let start_res = block_on(state_handler.0.lock().unwrap().add_job_to_handler(start_config));
      if &start_res.code == &200 {
        info!("Start mission {} success", &start_res.data.config.name); 
        mission.info.status = if mission.config.cron_enable {"timing".to_string()} else {"monitoring".to_string()};  
        response = Response { code: 200, data: true, msg: "".to_string() }; 
        break;
      } else {
        error!("Failed to start mission, add to handler failed");
        response = Response {code: 500, data: false, msg: "add to handler failed".to_string()};
        break;
      }    
    }
  }

  if _is_id_in_list == false {
    error!("Failed to start mission, not found in list");
    response = Response {code: 500, data: false, msg: "not found".to_string()};
  }

  state_handler.0.lock().unwrap().list = current_list;
  
  response
}

#[command]
pub fn stop_mission(id: String, state_handler: State<MissionHandlerWrapper>) -> Response<bool> {
  let mut response: Response<bool> = Response { code: 200, data: true, msg: "".to_string() };
  let mut _is_id_in_list: bool = false;
  let compare_id = id.clone();
  let mut current_list = state_handler.0.lock().unwrap().list.clone();

  for mission in &mut current_list {
    if mission.config.id == compare_id {
      // #[allow(unused_assignments)]
      _is_id_in_list = true;
      let stop_config = mission.config.clone();

      let stop_res = block_on(state_handler.0.lock().unwrap().remove_job_from_handler(id));
      if &stop_res.code == &200 {
        info!("Stop mission {} success", &stop_config.name);  
        mission.info.status = "pausing".to_string(); 
        response = Response { code: 200, data: true, msg: "".to_string() }; 
        break;
      } else {
        error!("Failed to stop mission, remove from handler failed");
        response = Response {code: 500, data: false, msg: "remove from handler failed".to_string()};
        break;
      }    
    }
  }

  if _is_id_in_list == false {
    error!("Failed to stop mission, not found in list");
    response = Response {code: 500, data: false, msg: "not found".to_string()};
  }

  state_handler.0.lock().unwrap().list = current_list;
  
  response
}

#[command]
pub fn edit_mission(id: String, config: MissionConfig, state_handler: State<MissionHandlerWrapper>) -> Response<Mission>  {
  let list_used_id = id.clone();
  let remove_res = block_on(state_handler.0.lock().unwrap().remove_job_from_handler(id)); 
  if &remove_res.code == &200 {
    let add_res = block_on(state_handler.0.lock().unwrap().add_job_to_handler(config));
    let mission = add_res.data.clone();
    if &add_res.code == &200 {
      // update mission in list
      state_handler.0.lock().unwrap().list.retain(|item| -> bool {
        if *item.config.id == list_used_id {
          return false;
        } else {
          return true;
        }
      });

      info!("Edit mission {} success", &add_res.data.config.name);    
      state_handler.0.lock().unwrap().list.push(mission);
    }
    return add_res;
  } else {
      let failed_info = MissionInfo { status: "invalid".to_string(), last_trigger_time: "".to_string(), next_run_time: "".to_string()};
      return Response{ code: 500, data: Mission { config: config, info: failed_info }, msg: "".to_string()};
  }
}

#[command]
pub fn delete_mission(id: String, state_handler: State<MissionHandlerWrapper>) -> Response<bool> {
  let mission_id = id.clone();
  let res = block_on(state_handler.0.lock().unwrap().remove_job_from_handler(id)); 

  let mut mission_name = "".to_string();
  if &res.code == &200 {
    state_handler.0.lock().unwrap().list.retain(|item| -> bool {
          if *item.config.id == mission_id {
            mission_name = (*item.config.name).to_string();
            return false;
          } else {
            return true;
          }
      });

    info!("Delete mission {} success", mission_name);    
  } else if &res.code == &500 {
    info!("Delete mission {} failed", mission_id);
  }

  res
}

#[command]
pub fn force_delete_mission(id: String, state_handler: State<MissionHandlerWrapper>) -> Response<bool> {
  let mission_id = id.clone();

  let mut mission_name = "".to_string();
  state_handler.0.lock().unwrap().list.retain(|item| -> bool {
        if *item.config.id == mission_id {
          mission_name = (*item.config.name).to_string();
          return false;
        } else {
          return true;
        }
    });

  info!("Force delete mission {} success", mission_name);    

  Response { code: 200, data: true, msg: "".to_string() }
}

#[command]
pub fn get_mission_backups_status(id: String, date_type: String, start_datetime: String, size_unit: String, state_handler: State<MissionHandlerWrapper>) -> Response<SavePathInfo> {
  let current_list = state_handler.0.lock().unwrap().list.clone();
  let mut save_path: String = "".to_string();

  for mission in current_list {
    if mission.config.id == id {
      save_path = mission.config.to_path;
      break;
    }
  }

  if save_path == "".to_string() {
    return Response { code: 500, data: SavePathInfo { count: [].to_vec(), size: [].to_vec() }, msg: "".to_string() };
  }

  let query_res = count_size_with_path(&save_path, &date_type, &start_datetime, &size_unit);
  let count_res = query_res.get("count").unwrap().clone();
  let size_res= query_res.get("size").unwrap().clone();

  Response { code: 200, data: SavePathInfo { count: count_res, size: size_res }, msg: "".to_string() }
}

#[command]
pub fn open_url(url: String, app_handle: tauri::AppHandle) {
  let _ = tauri::scope::ShellScope::open(
    &app_handle.shell_scope(),
    &url,
    None,
  );
}

#[command]
pub fn change_setting_is_notify_when_create_backup_success(is_notify: bool, state_handler: State<MissionHandlerWrapper>) -> bool {
  state_handler.0.lock().unwrap().setting.is_notify_when_create_backup_success = is_notify;
  debug!("Change setting is_notify_when_create_backup_success to {}", is_notify);
  true
}

#[command]
pub fn change_setting_is_notify_when_create_backup_failed(is_notify: bool, state_handler: State<MissionHandlerWrapper>) -> bool {
  state_handler.0.lock().unwrap().setting.is_notify_when_create_backup_failed = is_notify;
  debug!("Change setting is_notify_when_create_backup_failed to {}", is_notify);
  true
}

