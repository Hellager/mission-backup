//! The `migration` module helps migration from incompatible app version.

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Setting {
    pub is_auto_start: bool,
    pub is_light_theme: bool,
    pub is_password_protected: bool,
    pub password: String,
    pub is_close_to_tray: bool,
    pub language: String,
    pub monitor_delay: u16,
    pub software_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MissionConfig {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MissionInfo {
    pub status: String,
    pub next_run_time: String,
    pub last_trigger_time: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Mission {
    pub config: MissionConfig,
    pub info: MissionInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct APPData {
    pub setting: Setting,
    pub list: Vec<Mission>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigratedData {
    pub config: crate::config::AppConfig,
    pub ignores: Vec<crate::db::ignore::Ignore>,
    pub procedures: Vec<crate::db::procedure::Procedure>,
    pub missions: Vec<crate::db::mission::Mission>,
    pub backups: Vec<crate::db::backup::Backup>
}

fn migrate_config_data(raw: &Setting) -> Result<crate::config::AppConfig, std::io::Error> {
    use crate::config::AppConfig;

    let mut migrated_config = AppConfig::default();

    // Migrate system cnofig
    migrated_config.system.auto_start = raw.is_auto_start;
    migrated_config.system.theme = if raw.is_light_theme { "light".to_string() } else { "dark".to_string() };
    migrated_config.system.close_option = if raw.is_close_to_tray { 1 } else { 0 };
    migrated_config.system.language = raw.language.clone();

    // migrate screensaver config
    migrated_config.screensaver.enable = raw.is_password_protected;
    migrated_config.screensaver.password = raw.password.clone();

    // migrate watcher config
    migrated_config.watcher.timeout = raw.monitor_delay as u64;

    Ok(migrated_config)
}

fn extract_procedure(src: &MissionConfig) -> Result<crate::db::procedure::Procedure, std::io::Error> {
    let mut data = crate::db::procedure::Procedure::default();

    data.name = format!("{}_procedure", src.name);
    data.has_ignores = src.ignore_enable;
    data.is_compress = src.compress_enable;
    data.cron_expression = src.cron_expression.clone();

    match src.ignore_method.as_str() {
        "custom" => {
            data.ignore_method = 1;
        },
        ".gitignore" => {
            data.ignore_method = 2;
        },
        _ => {
            data.ignore_method = 0;
        }
    }

    match src.compress_format.as_str() {
        "zip" => {
            data.compress_format = 1;
        },
        "tar" => {
            data.compress_format = 0;
        },
        "tar.gz" => {
            data.compress_format = 2;
        },
        "tar.bz2" => {
            data.compress_format = 3;
        },
        "tar.xz" => {
            data.compress_format = 4;
        },
        _ => {
            data.compress_format = 0;
        }
    }

    if src.cron_enable {
        data.trigger = 1;
    } else {
        data.trigger = 2;
    }

    if src.restrict_save_days_enable && src.restrict_save_size_enable {
        data.restrict = 3;
    } else if src.restrict_save_days_enable {
        data.restrict = 1;
    } else if src.restrict_save_size_enable {
        data.restrict = 2;
    } else {
        data.restrict = 0;
    }

    data.restrict_days = src.save_days as i16;
    data.restrict_size = src.save_size as i64;

    Ok(data)
}

fn extract_ignores(src: &MissionConfig, procedure: &crate::db::procedure::Procedure) -> Result<Vec<crate::db::ignore::Ignore>, std::io::Error> {
    let mut data = Vec::new();

    match src.ignore_method.as_str() {
        "custom" => {
            for item in src.ignores.iter() {
                let mut ignore = crate::db::ignore::Ignore::default();
                ignore.procedure_id = procedure.procedure_id.clone();
                ignore.keyword = item.clone();
                data.push(ignore);
            }
        },
        ".gitignore" => {
            return Ok(data);
        },
        _ => {
            return Ok(data);
        }
    }

    Ok(data)
}

fn extract_mission(src: &MissionConfig, procedure: &crate::db::procedure::Procedure) -> Result<crate::db::mission::Mission, std::io::Error> {
    let mut data = crate::db::mission::Mission::default();

    data.procedure_id = procedure.procedure_id.clone();
    data.name = src.name.clone();
    data.status = 0;
    
    match src.target_type.as_str() {
        "file" => {
            data.path_type = 1;
        },
        "directory" => {
            data.path_type = 2;
        },
        _ => {
            data.path_type = 0;
        }
    }

    data.src_path = src.from_path.clone();
    data.dst_path = src.to_path.clone();

    Ok(data)
}

fn extract_backups(src: &MissionConfig, mission: &crate::db::mission::Mission) -> Result<Vec<crate::db::backup::Backup>, std::io::Error> {
    use walkdir::WalkDir;
    use log::error;
    use std::io::{ Error, ErrorKind };
    use fs_extra::dir::get_size;
    
    let mut data = Vec::new();

    let save_path = src.to_path.clone();

    for entries in WalkDir::new(save_path.clone()).min_depth(2).max_depth(2) {
        match entries {
            Ok(item) => {
                let mut backup = crate::db::backup::Backup::default();

                backup.mission_id = mission.mission_id.clone();
                backup.save_path = item.path().display().to_string().clone();
                if let Ok(size) = get_size(backup.save_path.clone()) {
                    backup.backup_size = size as i64;
                }

                data.push(backup);
            },
            Err(err) => {
                error!("failed to walk dir {:?}, errMsg: {:?}", save_path, err);
                return Err(Error::from(ErrorKind::Interrupted));
            }
        }
    }

    Ok(data)
}

fn migrate_file_data(raw: &Vec<Mission>) -> Result<MigratedData, std::io::Error> {
    use crate::config::AppConfig;
    
    let mut data = MigratedData {
        config: AppConfig::default(),
        ignores: Vec::new(),
        procedures: Vec::new(),
        missions: Vec::new(),
        backups: Vec::new()
    };

    for item in raw.iter() {
        let procedure = extract_procedure(&item.config)?;
        let mut ignores = extract_ignores(&item.config, &procedure)?;
        let mission = extract_mission(&item.config, &procedure)?;
        let mut backups = extract_backups(&item.config, &mission)?;

        data.ignores.append(&mut ignores);
        data.procedures.push(procedure);
        data.missions.push(mission);
        data.backups.append(&mut backups);
    }

    Ok(data)
}

pub fn parse_data_file(path: &str) -> Result<MigratedData, std::io::Error> {
    use std::fs::read_to_string;
    use super::crypto::decode_base64;
    use std::io::{ Error, ErrorKind };

    // let data_file = OpenOptions::new().read(true).open(path)?;
    let encoded_data = read_to_string(path)?;
    if let Ok(mut decoded_data) = decode_base64(&encoded_data, None).as_mut() {
        let app_data = serde_json::from_str::<APPData>(&mut decoded_data)?;

        let migrated_config = migrate_config_data(&app_data.setting)?;
        let mut migrated_data = migrate_file_data(&app_data.list)?;

        // println!("migrated config: {:?}", migrated_config);
        // println!("migrated data: {:?}", migrated_data);
        
        migrated_data.config = migrated_config;

        return Ok(migrated_data)
    }

    Err(Error::from(ErrorKind::InvalidData))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_migrate_from_old_app() {
        let file_path = "D:\\TEMP\\program\\mission-backup-v1.0.0\\config.dat";
        match parse_data_file(file_path) {
            Ok(_res) => {
                println!("success");
            },
            Err(error) => {
                println!("failed: {:?}", error);
            }
        }
    }
}
