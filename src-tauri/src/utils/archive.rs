use std::fs::{self, File, create_dir_all};
use std::io::{Error, ErrorKind, Read, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use zip::write::SimpleFileOptions;
use flate2::write::GzEncoder;
use xz2::write::XzEncoder;
use bzip2::write::BzEncoder;

/// Configuration for archive operations
#[derive(Debug, Clone)]
pub struct ArchiveOptions {
    /// Compression level (0-9)
    pub compression_level: u32,
    /// Whether to preserve file permissions
    pub preserve_permissions: bool,
    /// Whether to follow symbolic links
    pub follow_links: bool,
}

impl Default for ArchiveOptions {
    fn default() -> Self {
        Self {
            compression_level: 6,
            preserve_permissions: true,
            follow_links: false,
        }
    }
}

/// Prepares paths for archiving operation
///
/// # Arguments
/// * `from` - Source path
/// * `to` - Destination archive path
///
/// # Returns
/// * `Ok(String)` - Prepared source path
/// * `Err(std::io::Error)` if preparation fails
#[allow(dead_code)]
fn prepare_packing(from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<PathBuf, Error> {
    let src_path = from.as_ref();
    let save_path = to.as_ref();

    if !src_path.exists() {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!("Source path does not exist: {}", src_path.display())
        ));
    }

    if save_path.exists() {
        return Err(Error::new(
            ErrorKind::AlreadyExists,
            format!("Destination path already exists: {}", save_path.display())
        ));
    }

    if save_path.starts_with(src_path) {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Destination cannot be inside source"
        ));
    }

    // Create parent directories for destination
    if let Some(parent) = save_path.parent() {
        create_dir_all(parent)?;
    }

    // For single files, create a temporary directory
    if src_path.is_file() {
        let temp_dir = save_path.parent().unwrap().join("packing");
        create_dir_all(&temp_dir)?;
        let temp_file = temp_dir.join(
            src_path.file_name().ok_or_else(|| Error::new(
                ErrorKind::InvalidInput,
                "Invalid source filename"
            ))?
        );
        fs::copy(src_path, &temp_file)?;
        Ok(temp_dir)
    } else {
        Ok(src_path.to_path_buf())
    }
}

/// Cleans up temporary files after archiving
///
/// # Arguments
/// * `from` - Original source path
/// * `to` - Destination archive path
///
/// # Returns
/// * `Ok(())` if cleanup was successful
/// * `Err(std::io::Error)` if cleanup fails
#[allow(dead_code)]
fn finish_packing(from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<(), Error> {
    let src_path = from.as_ref();
    let save_path = to.as_ref();

    if src_path.is_file() {
        if let Some(parent) = save_path.parent() {
            fs::remove_dir_all(parent.join("packing"))?;
        }
    }

    Ok(())
}

/// Creates a ZIP archive from files or directories
///
/// # Arguments
/// * `from` - Source path (file or directory)
/// * `to` - Destination ZIP file path
/// * `options` - Archive configuration options
///
/// # Returns
/// * `Ok(())` if archive was created successfully
/// * `Err(std::io::Error)` if archive creation fails
#[allow(dead_code)]
pub fn create_zip_archive(
    from: impl AsRef<Path>,
    to: impl AsRef<Path>,
    method: zip::CompressionMethod,
    options: &ArchiveOptions,
) -> Result<(), Error> {
    let prepared_path = prepare_packing(&from, &to)?;
    let writer = File::create(to.as_ref())?;
    
    zip_dir(
        &mut WalkDir::new(&prepared_path)
            .follow_links(options.follow_links)
            .into_iter()
            .filter_map(Result::ok),
        &prepared_path,
        writer,
        method,
        options,
    )?;

    finish_packing(from, to)
}

/// Internal function to create ZIP archive
#[allow(dead_code)]
fn zip_dir<T>(
    it: &mut dyn Iterator<Item = walkdir::DirEntry>,
    prefix: &Path,
    writer: T,
    method: zip::CompressionMethod,
    options: &ArchiveOptions,
) -> zip::result::ZipResult<()>
where
    T: Write + std::io::Seek,
{
    let mut zip = zip::ZipWriter::new(writer);
    let zip_options = SimpleFileOptions::default()
        .compression_method(method)
        .unix_permissions(if options.preserve_permissions { 0o755 } else { 0o644 });

    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(prefix).unwrap();

        if path.is_file() {
            zip.start_file_from_path(name, zip_options)?;
            let mut f = File::open(path)?;
            f.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            zip.add_directory_from_path(name, zip_options)?;
        }
    }
    
    zip.finish()?;
    Ok(())
}

