use std::{path::{Path, PathBuf}, fs::{metadata, copy, read_dir, remove_dir_all, create_dir}};
use fs_extra::file::{CopyOptions as FileCopyOptions};
use byte_unit::{Byte, ByteUnit};
use std::io::ErrorKind;
use log::{debug, warn, error};
use walkdir::WalkDir;
use ignore::Walk;               

/**
 * Remove target whether is file or dir
 */
pub fn remove_target(target: &str) -> bool {
    let target_path = PathBuf::from(target);

    let mut remove_res: bool = false;
    if metadata(&target_path).unwrap().is_dir() {
        if let Err(e) = remove_dir_all(&target_path) {
            error!("Failed to remove {}, errMsg: {:?}", &target, e);
            remove_res = false;
        } else {
            remove_res = true;
        }
    } else if metadata(&target_path).unwrap().is_file() {
        if let Err(e) = fs_extra::file::remove(&target_path) {
            error!("Failed to remove {}, errMsg: {:?}", &target, e);
            remove_res = false;
        } else {
            remove_res = true;
        }
    }      

    return remove_res;
}

/**
 * Copy target with no ignores
 */
pub fn copy_target_with_no_ignores(from: &str, to: &str) -> bool {
    let from_path = PathBuf::from(from);
    let save_path = PathBuf::from(to);

    if !from_path.exists() || !save_path.exists() {
        return false;
    }

    let mut res: bool = false;
    if metadata(&from_path).unwrap().is_dir() {
        if let Err(e) = create_dir(&save_path) {
            match e.kind() {
                ErrorKind::AlreadyExists => {}
                _ => {
                    error!("Failed to create {}", from_path.display());
                    return false
                }
            }            
        }

        res = copy_target_with_ignores(&from_path.to_str().unwrap(), &save_path.to_str().unwrap(), false, &Vec::new());
    } else if metadata(&from_path).unwrap().is_file() {
        let target_name = from_path.file_name().unwrap().to_str().unwrap();
        let actual_save_path = save_path.join(target_name);

        let mut options = FileCopyOptions::new();
        options.skip_exist = true;

        if let Err(e) = fs_extra::file::copy(&from_path, &actual_save_path, &options) {
            error!("Failed to copy {}, errMsg: {:?}", from_path.display(), e);
            res = false;
        } else {
            res = true;
        }
    }   

    return res;
}

/**
 * Copy target with ignores
 */
pub fn copy_target_with_ignores(from: &str, to: &str, git: bool, ignores: &Vec<String>) -> bool {
    let from_path = PathBuf::from(from);
    let save_path = PathBuf::from(to);

    if !from_path.exists() || !save_path.exists() {
        return false;
    }
    
    let mut res: bool = false;
    if metadata(&from_path).unwrap().is_dir() {
        if git {
            res = copy_folder_with_ignore_file(from, to);
        } else {
            res = copy_folder_with_custom_ignores(from, to, ignores);
        }

        return res;
    } else if metadata(&from_path).unwrap().is_file() {
        let mut options = FileCopyOptions::new();
        options.skip_exist = true;

        if let Err(e) = fs_extra::file::copy(&from_path, &save_path, &options) {
            error!("Failed to copy {}, errMsg: {:?}", &from, e);
            res = false;
        } else {
            res = true;
        }
    }   
    
    return res;
}

/**
 * Copy folder with .gitignore, will ignores items in .gitignore file
 */
pub fn copy_folder_with_ignore_file(from: &str, to: &str) -> bool {
    let from_path = PathBuf::from(from);
    let save_path = PathBuf::from(to);

    for result in Walk::new(from) {
        match result {
            Ok(entry) => {
                let current_entry_path = entry.path();
                let entry_save_path = save_path.join(current_entry_path.strip_prefix(&from_path).unwrap());
                let debug_entry_save_path = entry_save_path.clone();

                if metadata(current_entry_path.to_str().unwrap()).unwrap().is_dir() {
                    if let Err(e) = create_dir(entry_save_path) {
                        match e.kind() {
                            ErrorKind::AlreadyExists => {}
                            _ => return false,
                        }
                    }

                    // copy .gitignore file
                    if Path::new(current_entry_path).join(".gitignore").exists() {
                        if let Err(e) = copy(current_entry_path.join(".gitignore").to_str().unwrap(), debug_entry_save_path.join(".gitignore").to_str().unwrap()) {
                            match e.kind() {
                                ErrorKind::AlreadyExists => {}
                                _ => return false,
                            }
                        }
                    }                    
                } else if metadata(current_entry_path.to_str().unwrap()).unwrap().is_file() {
                    copy(current_entry_path, entry_save_path).unwrap();
                }

                // debug!("copy {} => {}", current_entry_path.display(), debug_entry_save_path.display());
            },
            Err(_err) => {
                return false;
            }
        }
    }

    return true;
}

/**
 * Copy folder with custom ignores, will ignores items in arr
 * https://stackoverflow.com/questions/26958489/how-to-copy-a-folder-recursively-in-rust
 */
