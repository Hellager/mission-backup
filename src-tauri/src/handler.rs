// Handler related
use hotwatch::{ Hotwatch, Event as WatchEvent };
use tauri::async_runtime::block_on;
use tokio_cron_scheduler::{ Job, JobScheduler };
use auto_launch::AutoLaunch;

// System related
use tauri::{AppHandle, Manager};
use std::sync::Mutex;

// Time related
use cron::Schedule as Cron_Schedule;
use chrono::{Utc, Local, DateTime};

// General
use log::{ info, debug, warn, error };
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::fs::create_dir_all;
use std::io::ErrorKind;
use std::str::FromStr;
use uuid::Uuid;

// Module related
use crate::modules::explorer::{remove_target, copy_target_with_no_ignores, copy_target_with_ignores, restrict_save_path_day_count, restrict_save_path_size};
use crate::data_types::{ Setting, Mission, MissionConfig, Response, MissionInfo };
use crate::modules::compressor::create_archive;



pub async fn initialize_cron_scheduler() -> JobScheduler {
    let cron_handler = JobScheduler::new().await.expect("Failed to initialize tokio cron handler");

    cron_handler.start().await.expect("Failed to start tokio cron handler");

    cron_handler.clone()
}

/************************* initialize handlers *************************/
pub struct MissionHandler {
    pub list: Vec<Mission>,
    pub setting: Setting,
    pub is_initialized: bool,
    pub cron_handler: JobScheduler,
    pub monitor_handler: Hotwatch,
    pub app_handler: AppHandle,
    pub window_visiable: bool,
    pub cron_job_hashmap: HashMap<String, Uuid>,
    pub working_mission_list: HashMap<String, bool>,
    pub dir_increment: HashMap<String, Vec<String>>
}

impl MissionHandler {
    pub async fn add_job_to_handler(&mut self, config: MissionConfig) -> Response<Mission> {
        let mission_config = config.clone();
        let mission_name = mission_config.name.clone();
        let _working_id = mission_config.id.clone();
        let response_config = mission_config.clone();
        let mut cron_next_run_time = "".to_string();

        let callback_config = mission_config.clone();
        let callback_app_handle = self.app_handler.clone();

        // let is_in_working_list: bool = self.working_mission_list.contains_key(&mission_config.id);
        // let is_already_working: bool = *self.working_mission_list.get(&mission_config.id).unwrap();
        // if is_in_working_list && !is_already_working {
            let parent_path: String = String::from(Path::new(&mission_config.from_path).parent().unwrap().to_str().unwrap());
            let watch_path: String = if mission_config.target_type == "file" {parent_path} else {mission_config.from_path};
            if mission_config.monitor_enable == true {
                self.monitor_handler.watch(watch_path, move |event: WatchEvent| {
                    let mut event_valid: bool = false;
                    if let WatchEvent::Create(ref event_path) = event {
                        debug!("detect {} create event", event_path.display());
                        event_valid = true
                    }
            
                    if let WatchEvent::Write(ref event_path) = event {
                        debug!("detect {} write event", event_path.display());
                        event_valid = true;
                    }
            
                    if let WatchEvent::Remove(ref event_path) = event {
                        debug!("detect {} remove event", event_path.display());
                        event_valid = true;
                    }
            
                    if let WatchEvent::Rename(ref origin_path, ref _current_path) = event {
                        debug!("detect {} rename event", origin_path.display());
                        event_valid = true;
                    }
            
                    if let WatchEvent::Chmod(ref event_path) = event {
                        debug!("detect {} chmod event", event_path.display());
                        event_valid = true;
                    }

                    if event_valid == true {
                        MissionHandler::update_mission_info(&callback_config, &callback_app_handle);
                        let create_res = MissionHandler::create_backups(&callback_config);
                        MissionHandler::restrict_save_path(&callback_config);
                    
                        if !create_res {
                            error!("Failed to create backups for {}", &callback_config.from_path);
                            let _ = &callback_app_handle.emit_all("error", Response::<MissionConfig> {
                                code: 1,
                                data: callback_config.clone(),
                                msg: "".to_string()
                            }).unwrap();

                            MissionHandler::update_mission_status("unavailable",&callback_config, &callback_app_handle);                    
                        } else {
                            let _ = &callback_app_handle.emit_all("success", Response::<MissionConfig> {
                                code: 1,
                                data: callback_config.clone(),
                                msg: "".to_string()
                            }).unwrap();
                            
                            MissionHandler::update_mission_status("monitoring",&callback_config, &callback_app_handle);                    
                        }
                    }
                }).unwrap();            
            } else if mission_config.cron_enable == true {
                let expression = mission_config.cron_expression.as_str();
                let mission_job = Job::new(expression, move |_uuid, _l| {
                    MissionHandler::update_mission_info(&callback_config, &callback_app_handle);
                    let create_res = MissionHandler::create_backups(&callback_config);
                    MissionHandler::restrict_save_path(&callback_config);
                    if !create_res {
                        error!("Failed to create backups for {}", &callback_config.from_path);
                        let _ = &callback_app_handle.emit_all("error", Response::<MissionConfig> {
                            code: 1,
                            data: callback_config.clone(),
                            msg: "".to_string()
                        }).unwrap();

                        MissionHandler::update_mission_status("unavailable",&callback_config, &callback_app_handle);                    
                    } else {
                        let _ = &callback_app_handle.emit_all("success", Response::<MissionConfig> {
                            code: 1,
                            data: callback_config.clone(),
                            msg: "".to_string()
                        }).unwrap();

                        MissionHandler::update_mission_status("timing",&callback_config, &callback_app_handle);                    
                    }
                }).unwrap();
                let job_id = self.cron_handler.add(mission_job).await.unwrap();
                self.cron_job_hashmap.insert(mission_config.id, job_id);

                let next_tick = self.cron_handler.next_tick_for_job(job_id).await;
                match next_tick {
                    Ok(Some(next_time)) => {
                        // cron_next_run_time = next_time.to_string();
                        let local_run_time: DateTime<Local> = DateTime::from(next_time);
                        cron_next_run_time = local_run_time.to_string();
                    },
                    _ => {
                        warn!("Could not get next tick for job {}", &mission_name);
                    }
                }
                // cron_next_run_time = self.cron_handler.next_tick_for_job(job_id).await.unwrap().to_string();
                debug!("Get job {} next run time at initialize {}", mission_name, &cron_next_run_time);
            }            
        // }   else {
        //     warn!("Mission {} is running, cannot run twice", &mission_config.id);
        // }
        

        // self.working_mission_list.insert(working_id, true);

        let mission_status = if mission_config.cron_enable {"timing"} else {"monitoring"};
        let mission_info = MissionInfo::new(mission_status, &cron_next_run_time , "");

        let response_mission = Mission{ config: response_config, info: mission_info };

        debug!("Add job {} to handler", mission_name);
        if mission_config.cron_enable {
            self.app_handler.emit_all("cron_time_update", response_mission.clone()).unwrap();
        }

        Response::new(200, response_mission, "")
    }

