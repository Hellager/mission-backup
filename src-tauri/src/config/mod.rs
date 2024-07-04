//! # Config
//! 
//! `config` module contains all configuration about app.

pub mod notify;
pub mod screensaver;
pub mod system;
pub mod watcher;

use serde::{Serialize, Deserialize};
use notify::NotifyConfig;
use screensaver::ScreensaverConfig;
use system::SystemConfig;
use watcher::WatcherConfig;

/// Configuration for app
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// App system config, like `theme`, `language`...
    pub system: SystemConfig,

    /// App notify config, like `enable_notify`...
    pub notify: NotifyConfig,

    /// App watcher Config, like `timeout`...
    pub watcher: WatcherConfig,

    /// App screensaver config, like `enable`...
    pub screensaver: ScreensaverConfig,    
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            system: system::SystemConfig::default(),
            notify: notify::NotifyConfig::default(),
            watcher: watcher::WatcherConfig::default(),
            screensaver: screensaver::ScreensaverConfig::default(),
        }
    }
}

/// Get config file path in system.
/// This will not create config file, but will create the parent directory if not exists.
/// 
/// # Arguments
/// 
/// # Examples
/// 
/// ```
/// use config::get_config_file_path;
/// 
/// match get_config_file_path() {
///     Ok(path) => {
///         println!("the config file path is: {}", path);
///     },
///     Err(error) => {
///         println!("failed to get config file path, errMsg: {:?}", error);
///     }
/// }
/// ```
pub fn get_config_file_path() -> Result<String, std::io::Error> {
    use directories::ProjectDirs;
    use std::env::current_dir;
    use std::path::PathBuf;
    use std::fs::create_dir_all;


    // Where to get/how to generate config instance (to use with tauri::api::path::app_data_dir)?
    // https://github.com/tauri-apps/tauri/discussions/6583

    let application = option_env!("CARGO_PKG_NAME").unwrap_or("mission-backup");
    let mut config_dir: PathBuf = current_dir()?;
    if let Some(proj_dirs) = ProjectDirs::from(
        "dev",
        "",
        application
    ) {
        config_dir = proj_dirs.config_dir().to_path_buf();
        create_dir_all(config_dir.clone())?;
    }

    let file_path = config_dir.join(format!("{}.toml", application));
    return Ok(file_path.display().to_string());
}

/// Load app configuration.
/// If config file not exists or file incorrect, will return default app config.
/// 
/// # Arguments
/// 
/// # Examples
/// 
/// ```
/// use config::load_app_config;
/// 
/// match load_app_config() {
///     Ok(config) => {
///         println!("cur config: {:?}", config);
///     },
///     Err(error) => {
///         println!("failed to load config, errMsg: {:?}", error);
///     }
/// }
/// ```
pub fn load_app_config() -> Result<AppConfig, std::io::Error> {
    use std::path::Path;
    use std::fs::read_to_string;
    use std::io::{Error, ErrorKind};

    if let Ok(path) = get_config_file_path() {
        if Path::new(&path).exists() {
            let stored_config = read_to_string(path)?;
            match toml::from_str(stored_config.as_str()) {
                Ok(config) => {
                    return Ok(config);
                },
                Err(_error) => {
                    return Err(Error::from(ErrorKind::InvalidData));
                }
            }
        }
    }

    return Err(Error::from(ErrorKind::NotFound));
}

/// Save app configuration to config file.
/// 
/// # Arguments
/// 
/// * `config` - A configuration for app.
/// 
/// # Examples
/// 
/// ```
/// use config::{AppConfig, load_app_config};
/// 
/// let mut config: AppConfig = AppConfig::Default();
/// config.watcher.timeout = 6;
/// 
/// if let Ok(()) = save_app_config(&config) {
///     println!("save app config success");
///     let cur_config = load_app_config()?;
///     println!("cur config is: {:?}", cur_config);
/// }
/// ```
pub fn save_app_config(config: &AppConfig) -> Result<(), std::io::Error> {
    use std::io::Write;
    use std::fs::File;

    if let Ok(path) = get_config_file_path() {
        let mut toml_file = File::options().write(true).create(true).truncate(true).open(path)?;
        if let Ok(toml) = toml::to_string(config) {
            toml_file.write_all(toml.as_bytes())?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_app_config() {
        if let Ok(config) = load_app_config() {
            let config_file_path = get_config_file_path().unwrap();
            
            save_app_config(&config).unwrap();

            assert_eq!(std::path::Path::new(&config_file_path).exists(), true);

            std::fs::remove_file(config_file_path).unwrap();
        } else {
            panic!("failed to load app config");
        }
    }
}
