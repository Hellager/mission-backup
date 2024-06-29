//! # Backup
//! 
//! `backup` module contains all functions about handle 'backup' table.

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use chrono::{NaiveDateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use super::mission::Mission;
use crate::utils::common::rand_number;

/// Struct Backup
#[derive(Debug, Queryable, Selectable, Insertable, AsChangeset, Serialize, Deserialize, Clone)]
#[diesel(table_name = super::schema::backup)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Backup {
    /// Primary key for table
    pub id: i32,

    /// Uuid for backup
    pub backup_id: String,
    
    /// Belong to which mission
    pub mission_id: String,

    /// Backup save path(absolute)
    pub save_path: String,

    /// Backup size
    pub backup_size: i64,

    /// Reserved for future use
    pub reserved0: String,

    /// Reserved for future use
    pub reserved1: String,

    /// Reserved for future use
    pub reserved2: String,

    /// Backup create time
    pub create_at: NaiveDateTime,

    /// Backup update time
    pub update_at: NaiveDateTime,

    /// Whether been deleted
    /// 
    /// `0` - not deleted
    /// 
    /// `1` - been deleted
    pub is_deleted: i16,

    /// Delete time
    pub delete_at: NaiveDateTime,
}

impl Default for Backup {
    fn default() -> Self {
        Backup {
            id: rand_number(),
            backup_id: Uuid::new_v4().to_string(),
            mission_id: Uuid::new_v4().to_string(),
            save_path: "".to_string(),
            backup_size: 0,
            reserved0: "".to_string(),
            reserved1: "".to_string(),
            reserved2: "".to_string(),
            create_at: Utc::now().naive_utc(),
            update_at: Utc::now().naive_utc(),
            is_deleted: 0,
            delete_at: Utc::now().naive_utc(),
        }
    }
}

// #[derive(AsChangeset, Insertable)]
// #[diesel(table_name = super::schema::backup)]
// pub struct UpdateBackup<'a> {
//     pub backup_id:      Option<&'a String>,
//     pub mission_id:     Option<&'a String>,
//     pub save_path:      Option<&'a String>,
//     pub backup_size:    Option<&'a i64>,
//     pub create_at:      Option<&'a NaiveDateTime>,
//     pub is_deleted:     Option<&'a i16>,
//     pub delete_at:      Option<&'a NaiveDateTime>,
// }

// impl UpdateBackup<'_> {
//     pub fn from(data: &Backup) -> UpdateBackup {
//         UpdateBackup {
//             backup_id: Some(&data.backup_id),
//             mission_id: Some(&data.mission_id),
//             save_path: Some(&data.save_path),
//             backup_size: Some(&data.backup_size),
//             create_at: Some(&data.create_at),
//             is_deleted: Some(&data.is_deleted),
//             delete_at: Some(&data.delete_at),
//         }
//     }
// }

/// Create backup record and insert into database.
/// 
/// # Arguments
/// 
/// * `conn` - Connection to database.
/// * `data` - Data for backup.
/// * `mission` - Which mission generates this backup
/// 
/// # Examples
/// 
/// ```
/// use db::{establish_sqlite_connection, backup::create_backup_record};
/// 
/// if let Ok(mut conn) = establish_sqlite_connection() {
///     let mut backup = Backup::default();
///     let mission = Mission::default();
///     match create_backup_record(&mut conn, &mut backup, &mission) {
///         Ok(record) => {
///             println!("create record: {:?}", record);
///         },
///         Err(error) => {
///             println!("failed to create record, errMsg: {:?}", error);
///         }
///     }   
/// }
/// ```
pub fn create_backup_record(
    conn: &mut SqliteConnection,
    data: &mut Backup,
    mission: &Mission
) -> Result<Backup, diesel::result::Error> {
    use super::schema::backup::dsl::*;

    let cur_time: NaiveDateTime = Utc::now().naive_utc();
    data.id = backup.count().get_result(conn).unwrap_or(0) as i32 + 1;
    data.backup_id = Uuid::new_v4().to_string();
    data.mission_id = mission.mission_id.clone();
    data.create_at = cur_time;

    diesel::insert_into(backup)
        .values(data.clone())
        .returning(Backup::as_returning())
        .get_result(conn)
}

#[allow(dead_code)]
/// Update backup record in database.
/// 
/// # Arguments
/// 
/// * `conn` - Connection to database.
/// * `data` - Updated Data for backup.
/// 
/// # Examples
/// 
/// ```
/// use db::{establish_sqlite_connection, mission::update_backup_record};
/// 
/// if let Ok(mut conn) = establish_sqlite_connection() {
///     let mut backup = Backup::default();
///     backup.is_deleted = 1;
///     match update_backup_record(&mut conn, &mut backup) {
///         Ok(record) => {
///             println!("update record: {:?}", record);
///         },
///         Err(error) => {
///             println!("failed to update record, errMsg: {:?}", error);
///         }
///     }   
/// }
/// ```
pub fn update_backup_record(
    conn: &mut SqliteConnection,
    data: &mut Backup,
) -> Result<Backup, diesel::result::Error> {
    use super::schema::{backup, backup::backup_id};

    diesel::update(backup::table)
        .filter(backup_id.eq(&data.backup_id))
        .set(data.clone())
        .returning(Backup::as_returning())
        .get_result(conn)
}

