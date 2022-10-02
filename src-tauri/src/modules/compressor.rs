/**
 * Compress folder or file to zip/tar/gz..
 * Not support encrypted yet
 */
// 暂不支持加密
use log::{error, info};
// use super::explorer::{_is_file, _is_dir};


// zip bzip related
use std::io::prelude::*;
use std::io::{Seek, Write};
use std::iter::Iterator;
// use zip::result::ZipError;
use zip::write::FileOptions;

use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

//tar related
use std::io::Error;
// use tar::Builder;

// gz related
use flate2::write::GzEncoder;

// tar.gz
// use flate2::write::GzEncoder;

// tar.bz2
use bzip2::write::BzEncoder;

// xz
use xz2::write::XzEncoder;

// zip https://github.com/zip-rs/zip
// rar
// tar https://github.com/alexcrichton/tar-rs
// gzip(flate2) https://github.com/rust-lang/flate2-rs
// bzip https://github.com/alexcrichton/bzip2-rs
// xz https://github.com/alexcrichton/xz2-rs

pub fn create_archive(src_path: &str, dst_path: &str, format: &str) -> bool {
    let check_path = PathBuf::from(src_path);

    if !Path::new(&check_path.to_str().unwrap()).exists() {
        error!("Create archive failed, errMsg: soure target {} not exists!", &src_path);
        return false;        
    }

    let mut create_res: bool = false;
    match format {
        "zip" => {
            create_res = create_zip_archive(src_path, dst_path);
        },
        "rar" => {
            info!("rar");
        },
        "tar" => {
            create_res = create_tar_archive(src_path, dst_path);
        },
        // "gz" => {
        //     info!("bz");
        // },
        // "bz2" => {
        //     creata_bz2_archive(src_path, dst_path);
        // },
        // "xz" => {
        //     info!("xz");
        // },
        "tar.gz" => {
            create_res = create_tar_gz_archive(src_path, dst_path);
        },
        "tar.bz2" => {
            create_res = create_tar_bz2_archive(src_path, dst_path);
        },
        "tar.xz" => {
            create_res = create_tar_xz_archive(src_path, dst_path);
        }
        _ => {
            error!("Create archive failed, errMsg: invalid format, supports only zip/tar/tar.gz/tar.bz2/tar.xz!");
            return false;
        }
    }

    return create_res;
}

fn zip_dir<T>(
    it: &mut dyn Iterator<Item = DirEntry>,
    prefix: &str,
    writer: T,
    method: zip::CompressionMethod,
) -> zip::result::ZipResult<()>
where
    T: Write + Seek,
{
    let mut zip = zip::ZipWriter::new(writer);
    let options = FileOptions::default()
        .compression_method(method)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(Path::new(prefix)).unwrap();

        // Write file or directory explicitly
        // Some unzip tools unzip files with directory paths correctly, some do not!
        if path.is_file() {
            // info!("adding file {:?} as {:?} ...", path, name);
            #[allow(deprecated)]
            zip.start_file_from_path(name, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&*buffer)?;
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            // Only if not root! Avoids path spec / warning
            // and mapname conversion failed error on unzip
            // info!("adding dir {:?} as {:?} ...", path, name);
            #[allow(deprecated)]
            zip.add_directory_from_path(name, options)?;
        }
    }
    zip.finish()?;
    Result::Ok(())
}

fn create_zip_archive(src_path: &str, dst_path: &str) -> bool {
    let path = Path::new(dst_path);
    let file = File::create(&path).unwrap();

    let walkdir = WalkDir::new(src_path);
    let it = walkdir.into_iter();

    let res: zip::result::ZipResult<()>; 
    
    let from_path = PathBuf::from(src_path);
    if fs::metadata(from_path.to_str().unwrap()).unwrap().is_file() {
        let prefix = Path::new(src_path).parent().map_or_else(||"/", |p|p.to_str().unwrap());
        res = zip_dir(&mut it.filter_map(|e| e.ok()), prefix, file, zip::CompressionMethod::Deflated);
    } else {
        res = zip_dir(&mut it.filter_map(|e| e.ok()), src_path, file, zip::CompressionMethod::Deflated);
    }

    match res {
        Ok(_res) => {
            return true;
        },
        Err(err) => {
           error!("Compress {} to {} in format zip failed, errMsg: {:?}", src_path, dst_path, err); 
           return false;
        }
    }
}

// fn create_rar_archive(src_path: &str, dst_path: &str) {
//     // not available now, the crate only support decompression
// }

