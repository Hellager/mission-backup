pub mod ignore;
pub mod mission;
pub mod backup;
pub mod procedure;
pub mod schema;
pub mod utils;

use self::backup::Backup;
use self::ignore::Ignore;
use self::mission::Mission;
use self::procedure::Procedure;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use serde::{Serialize, Deserialize};

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

/// Struct Record
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Record {
    /// Backup record
    pub backup: Backup,

    /// Ignore record
    pub ignore: Ignore,

    /// Mission record
    pub mission: Mission,

    /// Procedure record
    pub procedure: Procedure,
}

impl Default for Record {
    fn default() -> Self {
        Record {
            backup: Backup::default(),
            ignore: Ignore::default(),
            mission: Mission::default(),
            procedure: Procedure::default(),
        }
    }
}

/// Establish connection to sqlite.
/// 
/// Will use env variable `DATABASE_URL` as default database path.
/// 
/// If `DATABASE_URL` not exists: 
/// In debug mode, will create a new database at app home directory.
/// In release mode, will create a new database at app project data directory.
/// 
/// # Arguments
/// 
/// # Examples
/// 
/// ```
/// use db::establish_sqlite_connection;
/// 
/// if let Ok(mut conn) = establish_sqlite_connection() {
///     println!("connect to sqlite success");
/// }
/// ```
pub fn establish_sqlite_connection() -> Result<SqliteConnection, diesel::result::ConnectionError> {
    use std::env;
    use dotenvy::dotenv;
    use std::path::Path;
    use log::error;
    use crate::utils::explorer::create_all;
    #[cfg(debug_assertions)]
    use crate::utils::common::get_app_home_dir;
    #[cfg(not(debug_assertions))]
    use directories::ProjectDirs;

    dotenv().ok();

    let mut database_url = env::var("DATABASE_URL").unwrap_or("".to_string());
    if database_url.is_empty() {
        #[cfg(not(debug_assertions))]
        {
            let application = option_env!("CARGO_PKG_NAME").unwrap_or("mission-backup");
            if let Some(proj_dirs) = ProjectDirs::from(
                "dev",
                "",
                application
            ) {
                database_url = proj_dirs.data_dir().join("database").join("mission_backup.db3").display().to_string();
            }            
        }
        
        #[cfg(debug_assertions)]
        if let Ok(app_dir) = get_app_home_dir() {
            database_url = app_dir.join("debug_db.db3").display().to_string();
        }
    }

    if !Path::new(&database_url).exists() {
        if let Err(error) = create_all(&database_url, "file") {
            error!("Failed to create database, errMsg: {error}");
        }
    }

    SqliteConnection::establish(&database_url)
}

/// Initialize database when release.
/// 
/// # Arguments
/// 
/// * `conn` - Connection to database.
/// 
/// # Examples
/// 
/// ```
/// use db::{establish_sqlite_connection, init_database};
/// 
/// if let Ok(mut conn) = establish_sqlite_connection() {
///     match init_database(&mut conn) {
///         Ok(_) => {
///             println!("initialize database success");
///         },
///         Err(error) => {
///             println!("failed to initialize database, errMsg: {:?}", error);
///         }
///     }   
/// }
/// ```
pub fn init_database(conn: &mut SqliteConnection) -> Result<(), std::io::Error> {
    use std::io::{ Error, ErrorKind };
    use log::error;

    match conn.run_pending_migrations(MIGRATIONS) {
        Ok(_) => {
            return Ok(())
        },
        Err(error) => {
            error!("Failed to migration database, errMsg: {:?}", error);
            return Err(Error::from(ErrorKind::Other));
        }
    }
}