pub fn copy_folder_with_custom_ignores(from: &str, to: &str, ignores: &Vec<String>) -> bool {
    let from_path = PathBuf::from(from);
    let save_path = PathBuf::from(to);

    for result in WalkDir::new(from) {
        match result {
            Ok(entry) => {
                let current_item_path = entry.path();
                let item_save_path = save_path.join(current_item_path.strip_prefix(&from_path).unwrap());
                
                let string_current_item_path = entry.path().to_str().unwrap();
                if ignores.iter().any(|item| string_current_item_path.contains(item.as_str())) {
                    continue;
                };
                if metadata(current_item_path.to_str().unwrap()).unwrap().is_dir() {
                    if let Err(e) = create_dir(to) {
                        match e.kind() {
                            ErrorKind::AlreadyExists => {}
                            _ => return false,
                        }
                    }
                } else if metadata(current_item_path.to_str().unwrap()).unwrap().is_file() {
                    copy(current_item_path, item_save_path).unwrap();
                } 
                // else {
                //     debug!("ignore path {}", current_item_path.display());
                // }
            },
            Err(_err) => {
                return false;
            }
        }
    }

    return true;
}


// fn my_remove(path: &str) -> bool {
//     let remove_res = remove(&path);
//     match remove_res {
//         Ok(_res) => {
//             info!("remove {} success", &path); 
//             return true;
//         },
//         Err(err) => {
//             error!("remove {} failed, errMsg: {}", &path, err); 

//             return false;                                
//         }
//     }           
// }

/**
 * recursivly remove content in dir until under set size
 */
// pub fn restrict_dir_to_size(path: &str, size: u64, top_path: &str) -> bool {
//     for entry in WalkDir::new(path) {
//         let entry = entry.unwrap();

//         // 跳过自身
//         if entry.path() == Path::new(path) {
//             info!("get path {}", &path);
//             continue;
//         }

//         if entry.path().is_dir() {
//             // info!("path {} is dir", &entry.path().display());
//             let after_delete_size = get_dir_size(&top_path) - get_dir_size(&entry.path().to_str().unwrap());
//             if after_delete_size > size {
//                 if my_remove(&entry.path().to_str().unwrap()) {
//                     // info!("trigger");
//                 } else {
//                     error!("restrict path {} size failed", path); 
//                 }           
//             } else {
//                 restrict_dir_to_size(&entry.path().to_str().unwrap(), size, top_path);
//             }
//         }     
//     }

//     true
// }

/**
 * Remove subfolder until equal or below set cout
 */
pub fn restrict_save_path_day_count(path: &str, count: u64) -> bool {
    let origin_days = read_dir(path).unwrap();
    for date_path in origin_days {
        let current_date = date_path.unwrap().path();
        let current_count = read_dir(path).unwrap().count();

        if (current_count - 1) as u64 >= count {
            if !remove_target(current_date.to_str().unwrap()) {
                warn!("Failed to remove {}", current_date.to_str().unwrap());
                return false;
            }
        } else {
            break;
        }
    }

    let finial_count = read_dir(path).unwrap().count();
    debug!("restrict path {} count to {}", path, finial_count);

    true
}

/**
 * Remove subfolder's subcontent until equal or below set size(bytes)
 */
pub fn restrict_save_path_size(path: &str, size: u64) -> bool {
    let date_times = read_dir(path).unwrap();
    for date_path in date_times {
        let current_date = date_path.unwrap().path();
        let current_date_path_size = fs_extra::dir::get_size(current_date.to_str().unwrap()).unwrap();
        let current_save_path_size = fs_extra::dir::get_size(path).unwrap();
    
        if (current_save_path_size - current_date_path_size) >= size {
            if !remove_target(current_date.to_str().unwrap()) {
                warn!("Failed to remove {}", current_date.to_str().unwrap());
                return false;
            }
        } else {
            let day_times = read_dir(current_date.to_str().unwrap()).unwrap();

            for day_time in day_times {
                let current_time = day_time.unwrap().path();
                let latest_save_path_size = fs_extra::dir::get_size(path).unwrap();

                // let mut current_time_size: u64 = 0;
                // if current_time.is_file() {
                //     current_time_size = metadata(current_time.to_str().unwrap()).unwrap().len();
                // } else if current_time.is_dir() {
                //     current_time_size = fs_extra::dir::get_size(current_time.to_str().unwrap()).unwrap();
                // }

                if latest_save_path_size >= size {
                    if !remove_target(current_time.to_str().unwrap()) {
                        warn!("Failed to remove {}", current_time.to_str().unwrap());
                        return false;
                    }                
                } else {
                    continue;
                }
            }
        }
    }

    let finial_save_path_size = fs_extra::dir::get_size(path).unwrap();
    let origin_byte = Byte::from_bytes(finial_save_path_size.into());
    let converted_string_size: String = origin_byte.get_adjusted_unit(ByteUnit::MiB).to_string();
    debug!("restrict path {} size to {}", path, converted_string_size);

    true
}
