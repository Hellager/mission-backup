//! # State
//! 
//! `state` module contains all about state that managed by tauri.

use tokio::{runtime::Handle, sync::mpsc::Receiver, sync::Mutex};
use std::io::{Error, ErrorKind};
use crate::utils::logger::initialize_logger;
use log::{debug, error, info, warn};
use serde::{Serialize, Deserialize};
use crate::config::AppConfig;
use tauri::AppHandle;
use diesel::sqlite::SqliteConnection;
use tokio_cron_scheduler::JobScheduler;
use notify::{Error as NotifyError, ReadDirectoryChangesWatcher, RecursiveMode, Watcher};
use notify_debouncer_full::{
    new_debouncer, DebounceEventResult, DebouncedEvent, Debouncer, FileIdMap,
};
use std::time::Duration;

/// Status for handler services
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandlerStatus {
    /// Whether log service available
    pub log: bool,

    /// Whether app instance available
    pub app: bool,

    /// Whether config service available
    pub config: bool,

    /// Whether database service available
    pub database: bool,

    /// Whether cron service available
    pub cron: bool,

    /// Whether watcher service available
    pub watcher: bool,
}

impl Default for HandlerStatus {
    fn default() -> Self {
        HandlerStatus {
            log: false,
            app: false,
            config: false,
            database: false,
            cron: false,
            watcher: false
        }
    }
}

/// State struct for app mission
pub struct MissionHandler {
    /// Whether initialized
    pub is_set: bool,

    /// Service status
    pub status: HandlerStatus,

    /// App config
    pub config: AppConfig,

    /// Whether log service available
    pub log_handler: Option<String>,
    
    /// App handle for cur app instance
    pub app_handler: Option<AppHandle>,
        
    /// Connection to database
    pub db_handler: Option<SqliteConnection>,

    /// Cron handler for cron jobs
    pub cron_handler: Option<JobScheduler>,

    /// Watcher handler for monitor jobs
    pub watcher_handler: Option<Debouncer<ReadDirectoryChangesWatcher, FileIdMap>>,
    
    /// Receiver for watcher handler
    pub watcher_receiver: Option<Receiver<Result<Vec<DebouncedEvent>, Vec<NotifyError>>>>,
}

impl MissionHandler {
    /// Init app status with default value.
    /// 
    /// # Arguments
    /// 
    /// # Examples
    /// 
    /// ```
    /// use core::state::{MissionHandler, HandlerStatus, init_app_status};
    /// 
    /// let mut handler = MissionHandler {
    ///     is_set: false,
    ///     status: HandlerStatus::default(),
    ///     config: AppConfig::default(),
    ///     log_handler: None,
    ///     app_handler: None,
    ///     db_handler: None,
    /// };
    /// 
    /// if let Ok(()) = handler.init_app_status() {
    ///     println!("cur status: {:?}", handler.status);
    /// }
    /// ```
    fn init_app_status(&mut self) -> Result<(), std::io::Error> {
        self.status = HandlerStatus::default();
        Ok(())
    }

    /// Init app config from file, it config file not exists, use default config.
    /// 
    /// # Arguments
    /// 
    /// # Examples
    /// 
    /// ```
    /// use core::state::{MissionHandler, HandlerStatus, init_app_status};
    /// 
    /// let mut handler = MissionHandler {
    ///     is_set: false,
    ///     status: HandlerStatus::default(),
    ///     config: AppConfig::default(),
    ///     log_handler: None,
    ///     app_handler: None,
    ///     db_handler: None,
    /// };
    /// 
    /// if let Ok(()) = handler.init_app_config() {
    ///     println!("cur config: {:?}", handler.config);
    /// }
    /// ```
    fn init_app_config(&mut self) -> Result<(), std::io::Error> {
        use crate::config::{ load_app_config, save_app_config };

        match load_app_config() {
            Ok(app_config) => {
                self.config = app_config;
                self.status.config = true;
                debug!("initialize app config success");
                return Ok(());
            },
            Err(error) => {
                error!("failed to initialize app config, errMsg: {:?}", error);
                warn!("use default config");
                self.config = AppConfig::default();
                self.status.config = true;
                save_app_config(&self.config)?;
                return Ok(());
            }
        }   
    }

