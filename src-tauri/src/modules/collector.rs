/**
 * Get mission save path's backups' count and size
 * Note: This is specific for the programe
 */
use std::{fs::{read_dir, metadata}, path::PathBuf};
use chrono::{ NaiveDate, Duration};
use byte_unit::{Byte, ByteUnit};
use std::collections::HashMap;
use fs_extra::dir::get_size;
use std::str::FromStr;
use log::warn;

// individual

// accept date like yyyy-mm-dd
pub fn get_one_day_analyse(path: &str, date: &str, individual: bool) -> HashMap<String, Vec<u64>> {
    let mut res: HashMap<String, Vec<u64>> = HashMap::new();

    let mut arr_count: [u64; 12] = [0; 12];
    let mut arr_size: [u64; 12] = [0; 12];

    let date_path = PathBuf::from(path).join(date);
    if !date_path.exists() {
        res.insert("count".to_string(), arr_count.to_vec());
        res.insert("size".to_string(), arr_size.to_vec());

        warn!("Date {} has no backups", date_path.display());

        return res;
    }

    let target_paths = read_dir(date_path.to_str().unwrap()).unwrap();
    
    if date_path.exists() {
        for content in target_paths {
            let content_path = content.unwrap().path();
            let hour_time = &content_path.file_name().unwrap().to_str().unwrap().to_string()[..2];
            let hour = u8::from_str(hour_time).unwrap();

            let mut content_size: u64 = 0;
            if content_path.is_file() {
                content_size = metadata(content_path.to_str().unwrap()).unwrap().len();
            } else if content_path.is_dir() {
                content_size = fs_extra::dir::get_size(content_path.to_str().unwrap()).unwrap();
            }

            // https://stackoverflow.com/questions/46102811/why-i-can-not-use-u8-as-an-index-value-of-a-rust-array
            // 统计 2点前 4点前....
            let pos: u8 = if hour % 2 == 0 {hour / 2} else {(hour - 1) / 2};

            if individual {
                arr_count[pos as usize] += 1;
                arr_size[pos as usize] += content_size;
            } else {
                for j in pos..12 {
                    arr_count[j as usize] += 1;
                    arr_size[j as usize] += content_size;
                }          
            }
        }

    }

    res.insert("count".to_string(), arr_count.to_vec());
    res.insert("size".to_string(), arr_size.to_vec());

    // println!("final arr {:?}", &res);

    res
}

// accept week like yyyy-mm-dd, auto compute week stop date
pub fn get_one_week_analyse(path: &str, start_date: &str, individual: bool) -> HashMap<String, Vec<u64>> {
    let mut res: HashMap<String, Vec<u64>> = HashMap::new();

    let mut arr_count: [u64; 7] = [0; 7];
    let mut arr_size: [u64; 7] = [0; 7];

    for i in 0..7 {
        let current_date = NaiveDate::parse_from_str(start_date, "%Y-%m-%d").unwrap() + Duration::days(i);
        let current_date_path = PathBuf::from(path).join(current_date.to_string());
        if !current_date_path.exists() {continue;}
        
        let current_date_path_count = read_dir(current_date_path.to_str().unwrap()).unwrap().count();
        let current_date_path_size = get_size(current_date_path.to_str().unwrap()).unwrap();
    
        if individual {
            arr_count[i as usize] = current_date_path_count as u64;
            arr_size[i as usize] = current_date_path_size;
        } else {
            for j in i..7 {
                arr_count[j as usize] += current_date_path_count as u64;
                arr_size[j as usize] += current_date_path_size;
            }             
        }

    }

    res.insert("count".to_string(), arr_count.to_vec());
    res.insert("size".to_string(), arr_size.to_vec());

    // println!("final arr {:?}", &res);

    res
}

fn get_days_from_month(year: i32, month: u32) -> i64 {
    NaiveDate::from_ymd(
        match month {
            12 => year + 1,
            _ => year,
        },
        match month {
            12 => 1,
            _ => month + 1,
        },
        1
    )
    .signed_duration_since(NaiveDate::from_ymd(year, month, 1))
    .num_days()
}

// accept month like yyyy-mm
pub fn get_one_month_analyse(path: &str, month: &str, individual: bool) -> HashMap<String, Vec<u64>> {
    let mut res: HashMap<String, Vec<u64>> = HashMap::new();

    let string_month = month.to_string();
    let split_date: Vec<&str> = string_month.split('-').collect();
    let month_days = get_days_from_month(split_date[0].to_string().parse::<i32>().unwrap(), split_date[1].to_string().parse::<u32>().unwrap());
    let start_date = month.clone().to_string() + "-01";

    let mut arr_count: Vec<u64> = vec![0; month_days as usize];
    let mut arr_size: Vec<u64> = vec![0; month_days as usize];

    for i in 0..month_days {
        let current_date = NaiveDate::parse_from_str(&start_date, "%Y-%m-%d").unwrap() + chrono::Duration::days(i);
        let current_date_path = PathBuf::from(path).join(current_date.to_string());
        if !current_date_path.exists() {continue;}
        
        let current_date_path_count = read_dir(current_date_path.to_str().unwrap()).unwrap().count();
        let current_date_path_size = get_size(current_date_path.to_str().unwrap()).unwrap();

        if individual {
            arr_count[i as usize] = current_date_path_count as u64;
            arr_size[i as usize] = current_date_path_size;
        } else {
            for j in i..month_days {
                arr_count[j as usize] += current_date_path_count as u64;
                arr_size[j as usize] += current_date_path_size;
            }      
        }
    }

    res.insert("count".to_string(), arr_count);
    res.insert("size".to_string(), arr_size);

    // println!("final arr {:?}", &res);

    res
}

