//! # Procedure
//! 
//! `procedure` module contains all functions about handle 'procedure' table.

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use chrono::{NaiveDateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::utils::common::rand_number;

/// Struct Procedure
#[derive(Debug, Queryable, Selectable, Insertable, AsChangeset, Serialize, Deserialize, Clone)]
#[diesel(table_name = super::schema::procedure)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Procedure {
    /// Primary key for table
    pub id: i32,

    /// Uuid for procedure
    pub procedure_id: String,

    /// Procedure name
    pub name: String,

    /// Whether has ignores
    pub has_ignores: bool,

    /// Ignore method
    /// 
    /// `0` - no ignores
    /// 
    /// `1` - use custom ignores
    /// 
    /// `2` - use .gitignore 
    pub ignore_method: i16,

    /// Whether compress
    pub is_compress: bool,

    /// Compress format
    /// 
    /// `0` - no compress
    /// 
    /// `1` - zip
    /// 
    /// `2` - tar.gz
    /// 
    /// `3` - tar.bz2
    /// 
    /// `4` - tar.xz
    /// 
    /// `5` - 7z
    pub compress_format: i16,

    /// Which trigger backup
    /// 
    /// `0` - reserved
    /// 
    /// `1` - cron trigger
    /// 
    /// `2` - monitore trigger
    pub trigger: i16,

    /// Cron expression
    /// 
    /// sec   min   hour   day of month   month   day of week   year
    /// 
    /// *     *     *      *              *       *             * 
    pub cron_expression: String,

    /// Whether restrict backups
    /// 
    /// `0` - no restrict
    /// 
    /// `1` - days restrict
    /// 
    /// `2` - size restrict
    /// 
    /// `3` - days and size restrict
    pub restrict: i16,

    /// Restrict days
    pub restrict_days: i16,

    /// Restrict size, in byte
    pub restrict_size: i64,

    /// Reserved for future use
    pub reserved_0: String,

    /// Reserved for future use
    pub reserved_1: String,

    /// Reserved for future use
    pub reserved_2: String,

    /// Procedure create time
    pub create_at: NaiveDateTime,

    /// Procedure update time
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

impl Default for Procedure {
    fn default() -> Self {
        Procedure {
            id: rand_number(),
            procedure_id: Uuid::new_v4().to_string(),
            name: "".to_string(),
            has_ignores: false,
            ignore_method: 1,
            is_compress: false,
            compress_format: 1,
            trigger: 1,
            cron_expression: "".to_string(),
            restrict: 0,
            restrict_days: 3,
            restrict_size: 1024,
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

// #[derive(AsChangeset, Insertable)]
// #[diesel(table_name = super::schema::procedure)]
// pub struct UpdateProcedure<'a> {
//     pub name:               Option<&'a String>,
//     pub has_ignores:         Option<&'a bool>,
//     pub ignore_method:      Option<&'a i16>,
//     pub is_compress:        Option<&'a bool>,
//     pub compress_format:    Option<&'a i16>,
//     pub trigger:            Option<&'a i16>,
//     pub cron_expression:    Option<&'a String>,
//     pub restrict:           Option<&'a i16>,
//     pub restrict_days:      Option<&'a i16>,
//     pub restrict_size:      Option<&'a i64>,
//     pub backup_method:      Option<&'a i16>,
//     pub update_at:          Option<&'a NaiveDateTime>,
//     pub is_deleted:         Option<&'a i16>,
//     pub delete_at:          Option<&'a NaiveDateTime>,
// }

// impl UpdateProcedure<'_> {
//     pub fn from(data: &Procedure) -> UpdateProcedure {
//         UpdateProcedure {
//             name: Some(&data.name),
//             has_ignores: Some(&data.has_ignores),
//             ignore_method: Some(&data.ignore_method),
//             is_compress: Some(&data.is_compress),
//             compress_format: Some(&data.compress_format),
//             trigger: Some(&data.trigger),
//             cron_expression: Some(&data.cron_expression),
//             restrict: Some(&data.restrict),
//             restrict_days: Some(&data.restrict_days),
//             restrict_size: Some(&data.restrict_size),
//             backup_method: Some(&data.backup_method),
//             update_at: Some(&data.update_at),
//             is_deleted: Some(&data.is_deleted),
//             delete_at: Some(&data.delete_at),
//         }
//     }
// }

/// Create procedure record and insert into database.
/// 
/// # Arguments
/// 
/// * `conn` - Connection to database.
/// * `data` - Data for procedure.
/// 
/// # Examples
/// 
/// ```
/// use db::{establish_sqlite_connection, procedure::create_procedure_record};
/// 
/// if let Ok(mut conn) = establish_sqlite_connection() {
///     let mut procedure = Procedure::default();
///     match create_procedure_record(&mut conn, &mut procedure) {
///         Ok(record) => {
///             println!("create record: {:?}", record);
///         },
///         Err(error) => {
///             println!("failed to create record, errMsg: {:?}", error);
///         }
///     }   
/// }
/// ```
pub fn create_procedure_record(
    conn: &mut SqliteConnection,
    data: &mut Procedure
) -> Result<Procedure, diesel::result::Error> {
    use super::schema::procedure::dsl::*;

    let cur_time: NaiveDateTime = Utc::now().naive_utc();
    data.id = procedure.count().get_result(conn).unwrap_or(0) as i32 + 1;
    data.procedure_id = Uuid::new_v4().to_string();
    data.create_at = cur_time;
    data.update_at = cur_time;

    diesel::insert_into(procedure)
        .values(data.clone())
        .returning(Procedure::as_returning())
        .get_result(conn)
}

/// Update procedure record in database.
/// 
/// # Arguments
/// 
/// * `conn` - Connection to database.
/// * `data` - Updated Data for procedure.
/// 
/// # Examples
/// 
/// ```
/// use db::{establish_sqlite_connection, procedure::update_procedure_record};
/// 
/// if let Ok(mut conn) = establish_sqlite_connection() {
///     let mut procedure = Procedure::default();
///     procedure.name = "procedure_1".to_string();
///     match update_procedure_record(&mut conn, &mut procedure) {
///         Ok(record) => {
///             println!("update record: {:?}", record);
///         },
///         Err(error) => {
///             println!("failed to update record, errMsg: {:?}", error);
///         }
///     }   
/// }
/// ```
pub fn update_procedure_record(
    conn: &mut SqliteConnection,
    data: &mut Procedure,
) -> Result<Procedure, diesel::result::Error> {
    use super::schema::{procedure, procedure::procedure_id};

    let cur_time: NaiveDateTime = Utc::now().naive_utc();
    data.update_at = cur_time;

    diesel::update(procedure::table)
        .filter(procedure_id.eq(&data.procedure_id))
        .set(data.clone())
        .returning(Procedure::as_returning())
        .get_result(conn)
}

/// Get procedure records from database.
/// 
/// # Arguments
/// 
/// * `conn` - Connection to database.
/// * `pid` - Uuid for procedure, if `None`, get all.
/// 
/// # Examples
/// 
/// ```
/// use db::{establish_sqlite_connection, procedure::query_procedure_record};
/// 
/// if let Ok(mut conn) = establish_sqlite_connection() {
///     match query_procedure_record(&mut conn, None) {
///         Ok(records) => {
///             println!("get all records: {:?}", records);
///         },
///         Err(error) => {
///             println!("failed to get records, errMsg: {:?}", error);
///         }
///     }   
/// }
/// ```
pub fn query_procedure_record(
    conn: &mut SqliteConnection,
    pid: Option<&str>,
) -> Result<Vec<Procedure>, diesel::result::Error> {
    use super::schema::procedure::dsl::*;

    match pid {
        Some(uid) => {
            procedure.filter(procedure_id.eq(uid))
                .filter(is_deleted.eq(0))
                .select(Procedure::as_select())
                .load(conn)
        },
        None => {
            procedure.select(Procedure::as_select())
                .filter(is_deleted.eq(0))
                .load(conn)
        }
    }
}

/// Delete procedure record in database.
/// 
/// # Arguments
/// 
/// * `conn` - Connection to database.
/// * `pid` - Uuid for procedure, delete single procedure records.
/// 
/// # Examples
/// 
/// ```
/// use db::{establish_sqlite_connection, procedure::delete_procedure_record};
/// 
/// if let Ok(mut conn) = establish_sqlite_connection() {
///     let pid = "0af123df-54ac-4030-beb6-1929c218c099";
///     match delete_procedure_record(&mut conn, Some(&pid)) {
///         Ok(cnt) => {
///             println!("delete {} records", cnt);
///         },
///         Err(error) => {
///             println!("failed to delete records, errMsg: {:?}", error);
///         }
///     }   
/// }
/// ```
pub fn delete_procedure_record(
    conn: &mut SqliteConnection,
    pid: Option<&str>,
) -> Result<usize, diesel::result::Error> {
    use super::schema::procedure::dsl::*;
    use diesel::result::Error;

    match pid {
        Some(uid) => {
            return diesel::update(procedure)
                .filter(procedure_id.eq(uid))
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

/// Clear 'procedure' table records.
/// 
/// # Arguments
/// 
/// * `conn` - Connection to database.
/// 
/// # Examples
/// 
/// ```
/// use db::{establish_sqlite_connection, procedure::clear_procedure_record};
/// 
/// if let Ok(mut conn) = establish_sqlite_connection() {
///     match clear_procedure_record(&mut conn) {
///         Ok(cnt) => {
///             println!("clear table with total {} records", cnt);
///         },
///         Err(error) => {
///             println!("failed to clear records, errMsg: {:?}", error);
///         }
///     }   
/// }
/// ```
pub fn clear_procedure_record(
    conn: &mut SqliteConnection, 
) -> Result<usize, diesel::result::Error> {
    use super::schema::procedure::dsl::*;

    diesel::delete(procedure)
        .execute(conn)
}

/// Clean 'procedure' table records.
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
/// use db::{establish_sqlite_connection, procedure::clean_procedure_record};
/// 
/// if let Ok(mut conn) = establish_sqlite_connection() {
///     match clean_procedure_record(&mut conn) {
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
    use super::schema::procedure::dsl::*;

    let cleaned: usize = diesel::delete(procedure.filter(is_deleted.eq(1))).execute(conn)?;

    let mut remaining: Vec<Procedure> = procedure.select(Procedure::as_select()).load(conn)?;
    for (idx, item) in remaining.iter_mut().enumerate() {
        let new_id = (idx + 1) as i32;
        diesel::update(procedure)
            .filter(procedure_id.eq(&item.procedure_id))
            .set(id.eq(new_id))
            .execute(conn)?;
    }

    Ok(cleaned)   
}