fn create_tar_archive(src_path: &str, dst_path: &str) -> bool {
    let file = File::create(dst_path).unwrap();
    let mut tar_builder = tar::Builder::new(file);

    let res: Result<(), Error>;
    let from_path = PathBuf::from(src_path);
    if fs::metadata(from_path.to_str().unwrap()).unwrap().is_file() {
        res = tar_builder.append_file(Path::new(src_path).file_name().unwrap(), &mut File::open(src_path).unwrap());
    } else {
        res = tar_builder.append_dir_all(Path::new(src_path).file_name().unwrap(), src_path);
    }

    match res {
        Ok(_res) => {
            return true;
        },
        Err(err) => {
           error!("Compress {} to {} in format tar failed, errMsg: {:?}", src_path, dst_path, err); 
           return false;
        }
    }
}

// fn creata_gz_archive(src_path: &str, dst_path: &str) -> bool {
//     true
// }

// fn creata_bz2_archive(src_path: &str, dst_path: &str) -> bool {
//     let path = Path::new(dst_path);
//     let file = File::create(&path).unwrap();

//     let walkdir = WalkDir::new(src_path);
//     let it = walkdir.into_iter();

//     let res: zip::result::ZipResult<()>; 
//     let from_path = PathBuf::from(src_path);
//     if fs::metadata(from_path.to_str().unwrap()).unwrap().is_file() {
//         let prefix = Path::new(src_path).parent().map_or_else(||"/", |p|p.to_str().unwrap());
//         res = zip_dir(&mut it.filter_map(|e| e.ok()), prefix, file, zip::CompressionMethod::Bzip2);
//     } else {
//         res = zip_dir(&mut it.filter_map(|e| e.ok()), src_path, file, zip::CompressionMethod::Bzip2);
//     }

//     match res {
//         Ok(_res) => {
//             return true;
//         },
//         Err(err) => {
//             error!("Compress {} to {} in format tar failed, errMsg: {:?}", src_path, dst_path, err); 
//            return false;
//         }
//     }    
// }

// fn creata_xz_archive(src_path: &str, dst_path: &str) -> bool {
//     true
// }

fn create_tar_gz_archive(src_path: &str, dst_path: &str) -> bool {
    let tar = File::create(dst_path).unwrap();
    let enc = GzEncoder::new(tar, flate2::Compression::default());
    let mut tar_builder = tar::Builder::new(enc);

    let res: Result<(), Error>;
    let from_path = PathBuf::from(src_path);
    if fs::metadata(from_path.to_str().unwrap()).unwrap().is_file() {
        res = tar_builder.append_file(Path::new(src_path).file_name().unwrap(), &mut File::open(src_path).unwrap());
    } else {
        res = tar_builder.append_dir_all("", src_path);
    }

    match res {
        Ok(_res) => {
            return true;
        },
        Err(err) => {
            error!("Compress {} to {} in format tar failed, errMsg: {:?}", src_path, dst_path, err); 
           return false;
        }
    }
}

fn create_tar_bz2_archive(src_path: &str, dst_path: &str) -> bool {
    let tar = File::create(dst_path).unwrap();
    let enc = BzEncoder::new(tar, bzip2::Compression::best());
    let mut tar_builder = tar::Builder::new(enc);

    let res: Result<(), Error>;
    let from_path = PathBuf::from(src_path);
    if fs::metadata(from_path.to_str().unwrap()).unwrap().is_file() {
        res = tar_builder.append_file(Path::new(src_path).file_name().unwrap(), &mut File::open(src_path).unwrap());
    } else {
        res = tar_builder.append_dir_all("", src_path);
    }

    match res {
        Ok(_res) => {
            return true;
        },
        Err(err) => {
            error!("Compress {} to {} in format tar failed, errMsg: {:?}", src_path, dst_path, err); 
           return false;
        }
    }
}

fn create_tar_xz_archive(src_path: &str, dst_path: &str) -> bool {
    let tar = File::create(dst_path).unwrap();
    let enc = XzEncoder::new(tar, 6);
    let mut tar_builder = tar::Builder::new(enc);

    let res: Result<(), Error>;
    let from_path = PathBuf::from(src_path);
    if fs::metadata(from_path.to_str().unwrap()).unwrap().is_file() {
        res = tar_builder.append_file(Path::new(src_path).file_name().unwrap(), &mut File::open(src_path).unwrap());
    } else {
        res = tar_builder.append_dir_all("", src_path);
    }

    match res {
        Ok(_res) => {
            return true;
        },
        Err(err) => {
            error!("Compress {} to {} in format tar failed, errMsg: {:?}", src_path, dst_path, err); 
           return false;
        }
    }
}