/// Supported compression formats
#[derive(Debug, Clone, Copy)]
pub enum CompressionFormat {
    None,
    Gzip,
    Bzip2,
    Xz,
    #[allow(dead_code)]
    SevenZip,
}

/// Creates a tar archive (optionally compressed)
///
/// # Arguments
/// * `from` - Source path
/// * `to` - Destination archive path
/// * `compression` - Compression format to use
/// * `options` - Archive configuration options
///
/// # Returns
/// * `Ok(())` if archive was created successfully
/// * `Err(std::io::Error)` if archive creation fails
#[allow(dead_code)]
pub fn create_compressed_tar(
    from: impl AsRef<Path>,
    to: impl AsRef<Path>,
    compression: CompressionFormat,
    options: &ArchiveOptions,
) -> Result<(), Error> {
    let prepared_path = prepare_packing(&from, &to)?;
    let file = File::create(to.as_ref())?;

    let result = match compression {
        CompressionFormat::Gzip => {
            let encoder = GzEncoder::new(file, flate2::Compression::new(options.compression_level));
            create_tar_archive(&prepared_path, encoder)
        },
        CompressionFormat::Bzip2 => {
            let encoder = BzEncoder::new(file, bzip2::Compression::new(options.compression_level as u32));
            create_tar_archive(&prepared_path, encoder)
        },
        CompressionFormat::Xz => {
            let encoder = XzEncoder::new(file, options.compression_level);
            create_tar_archive(&prepared_path, encoder)
        },
        CompressionFormat::None => {
            create_tar_archive(&prepared_path, file)
        },
        CompressionFormat::SevenZip => {
            create_7z_archive(&prepared_path, &to, options)
        },
    };

    if result.is_ok() {
        finish_packing(from, to)?;
    }

    result
}

/// Creates a 7z archive
///
/// # Arguments
/// * `from` - Source path (file or directory)
/// * `to` - Destination 7z file path
/// * `options` - Archive configuration options
///
/// # Returns
/// * `Ok(())` if archive was created successfully
/// * `Err(std::io::Error)` if archive creation fails
#[allow(dead_code)]
fn create_7z_archive(
    from: impl AsRef<Path>,
    to: impl AsRef<Path>,
    _options: &ArchiveOptions,
) -> Result<(), Error> {
    let prepared_path = prepare_packing(&from, &to)?;

    sevenz_rust::compress_to_path(&prepared_path, to.as_ref())
        .map_err(|e| Error::new(
            ErrorKind::Other,
            format!("Failed to create 7z archive: {}", e)
        ))?;

    finish_packing(from, to)
}

/// Creates a tar archive with the given writer
#[allow(dead_code)]
fn create_tar_archive<W: Write>(src: &Path, writer: W) -> Result<(), Error> {
    let mut builder = tar::Builder::new(writer);
    
    // 添加文件到归档
    builder.append_dir_all("", src).map_err(|e| Error::new(
        ErrorKind::Other,
        format!("Failed to create tar archive: {}", e)
    ))?;
    
    // 完成归档并获取底层写入器
    let mut writer = builder.into_inner().map_err(|e| Error::new(
        ErrorKind::Other,
        format!("Failed to finalize tar archive: {}", e)
    ))?;
    
    // 刷新写入器
    writer.flush().map_err(|e| Error::new(
        ErrorKind::Other,
        format!("Failed to flush archive data: {}", e)
    ))
}

