//! The `logger` module contains functions about log.

use serde::{Serialize, Deserialize};

#[allow(dead_code)]
/// Initialize simplelog.
/// 
/// # Arguments
/// 
/// * `term_enable` - whether enable terminal log output
/// * `file_enable` - whether enable file log output
/// * `path` - A string that holds the log file path
/// 
/// # Examples
/// 
/// ```
/// use logger::initialize_simplelog;
/// use log::info;
/// 
/// if let Err(error) = initialize_simplelog(true, true, Some("path\\to\\log\\file")) {
///     println!("Some error occurs when initialize simplelog, errMsg: {:?}", error);
/// } else {
///     info!("Initialize simplelog success!");
/// }
/// ```
pub fn initialize_simplelog(term_enable: bool, file_enable: bool, path: Option<&str>) -> Result<(), std::io::Error> {
    // https://github.com/Drakulix/simplelog.rs
    // https://github.com/Drakulix/simplelog.rs/issues/138

    // trace < debug < info < warn < error < off
    use simplelog::{Config, ConfigBuilder, SharedLogger, TermLogger, CombinedLogger, WriteLogger, LevelFilter, TerminalMode, ColorChoice};
    use chrono::Local;
    use time::UtcOffset;
    use std::io::{Error, ErrorKind};
    use std::fs::{File, remove_file};
    use std::env::current_dir;

    if !term_enable && !file_enable {
        return Err(Error::from(ErrorKind::InvalidData));
    }

    let mut config = Config::default();
    // use local time
    let offset_in_secs = Local::now().offset().local_minus_utc();
    if let Ok(utc_offset) = UtcOffset::from_whole_seconds(offset_in_secs) {
        config = ConfigBuilder::new().set_time_offset(utc_offset).build();
    }

    let mut _loggers: Vec<Box<dyn SharedLogger>> = vec![];
    let terminal_logger = TermLogger::new(LevelFilter::Debug, config.clone(), TerminalMode::Mixed, ColorChoice::Auto);
    
    let mut log_file_path = "".to_string();
    if file_enable {
        if let Some(file_path) = path {
            log_file_path = file_path.to_string();
        }
    } 
    
    if log_file_path == "" {
        log_file_path = current_dir()?.join("app.log").display().to_string();
    }

    let log_file = File::create(&log_file_path)?;
    let file_logger = WriteLogger::new(LevelFilter::Info, config, log_file);
    
    if term_enable && file_enable {
        _loggers = vec![terminal_logger, file_logger]; 
    } else if term_enable && !file_enable {
        _loggers = vec![terminal_logger]; 
        remove_file(&log_file_path)?;
    } else if !term_enable && file_enable {
        _loggers = vec![file_logger]; 
    }

    match CombinedLogger::init(_loggers) {
        Ok(()) => Ok(()),
        Err(error) => {
            println!("Some error occurs when initialize simplelog, errMsg: {:?}", error);
            return Err(Error::from(ErrorKind::AlreadyExists));
        }
    }
}

