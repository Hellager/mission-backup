//! # Mission
//! 
//! `mission` module contains all functions about handle 'mission' table.

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use chrono::{NaiveDateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::utils::common::rand_number;

/// Struct Mission
#[derive(Debug, Queryable, Selectable, Insertable, AsChangeset, Serialize, Deserialize, Clone)]
#[diesel(table_name = super::schema::mission)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Mission {
    /// Primary key for table
    pub id: i32,

    /// Uuid for mission
    pub mission_id: String,

    /// Using which procedure
    pub procedure_id: String,

    /// Mission name
    pub name: String,

    /// Mission status
    /// 
    /// `0` - pause
    /// 
    /// `1` - running
    /// 
    /// `2` - backuping
    pub status: i16,

    /// Description for mission
    pub description: String,

    /// Target source path(absolute)
    pub src_path: String,

    /// Target save path(absolute)
    pub dst_path: String,

    /// Target path type
    /// 
    /// `0` - unknown
    /// 
    /// `1` - file
    /// 
    /// `2` - directory
    pub path_type: i16,

    /// Mission next runtime
    pub next_runtime: NaiveDateTime,

    /// Mission last trigger time
    pub last_trigger: NaiveDateTime,

    /// Reserved for future use
    pub reserved_0: String,

    /// Reserved for future use
    pub reserved_1: String,

    /// Reserved for future use
    pub reserved_2: String,
    
    /// Mission create time
    pub create_at: NaiveDateTime,

    /// Mission update time
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

impl Default for Mission {
    fn default() -> Self {
        Mission {
            id: rand_number(),
            mission_id: Uuid::new_v4().to_string(),
            procedure_id: Uuid::new_v4().to_string(),
            name: "".to_string(),
            status: 0,
            description: "".to_string(),
            src_path: "".to_string(),
            dst_path: "".to_string(),
            path_type: 0,
            next_runtime: Utc::now().naive_utc(),
            last_trigger: Utc::now().naive_utc(),
            reserved_0: "".to_string(),
            reserved_1: "".to_string(),
            reserved_2: "".to_string(),
            create_at: Utc::now().naive_utc(),
            update_at: Utc::now().naive_utc(),
            is_deleted: 0,
            delete_at: Utc::now().naive_utc(),
        }
    }
}

/// Create mission record and insert into database.
/// 
/// # Arguments
/// 
/// * `conn` - Connection to database.
/// * `data` - Data for mission.
/// 
/// # Examples
/// 
/// ```
/// use db::{establish_sqlite_connection, mission::create_mission_record};
/// 
/// if let Ok(mut conn) = establish_sqlite_connection() {
///     let mut mission = Mission::default();
///     match create_mission_record(&mut conn, &mut mission) {
///         Ok(record) => {
///             println!("create record: {:?}", record);
///         },
///         Err(error) => {
///             println!("failed to create record, errMsg: {:?}", error);
///         }
///     }   
/// }
/// ```
pub fn create_mission_record(
    conn: &mut SqliteConnection,
    data: &mut Mission
) -> Result<Mission, diesel::result::Error> {
    use super::schema::mission::dsl::*;

    let cur_time: NaiveDateTime = Utc::now().naive_utc();
    data.id = mission.count().get_result(conn).unwrap_or(0) as i32 + 1;
    data.mission_id = Uuid::new_v4().to_string();
    data.create_at = cur_time;
    data.update_at = cur_time;

    diesel::insert_into(mission)
        .values(data.clone())
        .returning(Mission::as_returning())
        .get_result(conn)
}

/// Update mission record in database.
/// 
/// # Arguments
/// 
/// * `conn` - Connection to database.
/// * `data` - Updated Data for mission.
/// 
/// # Examples
/// 
/// ```
/// use db::{establish_sqlite_connection, mission::update_mission_record};
/// 
/// if let Ok(mut conn) = establish_sqlite_connection() {
///     let mut mission = Mission::default();
///     mission.name = "John Wick".to_string();
///     match update_ignore_record(&mut conn, &mut mission) {
///         Ok(record) => {
///             println!("update record: {:?}", record);
///         },
///         Err(error) => {
///             println!("failed to update record, errMsg: {:?}", error);
///         }
///     }   
/// }
/// ```
pub fn update_mission_record(
    conn: &mut SqliteConnection,
    data: &mut Mission,
) -> Result<Mission, diesel::result::Error> {
    use super::schema::{mission, mission::mission_id};

    let cur_time: NaiveDateTime = Utc::now().naive_utc();
    data.update_at = cur_time;

    diesel::update(mission::table)
        .filter(mission_id.eq(&data.mission_id))
        .set(data.clone())
        .returning(Mission::as_returning())
        .get_result(conn)
}

/// Update mission status in database.
/// 
/// # Arguments
/// 
/// * `conn` - Connection to database.
/// * `stat` - The status to update.
/// * `mid` - Which mission to update.
/// 
/// # Examples
/// 
/// ```
/// use db::{establish_sqlite_connection, mission::update_mission_status};
/// 
/// if let Ok(mut conn) = establish_sqlite_connection() {
///     let status = 1;
///     let mid = "73d96957-f383-4f6e-8fb8-b0d3824d0fc9";
///     match update_mission_time(&mut conn, status, mid) {
///         Ok(record) => {
///             println!("update mission time: {:?}", record);
///         },
///         Err(error) => {
///             println!("failed to update mission time, errMsg: {:?}", error);
///         }
///     }   
/// }
/// ```
pub fn update_mission_status(
    conn: &mut SqliteConnection,
    stat: i16,
    mid: &str,
) -> Result<Mission, diesel::result::Error> {
    use super::schema::mission::dsl::*;

    diesel::update(mission)
        .filter(mission_id.eq(mid))
        .set(status.eq(stat))
        .returning(Mission::as_returning())
        .get_result(conn)
}

/// Update mission next_runtime or last_trigger time in database.
/// 
/// # Arguments
/// 
/// * `conn` - Connection to database.
/// * `label` - Determine which time to update.('next', 'last')
/// * `time` - The time to update.
/// * `mid` - Which mission to update.
/// 
/// # Examples
/// 
/// ```
/// use db::{establish_sqlite_connection, mission::update_mission_time};
/// 
/// if let Ok(mut conn) = establish_sqlite_connection() {
///     let label = "next";
///     let time = chrono::Utc::now().naive_utc();
///     let mid = "73d96957-f383-4f6e-8fb8-b0d3824d0fc9";
///     match update_mission_time(&mut conn, label &time, mid) {
///         Ok(record) => {
///             println!("update mission time: {:?}", record);
///         },
///         Err(error) => {
///             println!("failed to update mission time, errMsg: {:?}", error);
///         }
///     }   
/// }
/// ```
pub fn update_mission_time(
    conn: &mut SqliteConnection,
    label: &str,
    time: &chrono::DateTime<chrono::Utc>,
    mid: &str,
) -> Result<Mission, diesel::result::Error> {
    use super::schema::mission::dsl::*;

    let runtime = time.naive_utc();

    match label {
        "next" => {
            diesel::update(mission)
                .filter(mission_id.eq(mid))
                .set(next_runtime.eq(runtime))
                .returning(Mission::as_returning())
                .get_result(conn)
        },
        "last" => {
            diesel::update(mission)
                .filter(mission_id.eq(mid))
                .set(last_trigger.eq(runtime))
                .returning(Mission::as_returning())
                .get_result(conn)
        },
        _ => {
            return Err(diesel::result::Error::NotFound);
        }
    }
}

/// Get mission records from database.
/// 
/// # Arguments
/// 
/// * `conn` - Connection to database.
/// * `mid` - Uuid for mission, if `None`, get all.
/// 
/// # Examples
/// 
/// ```
/// use db::{establish_sqlite_connection, mission::query_mission_record};
/// 
/// if let Ok(mut conn) = establish_sqlite_connection() {
///     match query_mission_record(&mut conn, None) {
///         Ok(records) => {
///             println!("get all records: {:?}", records);
///         },
///         Err(error) => {
///             println!("failed to get records, errMsg: {:?}", error);
///         }
///     }   
/// }
/// ```
pub fn query_mission_record(
    conn: &mut SqliteConnection,
    mid: Option<&str>,
) -> Result<Vec<Mission>, diesel::result::Error> {
    use super::schema::mission::dsl::*;

    match mid {
        Some(uid) => {
            mission.filter(mission_id.eq(uid))
                .filter(is_deleted.eq(0))
                .select(Mission::as_select())
                .load(conn)
        },
        None => {
            mission.select(Mission::as_select())
                .filter(is_deleted.eq(0))
                .load(conn)
        }
    }
}

/// Delete mission record in database logically.
/// 
/// # Arguments
/// 
/// * `conn` - Connection to database.
/// * `mid` - Uuid for mission, delete single mission records.
/// 
/// # Examples
/// 
/// ```
/// use db::{establish_sqlite_connection, mission::delete_mission_record};
/// 
/// if let Ok(mut conn) = establish_sqlite_connection() {
///     let mid = "73d96957-f383-4f6e-8fb8-b0d3824d0fc9";
///     match delete_mission_record(&mut conn, Some(mid)) {
///         Ok(cnt) => {
///             println!("delete {} records", cnt);
///         },
///         Err(error) => {
///             println!("failed to delete records, errMsg: {:?}", error);
///         }
///     }   
/// }
/// ```
pub fn delete_mission_record(
    conn: &mut SqliteConnection,
    mid: Option<&str>,
) -> Result<usize, diesel::result::Error> {
    use super::schema::mission::dsl::*;
    use diesel::result::Error;

    match mid {
        Some(uid) => {
            return diesel::update(mission)
                .filter(mission_id.eq(uid))
                .set((
                    is_deleted.eq(1),
                    delete_at.eq(Utc::now().naive_utc())
                ))
                .execute(conn); 
        },
        None => {
            return Err(Error::NotFound);
        }
    }
}

/// Clear 'mission' table records.
/// 
/// # Arguments
/// 
/// * `conn` - Connection to database.
/// 
/// # Examples
/// 
/// ```
/// use db::{establish_sqlite_connection, mission::clear_mission_record};
/// 
/// if let Ok(mut conn) = establish_sqlite_connection() {
///     match clear_mission_record(&mut conn) {
///         Ok(cnt) => {
///             println!("clear table with total {} records", cnt);
///         },
///         Err(error) => {
///             println!("failed to clear records, errMsg: {:?}", error);
///         }
///     }   
/// }
/// ```
pub fn clear_mission_record(
    conn: &mut SqliteConnection, 
) -> Result<usize, diesel::result::Error> {
    use super::schema::mission::dsl::*;

    diesel::delete(mission)
        .execute(conn)
}

/// Get mission related record, return related mission and procedure.
/// 
/// # Arguments
/// 
/// * `mid` - Target mission id.
/// * `conn` - Connection to database.
/// 
/// # Examples
/// 
/// ```
/// use db::{establish_sqlite_connection, mission::get_mission_related_record};
/// 
/// if let Ok(mut conn) = establish_sqlite_connection() {
///     let mid = "73d96957-f383-4f6e-8fb8-b0d3824d0fc9";
///     match get_mission_related_record(mid, &mut conn) {
///         Ok(record) => {
///             println!("related record: {:?}", record);
///         },
///         Err(error) => {
///             println!("failed to get related record, errMsg: {:?}", error);
///         }
///     }   
/// }
/// ```
pub fn get_mission_related_record(mid: &str, conn: &mut SqliteConnection) -> Result<crate::db::Record, std::io::Error> {
    use super::{Record, procedure::query_procedure_record};
    use std::io::{ Error, ErrorKind };

    let mut record = Record::default();
    if let Ok(missions) = query_mission_record(conn, Some(mid)) {
        if missions.len() == 0 {
            return Err(Error::from(ErrorKind::NotFound));
        }

        let mission = &missions[0];
        record.mission = missions[0].clone();
        if let Ok(procedures) = query_procedure_record(conn, Some(&mission.procedure_id)) {
            if procedures.len() == 0 {
                return Err(Error::from(ErrorKind::NotFound));
            }

            record.procedure = procedures[0].clone();
            if record.procedure.is_deleted == 1 {
                return Err(Error::from(ErrorKind::NotFound));
            }
        }

        return Ok(record);
    }   

    Err(Error::from(ErrorKind::NotFound))
}

/// Clean 'mission' table records.
/// 
/// Physically delete records where `is_deleted` is `1`, and reorder the remaining records.
/// 
/// # Arguments
/// 
/// * `conn` - Connection to database.
/// 
/// # Examples
/// 
/// ```
/// use db::{establish_sqlite_connection, mission::clean_mission_record};
/// 
/// if let Ok(mut conn) = establish_sqlite_connection() {
///     match clean_mission_record(&mut conn) {
///         Ok(cnt) => {
///             println!("cleaned {} records", cnt);
///         },
///         Err(error) => {
///             println!("failed to clean records, errMsg: {:?}", error);
///         }
///     }   
/// }
/// ```
pub fn clean_record(
    conn: &mut SqliteConnection, 
) -> Result<usize, diesel::result::Error> {
    use super::schema::mission::dsl::*;

    let cleaned: usize = diesel::delete(mission.filter(is_deleted.eq(1))).execute(conn)?;

    let mut remaining: Vec<Mission> = mission.select(Mission::as_select()).load(conn)?;
    for (idx, item) in remaining.iter_mut().enumerate() {
        let new_id = (idx + 1) as i32;
        diesel::update(mission)
            .filter(mission_id.eq(&item.mission_id))
            .set(id.eq(new_id))
            .execute(conn)?;
    }

    Ok(cleaned)   
}