/// Create record and insert into database.
/// 
/// # Arguments
/// 
/// * `table` - Which table to handle.
/// * `data` - Data that includes record about table.
/// * `conn` - Connection to database.
/// 
/// # Examples
/// 
/// ```
/// use db::{establish_sqlite_connection, create_db_record};
/// 
/// if let Ok(mut conn) = establish_sqlite_connection() {
///     let mut record = Record::default();
///     match create_db_record("mission", &mut record, &mut conn) {
///         Ok(record) => {
///             println!("create record: {:?}", record);
///         },
///         Err(error) => {
///             println!("failed to create record, errMsg: {:?}", error);
///         }
///     }   
/// }
/// ```
pub fn create_db_record(table: &str, data: &mut Record, conn: &mut SqliteConnection) -> Result<Record, diesel::result::Error> {
    use diesel::result::Error;
    
    match table {
        "ignore" => {
            ignore::create_ignore_record(conn, &mut data.ignore, &data.procedure)?;
        },
        "procedure" => {
            procedure::create_procedure_record(conn, &mut data.procedure)?;     
        },
        "mission" => {
            mission::create_mission_record(conn, &mut data.mission)?;
        },
        "backup" => {
            backup::create_backup_record(conn, &mut data.backup, &data.mission)?;
        },
        _ => {
            return Err(Error::from(Error::NotFound));
        }
    }

    Ok(data.clone())
}

/// Update records in database.
/// 
/// # Arguments
/// 
/// * `table` - Which table to handle.
/// * `data` - Data that includes record about table.
/// * `conn` - Connection to database.
/// 
/// # Examples
/// 
/// ```
/// use db::{establish_sqlite_connection, create_db_record};
/// 
/// if let Ok(mut conn) = update_db_record() {
///     let mut record = Record::default();
///     match update_db_record("mission", record, &mut conn) {
///         Ok(records) => {
///             println!("update record: {:?}", records);
///         },
///         Err(error) => {
///             println!("failed to update record, errMsg: {:?}", error);
///         }
///     }   
/// }
/// ```
pub fn update_db_record(table: &str, data: &mut Record, conn: &mut SqliteConnection) -> Result<Record, diesel::result::Error> {
    use diesel::result::Error;
    
    match table {
        "ignore" => {
            ignore::update_ignore_record(conn, &mut data.ignore)?; 
        },
        "procedure" => {
            procedure::update_procedure_record(conn, &mut data.procedure)?;       
        },
        "mission" => {
            mission::update_mission_record(conn, &mut data.mission)?;
        },
        "backup" => {
            // backup::update_backup_record(conn, &mut data.backup)?;
        },
        _ => {
            return Err(Error::from(Error::NotFound));
        }
    }

    Ok(data.clone())    
}

/// Get records from database.
/// 
/// # Arguments
/// 
/// * `table` - Which table to handle.
/// * `uid` - Uuid for record, if `None`, get all.
/// * `conn` - Connection to database.
/// 
/// # Examples
/// 
/// ```
/// use db::{establish_sqlite_connection, create_db_record};
/// 
/// if let Ok(mut conn) = establish_sqlite_connection() {
///     match query_db_record("mission", None, &mut conn) {
///         Ok(records) => {
///             println!("get records: {:?}", records);
///         },
///         Err(error) => {
///             println!("failed to get records, errMsg: {:?}", error);
///         }
///     }   
/// }
/// ```
pub fn query_db_record(table: &str, uid: Option<&str>, conn: &mut SqliteConnection) -> Result<Vec<Record>, diesel::result::Error> {
    use diesel::result::Error;
    let mut res: Vec<Record> = Vec::new();

    match table {
        "ignore" => {
            let records: Vec<Ignore> = ignore::query_ignore_record(conn, uid)?;

            for item in records {
                let mut full = Record::default();
                full.ignore = item;
                res.push(full);
            }
        },
        "procedure" => {
            let records: Vec<Procedure> = procedure::query_procedure_record(conn, uid)?;

            for item in &records {
                let mut full = Record::default();
                full.procedure = item.clone();
                res.push(full);
            }     
        },
        "mission" => {
            let records: Vec<Mission> = mission::query_mission_record(conn, uid)?;

            for item in records {
                let mut full = Record::default();
                full.mission = item;
                res.push(full);
            }
        },
        "backup" => {
            let records = backup::query_backup_record(conn, None, uid)?;

            for item in records {
                let mut full = Record::default();
                full.backup = item;
                res.push(full);
            }
        },
        _ => {
            return Err(Error::from(Error::NotFound));
        }
    }

    Ok(res)
}