// accept year like yyyy
pub fn get_one_year_analyse(path: &str, year: &str, individual: bool) -> HashMap<String, Vec<u64>> {
    let mut res: HashMap<String, Vec<u64>> = HashMap::new();

    let mut arr_count: [u64; 12] = [0; 12];
    let mut arr_size: [u64; 12] = [0; 12];

    if PathBuf::from(path).exists() {
        for i in 0..12 {
            let current_month = year.to_string() + "-" + &format!("{:02}", i + 1).to_string();

            if !PathBuf::from(path).exists() {continue;}
            let save_contents = read_dir(path).unwrap();

            let mut month_count: u64 = 0;
            let mut month_size: u64 = 0;

            for item in save_contents {
                let current_path = item.unwrap();
                let current_path_name = current_path.file_name();

                if current_path_name.to_str().unwrap().contains(&current_month) {
                    month_count += read_dir(current_path.path().to_str().unwrap()).unwrap().count() as u64;
                    month_size += get_size(current_path.path()).unwrap();
                }
            }            

            if individual {
                arr_count[i as usize] = month_count as u64;
                arr_size[i as usize] = month_size;
            } else {
                for j in i..12 {
                    arr_count[j as usize] += month_count;
                    arr_size[j as usize] += month_size;
                }  
            }
        }

    }

    res.insert("count".to_string(), arr_count.to_vec());
    res.insert("size".to_string(), arr_size.to_vec());

    // println!("final arr {:?}", &res);

    res
}

fn convert_count_unit(origin_count_arr: Vec<u64>) -> Vec<f64> {
    let mut converted_arr: Vec<f64> = Vec::new();

    for item in origin_count_arr {
        converted_arr.push((item as u64) as f64);
    }

    converted_arr
}

fn convert_size_unit(origin_size_arr: Vec<u64>, size_unit: &str) -> Vec<f64> {
    let mut converted_arr: Vec<f64> = Vec::new();

    for item in origin_size_arr {
        let origin_byte = Byte::from_bytes(item.into());
        let converted_string_size: String = match size_unit {
            "Kb" => {
                origin_byte.get_adjusted_unit(ByteUnit::KB).to_string()
            },

            "Kib" => {
                origin_byte.get_adjusted_unit(ByteUnit::KiB).to_string()
            },

            "Mb" => {
                origin_byte.get_adjusted_unit(ByteUnit::MB).to_string()
            }

            "Mib" => {
                origin_byte.get_adjusted_unit(ByteUnit::MiB).to_string()
            },

            _ => { origin_byte.get_adjusted_unit(ByteUnit::B).to_string() }
        };
        let blank_space_pos = converted_string_size.find(" ").unwrap();
        let remove_unit = converted_string_size[0..blank_space_pos].to_string();
        
        converted_arr.push(remove_unit.parse::<f64>().unwrap());
    }
    converted_arr
}

// Get path count and size with datetime
// Note: This is specific for this program
pub fn count_size_with_path(path: &str, query_type: &str, start: &str, size_unit: &str) -> HashMap<String, Vec<f64>> {
    let analyse_res = match query_type {
        "date" => {
            get_one_day_analyse(path, start, true)
        },
        "week" => {
            get_one_week_analyse(path, start, true)
        },
        "month" => {
            get_one_month_analyse(path, start, true)
        },
        "year" => {
            get_one_year_analyse(path, start, true)
        },
        _ => {
            let mut empty_res: HashMap<String, Vec<u64>> = HashMap::new();
            empty_res.insert("count".to_string(), [].to_vec());
            empty_res.insert("size".to_string(), [].to_vec());

            empty_res
        }
    };

    let convert_count_arr = convert_count_unit(analyse_res.get("count").unwrap().to_vec());
    let convert_size_arr = convert_size_unit(analyse_res.get("size").unwrap().to_vec(), size_unit);
    // println!("{:?}", analyse_res);

    let mut converted_res: HashMap<String, Vec<f64>> = HashMap::new();
    converted_res.insert("count".to_string(), convert_count_arr);
    converted_res.insert("size".to_string(), convert_size_arr);

    converted_res
}
