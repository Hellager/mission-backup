//! # Ignore
//! 
//! `ignore` module contains all functions about handle 'ignore' table.

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use chrono::{NaiveDateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use super::procedure::Procedure;
use crate::utils::common::rand_number;

/// Struct Ignore
#[derive(Debug, Queryable, Selectable, Insertable, AsChangeset, Serialize, Deserialize, Clone)]
#[diesel(table_name = super::schema::ignore)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Ignore {
    /// Primary key for table
    pub id: i32,

    /// Uuid for ignore
    pub ignore_id: String,

    /// Belong to which procedure
    pub procedure_id: String,

    /// Ignore keyword
    pub keyword: String,

    /// Reserved for future use
    pub reserved_0: String,

    /// Reserved for future use
    pub reserved_1: String,

    /// Reserved for future use
    pub reserved_2: String,

    /// Ignore create time
    pub create_at: NaiveDateTime,

    /// Ignore update time
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

impl Default for Ignore {
    fn default() -> Self {
        Ignore {
            id: rand_number(),
            ignore_id: Uuid::new_v4().to_string(),
            procedure_id: Uuid::new_v4().to_string(),
            keyword: "".to_string(),
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
// #[diesel(table_name = super::schema::ignore)]
// pub struct UpdateIgnore<'a> {
//     pub ignore_id:      Option<&'a String>,
//     pub procedure_id:   Option<&'a String>,
//     pub keyword:        Option<&'a String>,
//     pub create_at:      Option<&'a NaiveDateTime>,
//     pub update_at:      Option<&'a NaiveDateTime>,
//     pub is_deleted:     Option<&'a i16>,
//     pub delete_at:      Option<&'a NaiveDateTime>,
// }

// impl UpdateIgnore<'_> {
//     pub fn from(data: &Ignore) -> UpdateIgnore {
//         UpdateIgnore {
//             ignore_id: Some(&data.ignore_id),
//             procedure_id: Some(&data.procedure_id),
//             keyword: Some(&data.keyword),
//             create_at: Some(&data.create_at),
//             update_at: Some(&data.update_at),
//             is_deleted: Some(&data.is_deleted),
//             delete_at: Some(&data.delete_at),
//         }
//     }
// }

/// Create ignore record and insert into database.
/// 
/// # Arguments
/// 
/// * `conn` - Connection to database.
/// * `data` - Data for ignore.
/// * `procedure` - Which procedure generates this ignore
/// 
/// # Examples
/// 
/// ```
/// use db::{establish_sqlite_connection, ignore::create_ignore_record};
/// 
/// if let Ok(mut conn) = establish_sqlite_connection() {
///     let mut ignore = Ignore::default();
///     let procedure = Procedure::default();
///     match create_ignore_record(&mut conn, &mut ignore, &procedure) {
///         Ok(record) => {
///             println!("create record: {:?}", record);
///         },
///         Err(error) => {
///             println!("failed to create record, errMsg: {:?}", error);
///         }
///     }   
/// }
/// ```
pub fn create_ignore_record(
    conn: &mut SqliteConnection,
    data: &mut Ignore,
    procedure: &Procedure
) -> Result<Ignore, diesel::result::Error> {
    use super::schema::ignore::dsl::*;

    let cur_time: NaiveDateTime = Utc::now().naive_utc();
    data.id = ignore.count().get_result(conn).unwrap_or(0) as i32 + 1;
    data.ignore_id = Uuid::new_v4().to_string();
    data.procedure_id = procedure.procedure_id.clone();
    data.create_at = cur_time;
    data.update_at = cur_time;

    diesel::insert_into(ignore)
        .values(data.clone())
        .returning(Ignore::as_returning())
        .get_result(conn)
}

/// Update ignore record in database.
/// 
/// # Arguments
/// 
/// * `conn` - Connection to database.
/// * `data` - Updated Data for ignore.
/// 
/// # Examples
/// 
/// ```
/// use db::{establish_sqlite_connection, ignore::update_ignore_record};
/// 
/// if let Ok(mut conn) = establish_sqlite_connection() {
///     let mut ignore = Ignore::default();
///     ignore.keyword = "Hello, world!".to_string();
///     match update_ignore_record(&mut conn, &mut ignore) {
///         Ok(record) => {
///             println!("update record: {:?}", record);
///         },
///         Err(error) => {
///             println!("failed to update record, errMsg: {:?}", error);
///         }
///     }   
/// }
/// ```
pub fn update_ignore_record(
    conn: &mut SqliteConnection,
    data: &mut Ignore,
) -> Result<Ignore, diesel::result::Error> {
    use super::schema::{ignore, ignore::ignore_id};

    let cur_time: NaiveDateTime = Utc::now().naive_utc();
    data.update_at = cur_time;

    diesel::update(ignore::table)
        .filter(ignore_id.eq(&data.ignore_id))
        .set(data.clone())
        .returning(Ignore::as_returning())
        .get_result(conn)
}

/// Get ignore records from database.
/// 
/// # Arguments
/// 
/// * `conn` - Connection to database.
/// * `uuid` - Uuid for procedure, if `None`, get all.
/// 
/// # Examples
/// 
/// ```
/// use db::{establish_sqlite_connection, ignore::query_ignore_record};
/// 
/// if let Ok(mut conn) = establish_sqlite_connection() {
///     match query_ignore_record(&mut conn, None) {
///         Ok(records) => {
///             println!("get all records: {:?}", records);
///         },
///         Err(error) => {
///             println!("failed to get records, errMsg: {:?}", error);
///         }
///     }   
/// }
/// ```
pub fn query_ignore_record(
    conn: &mut SqliteConnection,
    uuid: Option<&str>,
) -> Result<Vec<Ignore>, diesel::result::Error> {
    use super::schema::ignore::dsl::*;

    match uuid {
        Some(uid) => {
            ignore.filter(procedure_id.eq(uid))
            .filter(is_deleted.eq(0))
            .select(Ignore::as_select())
            .load(conn)  
        },
        None => {
            ignore.select(Ignore::as_select())
                    .filter(is_deleted.eq(0))
                    .load(conn) 
        }
    }
}

/// Delete ignore record in database logically.
/// 
/// # Arguments
/// 
/// * `conn` - Connection to database.
/// * `nid` - Uuid for ignore, delete single ignore record.
/// * `pid` - Uuid for procedure, delete all related ignore records.
/// 
/// # Examples
/// 
/// ```
/// use db::{establish_sqlite_connection, ignore::delete_ignore_record};
/// 
/// if let Ok(mut conn) = establish_sqlite_connection() {
///     let pid = "e56da9c2-851e-4cb5-a896-f371f2e3997f";
///     match delete_ignore_record(&mut conn, None, Some(pid)) {
///         Ok(cnt) => {
///             println!("delete {} records", cnt);
///         },
///         Err(error) => {
///             println!("failed to delete records, errMsg: {:?}", error);
///         }
///     }   
/// }
/// ```
pub fn delete_ignore_record(
    conn: &mut SqliteConnection,
    nid: Option<&str>,
    pid: Option<&str>,
) -> Result<usize, diesel::result::Error> {
    use super::schema::ignore::dsl::*;

    if let Some(uuid) = nid {
        return diesel::update(ignore)
                    .filter(ignore_id.eq(uuid))
                    .set((
                        is_deleted.eq(1),
                        delete_at.eq(Utc::now().naive_utc())
                    ))
                    .execute(conn);      
    } else if let Some(uuid) = pid {
        return diesel::update(ignore)
                    .filter(procedure_id.eq(uuid))
                    .set((
                        is_deleted.eq(1),
                        delete_at.eq(Utc::now().naive_utc())
                    ))
                    .execute(conn);    
    }  

    Err(diesel::result::Error::NotFound)
}

/// Clear 'ignore' table records.
/// 
/// # Arguments
/// 
/// * `conn` - Connection to database.
/// 
/// # Examples
/// 
/// ```
/// use db::{establish_sqlite_connection, ignore::clear_ignore_record};
/// 
/// if let Ok(mut conn) = establish_sqlite_connection() {
///     match clear_ignore_record(&mut conn) {
///         Ok(cnt) => {
///             println!("clear table with total {} records", cnt);
///         },
///         Err(error) => {
///             println!("failed to clear records, errMsg: {:?}", error);
///         }
///     }   
/// }
/// ```
pub fn clear_ignore_record(
    conn: &mut SqliteConnection, 
) -> Result<usize, diesel::result::Error> {
    use super::schema::ignore::dsl::*;

    diesel::delete(ignore)
        .execute(conn)
}

/// Clean 'ignore' table records.
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
/// use db::{establish_sqlite_connection, ignore::clean_ignore_record};
/// 
/// if let Ok(mut conn) = establish_sqlite_connection() {
///     match clean_ignore_record(&mut conn) {
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
    use super::schema::ignore::dsl::*;

    let cleaned: usize = diesel::delete(ignore.filter(is_deleted.eq(1))).execute(conn)?;

    let mut remaining: Vec<Ignore> = ignore.select(Ignore::as_select()).load(conn)?;
    for (idx, item) in remaining.iter_mut().enumerate() {
        let new_id = (idx + 1) as i32;
        diesel::update(ignore)
            .filter(ignore_id.eq(&item.ignore_id))
            .set(id.eq(new_id))
            .execute(conn)?;
    }

    Ok(cleaned)   
}

/// Get procedure related ignores.
/// 
/// # Arguments
/// 
/// * `pid` - Target procedure.
/// * `conn` - Connection to database.
/// 
/// # Examples
/// 
/// ```
/// use db::{establish_sqlite_connection, ignore::get_procedure_ignores};
/// 
/// if let Ok(mut conn) = establish_sqlite_connection() {
///     let pid = "e56da9c2-851e-4cb5-a896-f371f2e3997f";
///     match get_procedure_ignores(pid, &mut conn) {
///         Ok(ignores) => {
///             println!("get ignores {:?} for procedure {}", ignores, pid);
///         },
///         Err(error) => {
///             println!("failed to get ignores for procedure {}, errMsg: {:?}", pid, error);
///         }
///     }   
/// }
/// ```
pub fn get_procedure_ignores(pid: &str, conn: &mut SqliteConnection) -> Vec<String> {
    let mut data = Vec::new();
    if let Ok(ignores) = query_ignore_record(conn, Some(pid)) {
        for ignore in ignores.iter() {
            if ignore.is_deleted == 1 {
                continue;
            }
            data.push(ignore.keyword.clone());
        }
    }   

    data
}