    /// Init app log handler.
    /// 
    /// # Arguments
    /// 
    /// # Examples
    /// 
    /// ```
    /// use core::state::{MissionHandler, HandlerStatus, init_app_status};
    /// 
    /// let mut handler = MissionHandler {
    ///     is_set: false,
    ///     status: HandlerStatus::default(),
    ///     config: AppConfig::default(),
    ///     log_handler: None,
    ///     app_handler: None,
    ///     db_handler: None,
    /// };
    /// 
    /// if let Ok(()) = handler.init_logger_handler() {
    ///     #[cfg(debug_assertions)] {
    ///         println!("cur log status: {:?}", handler.status.log);
    ///     }
    ///     
    ///     #[cfg(not(debug_assertions))] {
    ///         println!("cur log file: {}", handler.log_handler.unwrap());
    ///     }
    /// }
    /// ```
    fn init_logger_handler(&mut self) -> Result<(), std::io::Error> {
        if let None = self.log_handler {
            match initialize_logger(None, None) {
                Ok(log_file) => {
                    self.log_handler = Some(log_file.clone());
                    self.status.log = true;
                    debug!("initialize logger handler success, log at {}", log_file);
                    return Ok(());
                },
                Err(error) => {
                    self.log_handler = None;
                    self.status.log = false;
                    println!("failed to initialize logger handler, errMsg: {:?}", error);
                    return Err(error);
                }
            }            
        } else {
            warn!("logger handler already exists");
            self.status.log = true;
            return Ok(());
        }
    }

    /// Init app handler.
    /// 
    /// An app instance should be passed to the app_handler when handler initialize.
    /// 
    /// # Arguments
    /// 
    /// # Examples
    /// 
    /// ```
    /// use core::state::{MissionHandler, HandlerStatus, init_app_status};
    /// use log::error;
    /// 
    /// let mut handler = MissionHandler {
    ///     is_set: false,
    ///     status: HandlerStatus::default(),
    ///     config: AppConfig::default(),
    ///     log_handler: None,
    ///     app_handler: None,
    ///     db_handler: None,
    /// };
    /// 
    /// if let Ok(()) = handler.init_app_handler() {
    ///     println!("cur app status: {:?}", handler.status.app_handler);
    /// } else {
    ///     error!("app handler not exists!")
    /// }
    /// ```
    fn init_app_handler(&mut self) -> Result<(), std::io::Error> {
        if let None = self.app_handler {
            self.status.app = false;
            error!("failed to initialize app handler");
        } else {
            self.status.app = true;
            info!("app handler already exists");
        }

        Ok(())
    }

    /// Init database handler.
    /// 
    /// # Arguments
    /// 
    /// # Examples
    /// 
    /// ```
    /// use core::state::{MissionHandler, HandlerStatus, init_app_status};
    /// use log::error;
    /// 
    /// let mut handler = MissionHandler {
    ///     is_set: false,
    ///     status: HandlerStatus::default(),
    ///     config: AppConfig::default(),
    ///     log_handler: None,
    ///     app_handler: None,
    ///     db_handler: None,
    /// };
    /// 
    /// if let Ok(()) = handler.init_db_handler() {
    ///     println!("cur db status: {:?}", handler.status.db_handler);
    /// } else {
    ///     error!("db handler not exists!")
    /// }
    /// ```
    pub fn init_db_handler(&mut self) -> Result<(), std::io::Error> {
        use crate::db::{establish_sqlite_connection, init_database};

        if let None = self.db_handler {
            match establish_sqlite_connection() {
                Ok(conn) => {
                    self.db_handler = Some(conn);
                    self.status.database = true;

                    if let Some(mut dconn) = self.db_handler.as_mut() {
                        init_database(&mut dconn)?;
                    }
                    
                    debug!("initialize db handler success");
                    return Ok(());
                },
                Err(error) => {
                    self.status.database = false;
                    error!("failed to initialize db handler, errMsg: {:?}", error);
                    return Err(Error::from(ErrorKind::Other));
                }
            }            
        } else {
            self.status.database = true;
            warn!("database handler already success");
            return Ok(());         
        }
    }

    /// Init cron handler.
    /// 
    /// # Arguments
    /// 
    /// # Examples
    /// 
    /// ```
    /// use core::state::{MissionHandler, HandlerStatus, init_cron_handler};
    /// use log::error;
    /// 
    /// let mut handler = MissionHandler {
    ///     is_set: false,
    ///     status: HandlerStatus::default(),
    ///     config: AppConfig::default(),
    ///     log_handler: None,
    ///     app_handler: None,
    ///     db_handler: None,
    ///     cron_handler: None,
    /// };
    /// 
    /// if let Ok(()) = handler.init_cron_handler() {
    ///     println!("cur cron handler status: {:?}", handler.status.cron_handler);
    /// } else {
    ///     error!("failed to initialize cron handler!")
    /// }
    /// ```
    async fn init_cron_handler(&mut self) -> Result<(), std::io::Error> {
        if let None = self.cron_handler {
            if let Ok(handler) = JobScheduler::new().await {
                if let Err(error) = handler.start().await {
                    error!("Failed to initialize cron handler, errMsg: {:?}", error);
                    self.status.cron = false;
                    return Err(Error::from(ErrorKind::Other));
                } else {
                    self.status.cron = true;
                    self.cron_handler = Some(handler);
                    debug!("Initialize cron handler success");
                }
            }          
        } else {
            warn!("Cron handler already success");
            self.status.cron = true;
        }

        Ok(())
    }