#[allow(dead_code)]
/// Initialize log4rs and log to rolling file.
/// If `path` is relative, it will log to `{std::env::current_dir}\\path`.
/// If `path` is absoulte, it will log to the exact path file.
/// 
/// # Arguments
/// 
/// * `path` - A string that holds the log file path
/// 
/// # Examples
/// 
/// ```
/// use logger::initialize_log4rs_to_rolling_size_file;
/// use log::info;
/// 
/// match initialize_log4rs_to_rolling_size_file("log/request.log") {
///     Ok(()) => {
///         info!("Initialize log4rs success!");
///     },
///     Err(error) => {
///         println!("Initialize log4rs failed, errMsg: {:?}", error);
///     }
/// }
/// ```
fn initialize_log4rs_to_rolling_size_file(path: &str) -> Result<(), std::io::Error> {
    use log4rs::{
        init_config,
        append::rolling_file::{RollingFileAppender, policy::compound::{
                roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger, CompoundPolicy,
            }},
        config::{Appender, Config, Root},
        encode::pattern::PatternEncoder,
    };
    use byte_unit::Byte;
    use log::LevelFilter;
    use std::io::{Error, ErrorKind};
    use std::path::Path;
    use std::fs::File;

    let log_pattern = "{d(%Y-%m-%d %H:%M:%S)} | {({l}):5.5} | {m}{n}";
    let roller_pattern = "logs/request_{}.log";

    if !Path::new(path).exists() {
        let _ = File::create(&path)?;
    }

    let mut trigger_size = 10 * 1000 * 1000;
    if let Ok(size) = Byte::parse_str("10 MB", true) {
        trigger_size = u64::from(size);
    }

    let trigger = SizeTrigger::new(trigger_size);
    if let Some(appender) = FixedWindowRoller::builder()
            .base(1)
            .build(roller_pattern, 7).ok()
            .and_then(|roller| {
                Some(CompoundPolicy::new(Box::new(trigger), Box::new(roller)))
            })
            .and_then(|policy| {
                RollingFileAppender::builder()
                    .encoder(Box::new(PatternEncoder::new(log_pattern)))
                    .build(path, Box::new(policy)).ok()
            })
    {
        match Config::builder()
                .appender(
                    Appender::builder()
                        .build("rollingfile", Box::new(appender)))
                .build(
                    Root::builder()
                            .appender("rollingfile")
                            .build(LevelFilter::Info)
                ) {
                    Ok(config) => {
                        if let Err(error) = init_config(config) {
                            println!("Initialize log4rs failed, errMsg: {:?}", error);
                            return Err(Error::from(ErrorKind::Other));
                        }
                    },
                    Err(error) => {
                        println!("Initialize log4rs failed, errMsg: {:?}", error);
                        return Err(Error::from(ErrorKind::Other));
                    }
                }
    } else {
        println!("Initialize log4rs failed");
        return Err(Error::from(ErrorKind::Other)); 
    }


    Ok(())
}

