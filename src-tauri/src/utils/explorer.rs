mod explorer {
    //! The `meta` module contains functions about handling meta datas.
    
    /// Removes the whole file or directory.
    /// 
    /// # Arguments
    /// 
    /// * `path` - A string that holds the file or directory path
    /// 
    /// # Examples
    /// 
    /// ```
    /// use meta::remove_all;
    /// 
    /// remove_all("to\\remove\\file").unwrap();
    /// remove_all("to\\remove\\directory").unwrap();
    /// ```
    pub fn remove_all(path: &str) -> Result<(), std::io::Error> {
        use std::path::Path;
        use std::fs::{remove_file, remove_dir_all};
        
        let target = Path::new(path);

        if !target.exists() {
            return Ok(());
        }

        if target.is_file() {
            remove_file(target)?;
        } else if target.is_dir() {
            remove_dir_all(target)?;
        }

        Ok(())
    }

    /// Recursively copies the whole directory.
    /// 
    /// See [How to copy a folder recursively in Rust?](https://stackoverflow.com/questions/26958489/how-to-copy-a-folder-recursively-in-rust)
    /// 
    /// # Arguments
    /// 
    /// * `src` - Path of source directory
    /// * `dst` - Path of save directory
    /// 
    /// # Examples
    /// 
    /// ```
    /// use meta::copy_dir_all;
    /// 
    /// copy_dir_all("to\\copy\\directory", "to\\save\\directory").unwrap();
    /// ```
    fn copy_dir_all(src: impl AsRef<std::path::Path>, dst: impl AsRef<std::path::Path>) -> std::io::Result<()> {
        use std::fs;
        
        fs::create_dir_all(&dst)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let ty = entry.file_type()?;
            if ty.is_dir() {
                copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
            } else {
                fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
            }
        }
        Ok(())
    }

    /// Copies the whole file or directory.
    /// 
    /// # Arguments
    /// 
    /// * `from` - A string that holds the source path of file or directory
    /// * `to` - A string that holds the save path of file or directory
    /// 
    /// # Examples
    /// 
    /// ```
    /// use meta::copy_all;
    /// 
    /// copy_all("to\\copy\\file", "to\\save\\file").unwrap();
    /// copy_all("to\\copy\\directory", "to\\save\\directory").unwrap();
    /// ```
    pub fn copy_all(from: &str, to: &str) -> Result<(), std::io::Error> {
        use std::path::Path;
        use std::io::{Error, ErrorKind};
        use std::fs::{copy, OpenOptions};
        
        let src = Path::new(from);
        let save = Path::new(to);
        if !src.exists() {
            return Err(Error::from(ErrorKind::NotFound));
        }

        if src.is_file() {
            if !save.exists() {
                let _ = OpenOptions::new().write(true)
                            .create_new(true)
                            .open(to);
            }

            copy(from, to)?;
        } else if src.is_dir() {
            copy_dir_all(from, to)?;
        }

        Ok(())
    }

    /// Copies the whole directory with build in .gitignore file.
    /// 
    /// # Arguments
    /// 
    /// * `from` - A string that holds the source path of  directory
    /// * `to` - A string that holds the save path of directory
    /// 
    /// # Examples
    /// 
    /// ```
    /// use meta::copy_dir_with_build_in_ignore;
    /// 
    /// copy_dir_with_build_in_ignore("to\\copy\\directory", "to\\save\\directory").unwrap();
    /// ```
    pub fn copy_dir_with_build_in_ignore(from: &str, to: &str) -> Result<(), std::io::Error> {
        use ignore::Walk;
        use std::path::Path;
        use std::io::{Error, ErrorKind};
        use std::fs::{copy, create_dir_all};

        let src = Path::new(from);
        let save = Path::new(to);

        for result in Walk::new(src) {
            match result {
                Ok(entry) => {
                    let cur_entry_path = entry.path();
                    let entry_save = save.join(cur_entry_path.strip_prefix(from).unwrap_or(src));
                    let entry_save_path = entry_save.as_path();

                    if cur_entry_path.is_file() {
                        let _ = copy(cur_entry_path, entry_save_path)?;
                    } else if cur_entry_path.is_dir() {
                        let _ = create_dir_all(entry_save_path)?;

                        let ignore_file = cur_entry_path.join(".gitignore");
                        if ignore_file.exists() {
                            let _ = copy(ignore_file, entry_save_path.join(".gitignore"))?;
                        }
                    }
                },
                Err(_error) => {
                    // println!("Failed to parse ignore file in dir {}, errMsg: {:?}", from, error);
                    return Err(Error::from(ErrorKind::InvalidData));
                }
            }
        }

        Ok(())
    }

    /// Copies the whole directory with custom ignore strings.
    /// 
    /// # Arguments
    /// 
    /// * `from` - A string that holds the source path of  directory
    /// * `to` - A string that holds the save path of directory
    /// * `ignores` - A vec that contains the custom ignore strings
    /// 
    /// # Examples
    /// 
    /// ```
    /// use meta::copy_dir_with_custom_ignores;
    /// 
    /// let ignores = vec!["debug", "bin", "target"];
    /// copy_dir_with_custom_ignores("to\\copy\\directory", "to\\save\\directory", ignores).unwrap();
    /// ```
    pub fn copy_dir_with_custom_ignores(from: &str, to: &str, ignores: &Vec<String>) -> Result<(), std::io::Error> {
        use walkdir::WalkDir;
        use std::path::Path;
        use std::io::{Error, ErrorKind};
        use std::fs::{copy, create_dir_all};

        let src = Path::new(from);
        let save = Path::new(to);

        for result in WalkDir::new(src) {
            match result {
                Ok(entry) => {
                    let cur_entry_path = entry.path();
                    let entry_save = save.join(cur_entry_path.strip_prefix(from).unwrap_or(src));
                    let entry_save_path = entry_save.as_path();

                    if let Some(path_str) = entry.path().to_str() {
                        if ignores.iter().any(
                            |item| path_str.contains(item.as_str())
                        ) {
                            continue;
                        }
                    } 

                    if cur_entry_path.is_file() {
                        let _ = copy(cur_entry_path, entry_save_path)?;
                    } else if cur_entry_path.is_dir() {
                        let _ = create_dir_all(entry_save_path)?;
                    }
                },
                Err(_error) => {
                    // println!("Failed to parse ignore file in dir {}, errMsg: {:?}", from, error);
                    return Err(Error::from(ErrorKind::InvalidData));
                }
            }
        }

        Ok(())
    }

    /// Restricts the directory to wanted items count.
    /// 
    /// Will delete the ealiest created items.
    /// 
    /// # Arguments
    /// 
    /// * `path` - A string that holds the source path of directory
    /// * `count` - Wanted directory items count
    /// 
    /// # Examples
    /// 
    /// ```
    /// use meta::restrict_dir_items_by_count;
    /// 
    /// restrict_dir_items_by_count("to\\restrict\\directory", 3).unwrap();
    /// ```
    pub fn restrict_dir_items_by_count(path: &str, count: usize) -> Result<(), std::io::Error> {
        use std::path::Path;
        use std::io::{Error, ErrorKind};
        use std::time::UNIX_EPOCH;
        use std::fs::{metadata, read_dir};
    
        let target = Path::new(path);
        if !target.exists() {
            return Err(Error::from(ErrorKind::NotFound));
        }
    
        let mut dir_items: Vec<String> = vec![];
        for result in read_dir(path)? {
            match result {
                Ok(entry) => {
                    let cur_entry_path = entry.path().display().to_string();
                    dir_items.push(cur_entry_path);

                    // let cur_created_time = metadata(&cur_entry_path)?.created()?;
                    // println!("insert path: {:?}, time: {:?}", entry.path().display(), cur_created_time);
                },
                Err(error) => {
                    println!("Failed to read dir {}, errMsg: {:?}", path, error);
                    return Err(Error::from(ErrorKind::Other)); 
                }
            }
        }
    
        if dir_items.len() > count {
            dir_items.sort_by(|a, b|
                metadata(a).unwrap().created().unwrap().duration_since(UNIX_EPOCH).unwrap().cmp(
                    &metadata(b).unwrap().created().unwrap().duration_since(UNIX_EPOCH).unwrap()
                )
            );

            for path in dir_items[..(dir_items.len() - count)].iter() {
                let _ = remove_all(path)?;
            }   
        }
    
        Ok(())
    }

    /// Restricts the directory to wanted size.
    /// 
    /// This will not recursively walk the children directory!
    /// 
    /// # Arguments
    /// 
    /// * `path` - A string that holds the source path of directory
    /// * `size` - Wanted directory size in bytes
    /// 
    /// # Examples
    /// 
    /// ```
    /// use meta::restrict_dir_items_by_size;
    /// 
    /// restrict_dir_items_by_size("to\\restrict\\directory", 3 * 1024).unwrap();
    /// ```
    pub fn restrict_dir_items_by_size(path: &str, size: u64) -> Result<(), std::io::Error> {
        use std::path::Path;
        use std::io::{Error, ErrorKind};
        use std::fs::{metadata, read_dir};
        use fs_extra::dir::get_size;
    
        let target = Path::new(path);
        if !target.exists() {
            return Err(Error::from(ErrorKind::NotFound));
        }
    
        for result in read_dir(path)? {
            match result {
                Ok(entry) => {
                    if let Ok(dir_size) = get_size(path) {
                        let mut item_size: u64 = 0;
                        if entry.path().is_file() {
                            item_size = metadata(entry.path())?.len();
                        } else if entry.path().is_dir() {
                            item_size = get_size(entry.path()).unwrap_or(0);
                        }

                        if (dir_size - item_size) > size {
                            let _ = remove_all(entry.path().display().to_string().as_str())?;
                        }
                    }
                },
                Err(error) => {
                    println!("Failed to read dir {}, errMsg: {:?}", path, error);
                    return Err(Error::from(ErrorKind::Other)); 
                }
            }
        }
    
        Ok(())
    }
}