    pub async fn remove_job_from_handler(&mut self, id: String) -> Response<bool> {
        let mut is_success: bool = true;
        let mut is_id_in_list: bool = false;
        let mut mission_name: String = "".to_string();

        // let is_in_working_list: bool = self.working_mission_list.contains_key(&id);
        // let is_already_working: bool = *self.working_mission_list.get(&id).unwrap();
        // if is_in_working_list && !is_already_working {
            for job in &self.list {
                if *job.config.id == id {
                    is_id_in_list = true;
                    mission_name = (*job.config.name).to_string();
    
                    if job.config.cron_enable == true {
                        if self.cron_job_hashmap.contains_key(&id) {
                            let cron_job_id = self.cron_job_hashmap.get(&id).unwrap();
                            if let Err(cron_error) = self.cron_handler.remove(cron_job_id).await {
                                is_success = false;
                                error!("Failed to delete mission {}, errCode: {:?}", &mission_name, cron_error);
                            }
                            self.cron_job_hashmap.remove(&id).unwrap();                            
                        }
                    }
    
                    if job.config.monitor_enable == true {
                        let unwatch_path = &job.config.from_path;
                        if let Err(watch_error) = self.monitor_handler.unwatch(unwatch_path) {
                            is_success = false;
                            error!("Failed to delete mission {}, errCode: {:?}", &mission_name, watch_error);
                        }
                    }
                }       
            }            
        // } else {
        //     warn!("Mission {} is not running, no need to remove", &id);
        // }


        if is_id_in_list == false {
            warn!("No matched job in list, id: {}", id);
        } else {
            // self.working_mission_list.insert(id, false);
            debug!("Remove job {} from handler", mission_name);            
        }


        let response_code = if is_success {200} else {500};
        Response::new(response_code, is_success, "")
    }

    fn update_mission_status(status: &str, config: &MissionConfig, app_handle: &AppHandle) {
        let mission_name = &config.name;
        debug!("Update {} current status", mission_name);        
        
        let mission_config = config.clone();
        let mission_info = MissionInfo { status: status.to_string(), last_trigger_time: "".to_string(), next_run_time: "".to_string() };
        
        app_handle.emit_all("status_update", Mission { config: mission_config, info: mission_info}).unwrap();
    }