/// Creates an archive in the specified format
///
/// # Arguments
/// * `from` - Source path (file or directory)
/// * `to` - Destination archive path
/// * `options` - Optional archive configuration
///
/// # Returns
/// * `Ok(())` if archive was created successfully
/// * `Err(std::io::Error)` if archive creation fails
///
/// # Examples
/// ```
/// use crate::utils::archive::{create_archive, ArchiveOptions};
///
/// let options = ArchiveOptions::default();
/// let result = create_archive("source/dir", "archive.zip", Some(&options));
/// ```
#[allow(dead_code)]
pub fn create_archive(
    from: impl AsRef<Path>,
    to: impl AsRef<Path>,
    options: Option<&ArchiveOptions>,
) -> Result<(), Error> {
    let options = options.cloned().unwrap_or_default();
    let to_path = to.as_ref();
    
    match to_path.extension().and_then(|ext| ext.to_str()) {
        Some("zip") => create_zip_archive(from, to, zip::CompressionMethod::Deflated, &options),
        Some("tar") => create_compressed_tar(from, to, CompressionFormat::None, &options),
        Some("gz") | Some("tgz") => create_compressed_tar(from, to, CompressionFormat::Gzip, &options),
        Some("bz2") | Some("tbz2") => create_compressed_tar(from, to, CompressionFormat::Bzip2, &options),
        Some("xz") | Some("txz") => create_compressed_tar(from, to, CompressionFormat::Xz, &options),
        Some("7z") => create_7z_archive(from, to, &options),
        _ => Err(Error::new(
            ErrorKind::Unsupported,
            "Unsupported archive format"
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    /// Helper function to create test files and directories
    fn create_test_files(dir: &Path) -> Result<(), Error> {
        let test_files = [
            ("file1.txt", b"Hello, World!" as &[u8]),
            ("file2.txt", b"Another test file"),
            ("nested/file3.txt", b"Nested file content"),
        ];

        for (path, content) in test_files.iter() {
            let file_path = dir.join(path);
            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut file = File::create(file_path)?;
            file.write_all(content)?;
        }
        Ok(())
    }

    #[test]
    fn test_prepare_packing() -> Result<(), Error> {
        let temp_dir = tempdir()?;
        let source = temp_dir.path().join("source.txt");
        let dest = temp_dir.path().join("dest.zip");

        // Test non-existent source
        assert!(prepare_packing(&source, &dest).is_err());

        // Create source file
        File::create(&source)?.write_all(b"test content")?;

        // Test valid paths
        let prepared = prepare_packing(&source, &dest)?;
        assert!(prepared.exists());

        // Test destination exists
        File::create(&dest)?;
        assert!(prepare_packing(&source, &dest).is_err());

        Ok(())
    }

    #[test]
    fn test_zip_archive() -> Result<(), Error> {
        let temp_dir = tempdir()?;
        let source_dir = temp_dir.path().join("source");
        let archive_path = temp_dir.path().join("test.zip");

        fs::create_dir(&source_dir)?;
        create_test_files(&source_dir)?;

        let options = ArchiveOptions::default();
        create_zip_archive(&source_dir, &archive_path, zip::CompressionMethod::Deflated, &options)?;

        assert!(archive_path.exists());
        assert!(fs::metadata(&archive_path)?.len() > 0);

        Ok(())
    }

    #[test_log::test]
    fn test_compressed_tar_formats() -> Result<(), Error> {
        let temp_dir = tempdir()?;
        let source_dir = temp_dir.path().join("source");
        fs::create_dir(&source_dir)?;
        create_test_files(&source_dir)?;

        let options = ArchiveOptions::default();
        let formats = [
            (CompressionFormat::None, "test_ctf.tar"),
            (CompressionFormat::Gzip, "test_ctf.tar.gz"),
            (CompressionFormat::Bzip2, "test_ctf.tar.bz2"),
            (CompressionFormat::Xz, "test_ctf.tar.xz"),
        ];

        for (format, filename) in formats.iter() {
            let archive_path = temp_dir.path().join(filename);
            create_compressed_tar(&source_dir, &archive_path, *format, &options)?;
            assert!(archive_path.exists());

            let archive_size = crate::utils::explorer::get_file_size(&archive_path)?;
            println!("Archive path: {}, size: {}", archive_path.display(), archive_size);
            assert!(fs::metadata(&archive_path)?.len() > 0);
        }

        Ok(())
    }

    #[test]
    fn test_create_archive_auto_format() -> Result<(), Error> {
        let temp_dir = tempdir()?;
        let source_dir = temp_dir.path().join("source");
        fs::create_dir(&source_dir)?;
        create_test_files(&source_dir)?;

        let options = ArchiveOptions::default();
        let extensions = ["zip", "tar", "tar.gz", "tar.bz2", "tar.xz", "7z"];

        for ext in extensions.iter() {
            let archive_path = temp_dir.path().join(format!("test_caaf.{}", ext));
            create_archive(&source_dir, &archive_path, Some(&options))?;
            assert!(archive_path.exists());
            assert!(fs::metadata(&archive_path)?.len() > 0);
        }

        // Test invalid extension
        let invalid_path = temp_dir.path().join("test_caaf.invalid");
        assert!(create_archive(&source_dir, &invalid_path, Some(&options)).is_err());

        Ok(())
    }

    #[test]
    fn test_archive_options() -> Result<(), Error> {
        let temp_dir = tempdir()?;
        let source_dir = temp_dir.path().join("source");
        fs::create_dir(&source_dir)?;
        create_test_files(&source_dir)?;

        // Test different compression levels
        let archive_path = temp_dir.path().join("test_ao.tar.gz");
        let options_min = ArchiveOptions {
            compression_level: 0,
            preserve_permissions: true,
            follow_links: false,
        };
        let options_max = ArchiveOptions {
            compression_level: 9,
            preserve_permissions: true,
            follow_links: false,
        };

        create_compressed_tar(&source_dir, &archive_path, CompressionFormat::Gzip, &options_min)?;
        let size_min = fs::metadata(&archive_path)?.len();

        // Remove the existing file before creating new one
        fs::remove_file(&archive_path)?;

        create_compressed_tar(&source_dir, &archive_path, CompressionFormat::Gzip, &options_max)?;
        let size_max = fs::metadata(&archive_path)?.len();

        // Maximum compression should result in smaller or equal file size
        assert!(size_max <= size_min);

        Ok(())
    }

    #[test]
    fn test_error_cases() -> Result<(), Error> {
        let temp_dir = tempdir()?;
        let source_dir = temp_dir.path().join("source");
        let archive_path = temp_dir.path().join("test.zip");

        // Test non-existent source
        let options = ArchiveOptions::default();
        assert!(create_archive(&source_dir, &archive_path, Some(&options)).is_err());

        // Create a directory with restricted permissions
        fs::create_dir(&source_dir)?;
        create_test_files(&source_dir)?;
        
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let restricted_dir = temp_dir.path().join("restricted");
            fs::create_dir(&restricted_dir)?;
            fs::set_permissions(&restricted_dir, fs::Permissions::from_mode(0o000))?;
            
            let invalid_dest = restricted_dir.join("test.zip");
            assert!(create_archive(&source_dir, &invalid_dest, Some(&options)).is_err());
            
            // Restore permissions so the directory can be cleaned up
            fs::set_permissions(&restricted_dir, fs::Permissions::from_mode(0o755))?;
        }

        #[cfg(windows)]
        {
            // On Windows, test with invalid characters in path
            let invalid_dest = temp_dir.path().join("test\0invalid.zip");
            assert!(create_archive(&source_dir, &invalid_dest, Some(&options)).is_err());
        }

        Ok(())
    }
}