    /// Init watcher handler.
    /// 
    /// # Arguments
    /// 
    /// # Examples
    /// 
    /// ```
    /// use core::state::{MissionHandler, HandlerStatus, init_watcher_handler};
    /// use log::error;
    /// 
    /// let mut handler = MissionHandler {
    ///     is_set: false,
    ///     status: HandlerStatus::default(),
    ///     config: AppConfig::default(),
    ///     log_handler: None,
    ///     app_handler: None,
    ///     db_handler: None,
    ///     cron_handler: None,
    ///     watcher_handler: None,
    /// };
    /// 
    /// if let Ok(()) = handler.init_watcher_handler() {
    ///     println!("cur watcher handler status: {:?}", handler.status.watcher_handler);
    /// } else {
    ///     error!("failed to initialize watcher handler!")
    /// }
    /// ```
    async fn init_watcher_handler(&mut self) -> Result<(), std::io::Error> {
        if let None = self.watcher_handler {
            let (tx, rx) = tokio::sync::mpsc::channel(1);
            let rt = Handle::current();

            let timeout = self.config.watcher.timeout;

            let debouncer = new_debouncer(
                Duration::from_secs(timeout),
                None,
                move |result: DebounceEventResult| {
                    let tx = tx.clone();

                    println!("calling by notify -> {:?}", &result);
                    rt.spawn(async move {
                        if let Err(e) = tx.send(result).await {
                            println!("Error sending event result: {:?}", e);
                        }
                    });
                },
            );

            match debouncer {
                Ok(watcher) => {
                    self.watcher_handler = Some(watcher);
                    self.watcher_receiver = Some(rx);
                    self.status.watcher = true;
                    debug!("Initialize notify handler success");
                    return Ok(());
                }
                Err(error) => {
                    self.status.watcher = false;
                    error!("Failed to initialize notify handler, errMsg: {:?}", error);

                    return Err(Error::from(ErrorKind::Other));
                }
            }                    
        }else {
            self.status.watcher = true;
            warn!("Notify handler already success");
            return Ok(());
        }
    }

    /// Initialize mission handler in sequence.
    /// 
    /// # Arguments
    /// 
    /// # Examples
    /// 
    /// ```
    /// use core::state::{MissionHandler, HandlerStatus, init_app_status};
    /// use log::error;
    /// 
    /// let mut handler = MissionHandler {
    ///     is_set: false,
    ///     status: HandlerStatus::default(),
    ///     config: AppConfig::default(),
    ///     log_handler: None,
    ///     app_handler: None,
    ///     db_handler: None,
    /// };
    /// 
    /// if let Ok(()) = handler.initialize() {
    ///     println!("init handler success: {:?}", handler);
    /// } else {
    ///     error!("init handler failed!");
    /// }
    /// ```
    pub async fn initialize(&mut self) -> Result<(), std::io::Error> {
        self.init_app_status()?;        
        self.init_logger_handler()?;
        self.init_app_config()?;
        self.init_app_handler()?;
        self.init_db_handler()?;
        self.init_cron_handler().await?;
        self.init_watcher_handler().await?;

        self.is_set = true;
        Ok(())
    }

    /// Shutdown mission handler in sequence.
    /// 
    /// # Arguments
    /// 
    /// # Examples
    /// 
    /// ```
    /// use core::state::{MissionHandler, HandlerStatus, init_app_status};
    /// use log::error;
    /// 
    /// let mut handler = MissionHandler {
    ///     is_set: false,
    ///     status: HandlerStatus::default(),
    ///     config: AppConfig::default(),
    ///     log_handler: None,
    ///     app_handler: None,
    ///     db_handler: None,
    /// };
    /// 
    /// if let Ok(()) = handler.shutdown() {
    ///     println!("shutdown handler success: {:?}", handler);
    /// } else {
    ///     error!("shutdown handler failed!");
    /// }
    /// ```
    pub fn shutdown(&mut self) -> Result<(), std::io::Error> {      
        use crate::config::save_app_config;

        save_app_config(&self.config)?;

        Ok(())
    }
}

/// MissionHandler state, will managed by tauri
pub struct MissionHandlerState(pub Mutex<MissionHandler>);