    fn update_mission_info(config: &MissionConfig, app_handle: &AppHandle) {
        let mission_name = &config.name;
        debug!("Update {} current info", mission_name);
        
        let new_status = "executing".to_string();
        let last_trigger_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        
        let mut next_run_time = "".to_string();
        if config.cron_enable {
            let cron_mission = Cron_Schedule::from_str(&config.cron_expression).unwrap();
            let next_run_time_vec: Vec<DateTime<Utc>> = cron_mission.upcoming(Utc).take(1)
                        .filter_map(|datetime| Some(datetime)).collect::<Vec<DateTime<Utc>>>();
            let local_run_time: DateTime<Local> = DateTime::from(next_run_time_vec[0]);
            next_run_time = local_run_time.to_string();            
        }

        let mission_config = config.clone();
        let mission_info = MissionInfo { status: new_status, last_trigger_time: last_trigger_time, next_run_time: next_run_time };
        app_handle.emit_all("info_update", Mission { config: mission_config, info: mission_info}).unwrap();
    }

    fn create_backups(config: &MissionConfig) -> bool {
        let current_datetime = Local::now().format("%Y-%m-%d").to_string();
        let current_hourtime = Local::now().format("%H-%M").to_string();
        let parent_datetime_path = PathBuf::from(config.to_path.as_str()).join(current_datetime);
        let actual_save_path = parent_datetime_path.join(current_hourtime);
        
        if !Path::new(&actual_save_path.to_str().unwrap()).exists() {
            if let Err(e) = create_dir_all(&actual_save_path) {
                match e.kind() {
                    ErrorKind::AlreadyExists => {}
                    _ => {
                        error!("Failed to create folder {}, errMsg: {:?}", &actual_save_path.to_str().unwrap(), e);
                        return false;
                    }
                }
            }            
        }

        // copy source file/dir to save path, will be named as hh-mm
        #[allow(unused_assignments)]
        let mut copy_res: bool = false;
        if config.ignore_enable {
            // when use .gitignore method, the folder should have .git dir, otherwise it will copy all items;
            let is_gitignore = if config.ignore_method == ".gitignore" {true} else {false};
            copy_res = copy_target_with_ignores(config.from_path.as_str(), actual_save_path.to_str().unwrap(), is_gitignore, &Vec::new());
        } else {
            copy_res = copy_target_with_no_ignores(config.from_path.as_str(), actual_save_path.to_str().unwrap());
        }

        if !copy_res {
            error!("Failed to copy source {}", &config.from_path);
            return copy_res;
        }

        // whether compress
        if config.compress_enable {
            let compressed_path: String = String::from(actual_save_path.to_str().unwrap()) + "." + &config.compress_format;

            let compress_res: bool = create_archive(actual_save_path.to_str().unwrap(), compressed_path.as_str(), &config.compress_format);
        
            // delete uncompress folder/file after compress
            let remove_res = remove_target(actual_save_path.to_str().unwrap());
            if !remove_res {
                warn!("Failed to source file/target {}", actual_save_path.to_str().unwrap());
            }

            if !compress_res {
                return false;
            }
        }

        // app_handle.emit_all("info_update", MissionInfo{ status: "executing".to_string(), })
        info!("Create backups for {}", config.name);

        true
    }

    fn restrict_save_path(config: &MissionConfig) {
        if config.restrict_save_days_enable {
            restrict_save_path_day_count(config.to_path.as_str(), config.save_days);
        }

        if config.restrict_save_size_enable {
            let convert_size = byte_unit::n_mib_bytes!(config.save_size, u64);
            restrict_save_path_size(config.to_path.as_str(), convert_size as u64);
        }
    }

    pub fn start_timing_save_data(&mut self) {
        let callback_app_handle = self.app_handler.clone();

        let expression = "0 0/30 * * * ?";
        let mission_job = Job::new(expression, move |_uuid, _l| {
            let _ = &callback_app_handle.emit_all("save_data", {});

            debug!("Request save data");
        }).unwrap();

        block_on(self.cron_handler.add(mission_job)).unwrap();
    }
}

pub struct AutoStartHandler {
    pub handler: AutoLaunch,
}

impl AutoStartHandler {
    pub fn enable_auto_start(&self) -> bool {
        if let Err(e) = self.handler.enable() {
            error!("Failed to enable auto start, errMsg: {:?}", e);
            return false;
        } 

        true
    }

    pub fn disable_auto_start(&self) -> bool {
        if let Err(e) = self.handler.disable() {
            error!("Failed to disable auto start, errMsg: {:?}", e);
            return false;
        } 

        true
    }
}

pub struct MissionHandlerWrapper(pub Mutex<MissionHandler>);
pub struct AutoStartHandlerWrapper(pub Mutex<AutoStartHandler>);