#[allow(dead_code)]
/// Initialize app logger.
/// 
/// By default, it will use `simplelog` as terminal logger when debug with no file logger and `log4rs` as file logger when release with no terminal logger.
/// 
/// If `name` has been configured, your log file will be stored at directory like {FOLDERID_RoamingAppData}/<name>/data/log.
/// 
/// If `file` has been configured, your log file will be stotred at the file path.
/// 
/// # Arguments
/// 
/// * `name` - An option that holds the log save directory name.
/// * `file` - An option that holds the log file save path.
/// 
/// # Examples
/// 
/// ```
/// use logger::initialize_logger;
/// use log::info;
/// 
/// if let Err(error) = initialize_logger(None, None)) {
///     println!("Some error occurs when initialize logger, errMsg: {:?}", error);
/// } else {
///     info!("Initialize logger success!");
/// }
/// ```
pub fn initialize_logger(name: Option<&str>, file: Option<&str>) -> Result<String, std::io::Error> {
    use directories::ProjectDirs;
    use std::fs::create_dir_all;
    use std::env::current_dir;
    use std::path::PathBuf;
    
    let mut file_path = file.unwrap_or("").to_string();
    if cfg!(debug_assertions) {
        initialize_simplelog(true, false, None)?;
    } else {
        let mut app_name = option_env!("CARGO_PKG_NAME").unwrap_or("mission-backup");
        if let Some(logger_name) = name {
            app_name = logger_name;
        }
            
        if file_path.is_empty() {
            let logger_dir: PathBuf;
            if let Some(proj_dirs) = ProjectDirs::from(
                "dev",
                "",
                app_name
            ) {
                logger_dir = proj_dirs.data_dir().join("log");
                create_dir_all(logger_dir.clone())?;
            } else {
                logger_dir = current_dir()?.join("log");
                create_dir_all(logger_dir.clone())?;
            }

            create_dir_all(logger_dir.clone())?;
            let log_path = logger_dir.join(format!("{}.log", app_name)); 
            file_path = log_path.display().to_string();
        }

        initialize_log4rs_to_rolling_size_file(file_path.as_str())?;
    }

    Ok(file_path)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogInfo {
    pub path: String,

    pub size: u64,
}

pub fn get_log_info(log_path: &str) -> Result<LogInfo, std::io::Error> {
    use std::path::Path;
    use fs_extra::dir::get_size;
    use std::io::{ Error, ErrorKind };

    let mut info = LogInfo {
        path: log_path.to_string(),
        size: 0,
    };

    if Path::new(log_path).exists() {
        match Path::new(log_path).parent() {
            Some(log_dir) => {
                if let Ok(dir_size) = get_size(log_dir) {
                    info.size = dir_size;
                }
                return Ok(info);
            },
            None => {
                println!("no log die detected!");
                return Err(Error::from(ErrorKind::NotFound));
            }
        }
    }

    Err(Error::from(ErrorKind::NotFound))    
}

pub fn clean_log(log_path: &str) -> Result<u64, std::io::Error> {
    use std::path::Path;
    use super::explorer::remove_all;
    use fs_extra::dir::get_size;
    use std::io::{ Error, ErrorKind };

    if Path::new(log_path).exists() {
        match Path::new(log_path).parent() {
            Some(log_dir) => {
                if let Ok(dir_size) = get_size(log_dir) {
                    remove_all(log_dir.to_str().expect(""))?;
                    return Ok(dir_size);
                }
            },
            None => {
                println!("no log die detected!");
                return Err(Error::from(ErrorKind::NotFound));
            }
        }
    }

    Err(Error::from(ErrorKind::NotFound))        
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn test_initialize_simplelog() {
        use std::env::current_dir;
        use std::fs::{create_dir_all, remove_dir_all};
        use std::path::Path;
        // use log::info;

        let test_log_path = current_dir().unwrap().join("test_simplelog");
        let _ = create_dir_all(test_log_path.clone());

        let log_file_path = test_log_path.join("app.log").display().to_string();
        let log_file = Some(log_file_path.as_str());
        if let Ok(()) = initialize_simplelog(true, true, log_file) {
            // info!("Hello world!");

            assert_eq!(Path::new(&log_file_path).exists(), true);

            let _ = remove_dir_all(current_dir().expect("").join("test_simplelog")).unwrap();
        }
    }

    #[test]
    #[ignore]
    fn test_initialize_log4rs_to_rolling_size_file() {
        use std::env::current_dir;
        use std::fs::{create_dir_all, remove_dir_all};
        use std::path::Path;
        use log::info;

        let test_log_path = current_dir().unwrap().join("test_log4rs_rolling_size");
        let _ = create_dir_all(test_log_path.clone());

        let log_file_path = test_log_path.join("request.log").display().to_string();
        if let Ok(()) = initialize_log4rs_to_rolling_size_file(log_file_path.as_str()) {
            info!("Hello world!");

            assert_eq!(Path::new(&log_file_path).exists(), true);

            let _ = remove_dir_all(current_dir().expect("").join("test_log4rs_rolling_size")).unwrap();
        }
    }

    #[test]
    fn test_initialize_logger() {
        use directories::ProjectDirs;
        use std::fs::{create_dir_all, remove_dir_all};
        use std::path::Path;

        if !cfg!(nodebug_assertions) {
            if let Some(proj_dirs) = ProjectDirs::from(
                "dev",
                "",
                env!("CARGO_PKG_NAME")
            ) {
                let data_dir = proj_dirs.data_dir();
                let logger_dir = data_dir.join("log");
                let log_file = logger_dir.clone().join(format!("{}.log", env!("CARGO_PKG_NAME")));

                create_dir_all(logger_dir.clone()).unwrap();
                if let Ok(log_path) = initialize_logger(None, log_file.to_str()) {
                    assert_eq!(Path::new(&logger_dir).exists(), true);
                    assert_eq!(Path::new(&log_path).exists(), true);                        
                }

                let _ = remove_dir_all(logger_dir).unwrap();
            }
        }
    }
}