/// Delete records in database.
/// 
/// # Arguments
/// 
/// * `table` - Which table to handle.
/// * `uuid_0` - Uuid for record, if `None`, get all.
/// * `uuid_1` - Uuid for record, if `None`, get all.
/// * `conn` - Connection to database.
/// 
/// # Examples
/// 
/// ```
/// use db::{establish_sqlite_connection, create_db_record};
/// 
/// if let Ok(mut conn) = establish_sqlite_connection() {
///     match query_db_record("mission", None, &mut conn) {
///         Ok(records) => {
///             println!("get records: {:?}", records);
///         },
///         Err(error) => {
///             println!("failed to get records, errMsg: {:?}", error);
///         }
///     }   
/// }
/// ```
pub fn delete_db_record(table: &str, uuid_0: Option<&str>, uuid_1: Option<&str>, conn: &mut SqliteConnection) -> Result<usize, diesel::result::Error> {
    use diesel::result::Error;
    let remove_cnt: usize;
    
    match table {
        "ignore" => {
            remove_cnt = ignore::delete_ignore_record(conn, uuid_0, uuid_1)?;
        },
        "procedure" => {
            remove_cnt = procedure::delete_procedure_record(conn, uuid_0)?;   
        },
        "mission" => {
            remove_cnt = mission::delete_mission_record(conn, uuid_0)?;
        },
        "backup" => {
            remove_cnt = backup::delete_backup_record(conn, uuid_0, uuid_1)?;
        },
        _ => {
            return Err(Error::from(Error::NotFound));
        }
    }

    Ok(remove_cnt)    
}

/// Clear table records.
/// 
/// # Arguments
/// 
/// * `table` - Which table to handle.
/// * `conn` - Connection to database.
/// 
/// # Examples
/// 
/// ```
/// use db::{establish_sqlite_connection, clear_db_record};
/// 
/// if let Ok(mut conn) = establish_sqlite_connection() {
///     match clear_db_record("mission", &mut conn) {
///         Ok(cnt) => {
///             println!("clear total {} records", cnt);
///         },
///         Err(error) => {
///             println!("failed to clear records, errMsg: {:?}", error);
///         }
///     }   
/// }
/// ```
pub fn clear_db_record(table: &str, conn: &mut SqliteConnection) -> Result<usize, diesel::result::Error> {
    use diesel::result::Error;
    let remove_cnt: usize;
    
    match table {
        "ignore" => {
            remove_cnt = ignore::clear_ignore_record(conn)?;
        },
        "procedure" => {
            remove_cnt = procedure::clear_procedure_record(conn)?;       
        },
        "mission" => {
            remove_cnt = mission::clear_mission_record(conn)?;
        },
        "backup" => {
            remove_cnt = backup::clear_backup_record(conn)?;
        },
        _ => {
            return Err(Error::from(Error::NotFound));
        }
    }

    Ok(remove_cnt)    
}

/// Clean db records.
/// 
/// Delete `is_deleted` items and reorder the remaining id.
/// 
/// # Arguments
/// 
/// * `table` - Which table to handle.
/// * `conn` - Connection to database.
/// 
/// # Examples
/// 
/// ```
/// use db::{establish_sqlite_connection, clean_db_record};
/// 
/// if let Ok(mut conn) = establish_sqlite_connection() {
///     match clean_db_record("mission", &mut conn) {
///         Ok(cnt) => {
///             println!("table {} cleaned!", "mission");
///         },
///         Err(error) => {
///             println!("failed to clean table, errMsg: {:?}", error);
///         }
///     }   
/// }
/// ```
pub fn clean_db_record(table: &str, conn: &mut SqliteConnection) -> Result<usize, diesel::result::Error> {
    use diesel::result::Error;
    let remove_cnt: usize;
    
    match table {
        "ignore" => {
            remove_cnt = ignore::clean_record(conn)?;
        },
        "procedure" => {
            remove_cnt = procedure::clean_record(conn)?;       
        },
        "mission" => {
            remove_cnt = mission::clean_record(conn)?;
        },
        "backup" => {
            remove_cnt = backup::clean_record(conn)?;
        },
        _ => {
            return Err(Error::from(Error::NotFound));
        }
    }

    Ok(remove_cnt)    
}
