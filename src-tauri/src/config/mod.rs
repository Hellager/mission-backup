use crate::{AppResult, error::AppError};
use notify::NotifyConfig;
use serde::{Deserialize, Serialize};
use system::SystemConfig;
use extra::ExtraConfig;

pub mod notify;
pub mod system;
pub mod extra;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub system: SystemConfig,
    pub notify: NotifyConfig,
    pub extra: ExtraConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            system: system::SystemConfig::default(),
            notify: notify::NotifyConfig::default(),
            extra: extra::ExtraConfig::default(),
        }
    }
}

// impl AppConfig {
//     pub fn update_system(&mut self, system: SystemConfig) -> AppResult<()> {
//         self.system = system;
//         save_app_config(self)
//     }

//     pub fn update_notify(&mut self, notify: NotifyConfig) -> AppResult<()> {
//         self.notify = notify;
//         save_app_config(self)
//     }

//     pub fn update_extra(&mut self, extra: ExtraConfig) -> AppResult<()> {
//         self.extra = extra;
//         save_app_config(self)
//     }
// }

fn get_config_file_path() -> AppResult<String> {
    use directories::ProjectDirs;
    use std::env::current_dir;
    use std::fs::create_dir_all;
    use std::path::PathBuf;

    let application = option_env!("CARGO_PKG_NAME").unwrap_or("mission-backup");
    let mut config_dir: PathBuf = current_dir().map_err(|_e| AppError::ConfigPathUnavailable)?;

    if let Some(proj_dirs) = ProjectDirs::from("", "", application) {
        config_dir = proj_dirs.config_dir().to_path_buf();
        create_dir_all(&config_dir).map_err(|_e| AppError::ConfigPathUnavailable)?;
    }

    let file_path = config_dir.join(format!("{}.toml", application));
    Ok(file_path.display().to_string())
}

pub fn load_app_config() -> AppResult<AppConfig> {
    use std::fs::read_to_string;
    use std::path::Path;

    let path = get_config_file_path()?;
    if Path::new(&path).exists() {
        let stored_config = read_to_string(&path)?;
        Ok(toml::from_str(&stored_config)?)
    } else {
        Ok(AppConfig::default())
    }
}

pub fn save_app_config(config: &AppConfig) -> AppResult<()> {
    use std::fs::File;
    use std::io::Write;

    let path = get_config_file_path()?;
    let mut toml_file = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;

    let toml = toml::to_string(config)?;
    toml_file
        .write_all(toml.as_bytes())
        .map_err(|e| AppError::ConfigSaveFailed(e.to_string()))?;

    Ok(())
}