/// Get backup records from database.
/// 
/// # Arguments
/// 
/// * `conn` - Connection to database.
/// * `bid` - Uuid for backup, query single backup record.
/// * `mid` - Uuid for mission, query mission related backups.
/// 
/// # Examples
/// 
/// ```
/// use db::{establish_sqlite_connection, backup::query_backup_record};
/// 
/// if let Ok(mut conn) = establish_sqlite_connection() {
///     match query_backup_record(&mut conn, None, None) {
///         Ok(records) => {
///             println!("get all records: {:?}", records);
///         },
///         Err(error) => {
///             println!("failed to get records, errMsg: {:?}", error);
///         }
///     }   
/// }
/// ```
pub fn query_backup_record(
    conn: &mut SqliteConnection,
    bid: Option<&str>,
    mid: Option<&str>,
) -> Result<Vec<Backup>, diesel::result::Error> {
    use super::schema::backup::dsl::*;

    if let Some(uuid) = bid {
        backup.filter(backup_id.eq(uuid))
            .filter(is_deleted.eq(0))
            .select(Backup::as_select())
            .load(conn)
    } else if let Some(uuid) = mid {
        backup.filter(mission_id.eq(uuid))
            .filter(is_deleted.eq(0))
            .select(Backup::as_select())
            .load(conn)
    } else {
        backup.filter(is_deleted.eq(0))
            .select(Backup::as_select())
            .load(conn)        
    }  
}

// /// Get backup records from database for statistic.
// /// 
// /// # Arguments
// /// 
// /// * `conn` - Connection to database.
// /// * `mid` - Uuid for mission, query mission related backups.
// /// * `start` - Start date for backup.
// /// * `stop` - Stop date for backup.
// /// 
// /// # Examples
// /// 
// /// ```
// /// use db::{establish_sqlite_connection, backup::query_backup_record_with_date};
// /// use chrono::{Duration, NaiveDateTime, Utc};
// /// 
// /// if let Ok(mut conn) = establish_sqlite_connection() {
// ///     let mid = "1c69eead-b7cf-457e-95e2-9c9f459120ff";
// ///     let start = Utc::now().naive_utc();
// ///     let stop_at = Utc::now() + Duration::days(1);
// ///     let stop = stop_at.naive_utc();
// ///     match query_backup_record_with_date(&mut conn, mid, start, stop) {
// ///         Ok(records) => {
// ///             println!("get records: {:?}", records);
// ///         },
// ///         Err(error) => {
// ///             println!("failed to get records, errMsg: {:?}", error);
// ///         }
// ///     }   
// /// }
// /// ```
// pub fn query_backup_record_with_date(
//     conn: &mut SqliteConnection,
//     mid: &str,
//     start: Option<&NaiveDateTime>,
//     stop: Option<&NaiveDateTime>,
// ) -> Result<Vec<Backup>, diesel::result::Error> {
//     use super::schema::backup::dsl::*;

//     if start == None && stop == None {
//         return  backup.filter(mission_id.eq(mid))
//             .filter(is_deleted.eq(0))
//             .select(Backup::as_select())
//             .load(conn);     
//     }

//     if let Some(s_date) = start{
//         if let Some(e_date) = stop {
//             return     backup.filter(mission_id.eq(mid))
//                 .filter(is_deleted.eq(0))
//                 .filter(create_at.between(s_date, e_date))
//                 .select(Backup::as_select())
//                 .load(conn);
//         } else {
//             return     backup.filter(mission_id.eq(mid))
//                 .filter(is_deleted.eq(0))
//                 .filter(create_at.ge(s_date))
//                 .select(Backup::as_select())
//                 .load(conn);
//         }
//     } else {
//         if let Some(e_date) = stop {
//             return     backup.filter(mission_id.eq(mid))
//             .filter(is_deleted.eq(0))
//             .filter(create_at.le(e_date))
//             .select(Backup::as_select())
//             .load(conn); 
//         }
//     }

//     Err(diesel::result::Error::NotFound)
// }

