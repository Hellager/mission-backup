// JSON serialize
use serde::{Serialize, Deserialize};

// Response Struct
#[derive(Clone, Serialize, Deserialize)]
pub struct DropPathInfo {
    pub path: String,
    pub meta_type: String,
    pub name: String,
    pub save_path: String
}

// Statistic Response Struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavePathInfo {
    pub count: Vec<f64>,
    pub size: Vec<f64>
}

// Global Setting Struct
#[derive(Clone, Serialize, Deserialize)]
pub struct Setting {
    pub is_auto_start: bool,
    pub is_light_theme: bool,
    pub is_password_protected: bool,
    pub password: String,
    pub is_close_to_tray: bool,
    pub language: String,
    pub monitor_delay: u16,
    pub is_notify_when_create_backup_success: bool,
    pub is_notify_when_create_backup_failed: bool,
    // is_webdav_enable: bool,
    // is_webdav_available: bool,
    // webdav_host_address: String,
    // webdav_username: String,
    // webdav_password: String,
    // is_samba_enable: bool,
    // is_samba_available: bool,
    // samba_host_address: String,
    // samba_uername: String,
    // samba_password: String,
    // is_ftp_enable: bool,
    // is_ftp_available: bool,
    // ftp_host_address: String,
    // ftp_username: String,
    // ftp_password: String,
    // pub is_update_available: bool,
    pub software_version: String,
}

// Mission Config Struct
#[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(rename_all = "PascalCase")]
pub struct MissionConfig {
    pub id: String,
    pub name: String,
    pub target_type: String,
    pub from_path: String,
    pub to_path: String,
    pub ignore_enable: bool,
    pub ignore_method: String,
    pub ignores: Vec<String>,
    pub compress_enable: bool,
    pub compress_format: String,
    pub cron_enable: bool,
    pub cron_expression: String,
    pub monitor_enable: bool,
    pub restrict_save_days_enable: bool,
    pub save_days: u64,
    pub restrict_save_size_enable: bool,
    pub save_size: u64,
    pub backup_method: String,
}

// Mission Info Struct
#[derive(Clone, Serialize, Deserialize)]
pub struct MissionInfo {
    pub status: String,
    pub next_run_time: String,
    pub last_trigger_time: String
}

impl MissionInfo {
    pub fn new(status: &str, next: &str, last: &str) -> MissionInfo {
        MissionInfo {
            status: status.to_string(),
            next_run_time: next.to_string(),
            last_trigger_time: last.to_string(),
        }        
    }
}

// Mission Struct
#[derive(Clone, Serialize, Deserialize)]
pub struct Mission {
    pub config: MissionConfig,
    pub info: MissionInfo,
}

// Program Data Struct
#[derive(Clone, Serialize, Deserialize)]
// #[serde(rename_all = "PascalCase")]
pub struct APPData {
    pub setting: Setting,
    pub list: Vec<Mission>
}

// Response Struct
#[derive(Clone, Serialize, Deserialize)]
pub struct Response<T> {
    pub code: i32,
    pub data: T,
    pub msg: String,
}

/**
 * code:
 *  1 - Create backup related
 */
impl<T> Response<T> {
    pub fn new(code: i32, data: T, msg: &str) -> Response<T> {
        Response {
            code,
            data,
            msg: msg.to_string(),
        }
    }

    // pub fn ok(data: T, msg: &str) -> Self {
    //     Self::new(200, data, msg)
    // }

    // pub fn err(data: T, msg: &str) -> Self {
    //     Self::new(300, data, msg)
    // }
}