/// Delete backup record in database logically.
/// 
/// # Arguments
/// 
/// * `conn` - Connection to database.
/// * `bid` - Uuid for backup, delete single backup record.
/// * `mid` - Uuid for mission, delete all related backup records.
/// 
/// # Examples
/// 
/// ```
/// use db::{establish_sqlite_connection, backup::delete_backup_record};
/// 
/// if let Ok(mut conn) = establish_sqlite_connection() {
///     let mid = "1c69eead-b7cf-457e-95e2-9c9f459120ff";
///     match delete_backup_record(&mut conn, None, Some(mid)) {
///         Ok(cnt) => {
///             println!("delete {} records", cnt);
///         },
///         Err(error) => {
///             println!("failed to delete records, errMsg: {:?}", error);
///         }
///     }   
/// }
/// ```
pub fn delete_backup_record(
    conn: &mut SqliteConnection,
    bid: Option<&str>,
    mid: Option<&str>,
) -> Result<usize, diesel::result::Error> {
    use super::schema::backup::dsl::*;

    if let Some(uuid) = bid {
        return diesel::update(backup)
                    .filter(backup_id.eq(uuid))
                    .set((
                        is_deleted.eq(1),
                        delete_at.eq(Utc::now().naive_utc())
                    ))
                    .execute(conn);
    } else if let Some(uuid) = mid {
        return diesel::update(backup)
                    .filter(mission_id.eq(uuid))
                    .set((
                        is_deleted.eq(1),
                        delete_at.eq(Utc::now().naive_utc())
                    ))
                    .execute(conn);
    }   

    Err(diesel::result::Error::NotFound)
}

/// Clear 'backup' table records.
/// 
/// # Arguments
/// 
/// * `conn` - Connection to database.
/// 
/// # Examples
/// 
/// ```
/// use db::{establish_sqlite_connection, backup::clear_backup_record};
/// 
/// if let Ok(mut conn) = establish_sqlite_connection() {
///     match clear_backup_record(&mut conn) {
///         Ok(cnt) => {
///             println!("clear table with total {} records", cnt);
///         },
///         Err(error) => {
///             println!("failed to clear records, errMsg: {:?}", error);
///         }
///     }   
/// }
/// ```
pub fn clear_backup_record(
    conn: &mut SqliteConnection, 
) -> Result<usize, diesel::result::Error> {
    use super::schema::backup::dsl::*;

    diesel::delete(backup)
        .execute(conn)
}

// /// Clean 'backup' table records.
// /// 
// /// Physically delete records where `is_deleted` is `1`, and reorder the remaining records.
// /// 
// /// # Arguments
// /// 
// /// * `conn` - Connection to database.
// /// 
// /// # Examples
// /// 
// /// ```
// /// use db::{establish_sqlite_connection, backup::clean_backup_record};
// /// 
// /// if let Ok(mut conn) = establish_sqlite_connection() {
// ///     match clean_backup_record(&mut conn) {
// ///         Ok(cnt) => {
// ///             println!("cleaned {} records", cnt);
// ///         },
// ///         Err(error) => {
// ///             println!("failed to clean records, errMsg: {:?}", error);
// ///         }
// ///     }   
// /// }
// /// ```
// pub fn clean_backup_record(
//     conn: &mut SqliteConnection, 
// ) -> Result<usize, diesel::result::Error> {
//     use super::schema::backup::dsl::*;
//     use std::path::Path;

//     // delete invalid backups
//     let cur_backups = query_backup_record(conn, None, None)?;
//     for item in cur_backups.iter() {
//         if !Path::new(&item.save_path).exists() {
//             delete_backup_record(conn, Some(&item.backup_id), None)?;
//         }
//     }

//     // physically delete backup records
//     let cleaned: usize = diesel::delete(backup.filter(is_deleted.eq(1))).execute(conn)?;

//     // reorder remaining records
//     let mut remaining: Vec<Backup> = backup.select(Backup::as_select()).load(conn)?;
//     for (idx, item) in remaining.iter_mut().enumerate() {
//         let new_id = (idx + 1) as i32;
//         diesel::update(backup)
//             .filter(backup_id.eq(&item.backup_id))
//             .set(id.eq(new_id))
//             .execute(conn)?;
//     }

//     Ok(cleaned)   
// }

/// Physically delete backup in disk.
/// 
/// Logically delete backup in record.
/// 
/// # Arguments
/// 
/// * `bid` - Uuid for backup.
/// * `conn` - Connection to database.
/// 
/// # Examples
/// 
/// ```
/// use db::{establish_sqlite_connection, backup::delete_backup};
/// 
/// if let Ok(mut conn) = establish_sqlite_connection() {
///     let bid = "661b7d0e-a52c-457e-89e1-2ffe9a230c14";
///     match delete_backup(backup_id, &mut conn) {
///         Ok(cnt) => {
///             println!("remove {} backup with id {}", cnt, backup_id);
///         },
///         Err(error) => {
///             println!("failed to remove backup, errMsg: {:?}", error);
///         }
///     }   
/// }
/// ```
pub fn delete_backup(bid: &str, conn: &mut SqliteConnection) -> Result<(), std::io::Error> {
    use crate::utils::explorer::remove_all;
    use std::path::Path;
    use std::io::{ Error, ErrorKind };
    
    if let Ok(record) = query_backup_record(conn, Some(bid), None) {
        if record.len() > 0 {
            let backup = record[0].clone();

            if let Some(backup_dir) = Path::new(&backup.save_path).parent() {
                remove_all(backup_dir.display().to_string().as_str())?;

                if let Ok(_) = delete_backup_record(conn, Some(bid), None) {
                    return Ok(());
                }
            }            
        }
    }

    Err(Error::from(ErrorKind::NotFound))
}
