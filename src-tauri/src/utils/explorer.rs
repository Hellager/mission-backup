//! The `explorer` module contains functions about system explorer.
use std::fs::{self, remove_file, remove_dir_all, create_dir_all, read_dir, copy, metadata, OpenOptions};
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use ignore::Walk;
use walkdir::WalkDir;
use fs_extra::dir::get_size;
use std::time::SystemTime;

/******************************************* Basic Action Functions ********************************************/

/// Represents the type of filesystem entry to create
#[derive(Debug, PartialEq)]
pub enum EntryType {
    File,
    Directory,
}

impl std::str::FromStr for EntryType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "file" => Ok(EntryType::File),
            "dir" | "directory" => Ok(EntryType::Directory),
            _ => Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Unsupported entry type: {}", s)
            )),
        }
    }
}

#[allow(dead_code)]
/// Removes a file or directory at the specified path
///
/// # Arguments
/// * `path` - The path to the file or directory to remove
///
/// # Returns
/// * `Ok(())` if removal was successful
/// * `Err(std::io::Error)` if path doesn't exist or removal failed
///
/// # Examples
/// ```
/// use crate::utils::explorer::remove_all;
/// 
/// let result = remove_all("path/to/remove");
/// match result {
///     Ok(_) => println!("Successfully removed"),
///     Err(e) => println!("Failed to remove: {}", e),
/// }
/// ```
pub fn remove_all(path: impl AsRef<Path>) -> Result<(), Error> {
    let target = path.as_ref();

    if !target.exists() {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!("Path does not exist: {}", target.display())
        ));
    }

    match (target.is_file(), target.is_dir()) {
        (true, _) => remove_file(target),
        (_, true) => remove_dir_all(target),
        _ => Err(Error::new(
            ErrorKind::Other,
            format!("Path is neither a file nor a directory: {}", target.display())
        ))
    }
}

#[allow(dead_code)]
/// Creates a file or directory at the specified path, creating parent directories if needed
///
/// # Arguments
/// * `path` - The path where to create the file or directory
/// * `entry_type` - The type of entry to create (File or Directory)
///
/// # Returns
/// * `Ok(())` if creation was successful
/// * `Err(std::io::Error)` if path already exists or creation failed
///
/// # Examples
/// ```
/// use crate::utils::explorer::{create_all, EntryType};
/// 
/// let result = create_all("path/to/new/file.txt", EntryType::File);
/// match result {
///     Ok(_) => println!("Successfully created"),
///     Err(e) => println!("Failed to create: {}", e),
/// }
/// ```
pub fn create_all(path: impl AsRef<Path>, entry_type: EntryType) -> Result<(), Error> {
    let path = path.as_ref();

    if path.exists() {
        return Err(Error::new(
            ErrorKind::AlreadyExists,
            format!("Path already exists: {}", path.display())
        ));
    }

    match entry_type {
        EntryType::File => {
            let parent = path.parent().ok_or_else(|| {
                Error::new(ErrorKind::InvalidInput, "Invalid file path: no parent directory")
            })?;

            if !parent.exists() {
                create_dir_all(parent)?;
            }

            OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(path)?;
        }
        EntryType::Directory => {
            create_dir_all(path)?;
        }
    }

    Ok(())
}

/******************************************* Copy Functions ********************************************/

/// Configuration for directory copying
#[derive(Debug, Clone)]
pub struct CopyOptions {
    /// Patterns to ignore during copying
    pub ignore_patterns: Vec<String>,
    /// Whether to follow symbolic links
    pub follow_links: bool,
    /// Maximum depth to traverse
    pub max_depth: Option<usize>,
}

impl Default for CopyOptions {
    fn default() -> Self {
        Self {
            ignore_patterns: Vec::new(),
            follow_links: false,
            max_depth: None,
        }
    }
}

#[allow(dead_code)]
/// Recursively copies a directory and all its contents to a new location
///
/// # Arguments
/// * `from` - Source directory path
/// * `to` - Destination directory path
///
/// # Returns
/// * `Ok(())` if copy was successful
/// * `Err(std::io::Error)` if copy failed
///
/// # Examples
/// ```
/// use crate::utils::explorer::copy_dir_all;
///
/// let result = copy_dir_all("source/dir", "dest/dir");
/// match result {
///     Ok(_) => println!("Directory copied successfully"),
///     Err(e) => println!("Failed to copy directory: {}", e),
/// }
/// ```
pub fn copy_dir_all(
    from: impl AsRef<Path>,
    to: impl AsRef<Path>,
) -> Result<(), Error> {
    let from_path = from.as_ref();
    let to_path = to.as_ref();

    // Validate source exists and is a directory
    if !from_path.exists() {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!("Source directory does not exist: {}", from_path.display())
        ));
    }
    if !from_path.is_dir() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!("Source path is not a directory: {}", from_path.display())
        ));
    }

    // Create destination directory if it doesn't exist
    create_dir_all(to_path)?;

    for entry in read_dir(from_path)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let source = entry.path();
        let destination = to_path.join(entry.file_name());

        if file_type.is_dir() {
            copy_dir_all(&source, &destination)?;
        } else if file_type.is_file() {
            copy(&source, &destination).map_err(|e| {
                Error::new(
                    e.kind(),
                    format!("Failed to copy '{}' to '{}': {}", 
                        source.display(), 
                        destination.display(), 
                        e
                    )
                )
            })?;
        } else {
            // Handle special files (symlinks, etc.)
            println!("Skipping special file: {}", source.display());
        }
    }
    Ok(())
}

/// Copies a single file, creating parent directories if needed
///
/// # Arguments
/// * `src` - Source file path
/// * `dest` - Destination file path
///
/// # Returns
/// * `Ok(())` if the file was copied successfully
/// * `Err(std::io::Error)` if the copy operation fails
///
/// # Examples
/// ```
/// use crate::utils::explorer::copy_file;
/// use std::path::Path;
/// 
/// let result = copy_file(
///     Path::new("source/file.txt"),
///     Path::new("dest/nested/file.txt")
/// );
/// match result {
///     Ok(_) => println!("File copied successfully"),
///     Err(e) => println!("Failed to copy file: {}", e),
/// }
/// ```
///
/// # Notes
/// - Creates parent directories of the destination path if they don't exist
/// - Preserves file permissions and timestamps
/// - Returns detailed error messages including source and destination paths
fn copy_file(src: &Path, dest: &Path) -> Result<(), Error> {
    if let Some(parent) = dest.parent() {
        if !parent.exists() {
            create_dir_all(parent)?;
        }
    }

    copy(src, dest).map(|_| ()).map_err(|e| {
        Error::new(
            e.kind(),
            format!("Failed to copy '{}' to '{}': {}", 
                src.display(), 
                dest.display(), 
                e
            )
        )
    })
}

#[allow(dead_code)]
/// Copies a file or directory to a new location
///
/// # Arguments
/// * `from` - Source path (file or directory)
/// * `to` - Destination path
///
/// # Returns
/// * `Ok(())` if copy was successful
/// * `Err(std::io::Error)` if copy failed
///
/// # Examples
/// ```
/// use crate::utils::explorer::copy_all;
///
/// let result = copy_all("source/path", "dest/path");
/// match result {
///     Ok(_) => println!("Successfully copied"),
///     Err(e) => println!("Failed to copy: {}", e),
/// }
/// ```
pub fn copy_all(
    from: impl AsRef<Path>,
    to: impl AsRef<Path>
) -> Result<(), Error> {
    let src = from.as_ref();
    let dest = to.as_ref();

    // Validate source exists
    if !src.exists() {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!("Source path does not exist: {}", src.display())
        ));
    }

    // Check if destination already exists
    if dest.exists() {
        return Err(Error::new(
            ErrorKind::AlreadyExists,
            format!("Destination path already exists: {}", dest.display())
        ));
    }

    match (src.is_file(), src.is_dir()) {
        (true, _) => copy_file(src, dest),
        (_, true) => copy_dir_all(src, dest),
        _ => Err(Error::new(
            ErrorKind::Other,
            format!("Source is neither a file nor a directory: {}", src.display())
        ))
    }
}

/// Validates source and destination paths
///
/// # Arguments
/// * `src` - Source path to validate
/// * `dest` - Destination path to validate
///
/// # Returns
/// * `Ok(())` if validation passes
/// * `Err(std::io::Error)` if validation fails
///
/// # Examples
/// ```
/// use crate::utils::explorer::validate_paths;
/// 
/// let result = validate_paths(Path::new("src"), Path::new("dest"));
/// match result {
///     Ok(_) => println!("Paths are valid"),
///     Err(e) => println!("Invalid paths: {}", e),
/// }
/// ```
fn validate_paths(src: &Path, dest: &Path) -> Result<(), Error> {
    if !src.exists() {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!("Source directory does not exist: {}", src.display())
        ));
    }

    if !src.is_dir() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!("Source path is not a directory: {}", src.display())
        ));
    }

    if dest.exists() {
        return Err(Error::new(
            ErrorKind::AlreadyExists,
            format!("Destination path already exists: {}", dest.display())
        ));
    }

    Ok(())
}

/// Copies a single file system entry (file or directory)
///
/// # Arguments
/// * `src` - Source path to copy from
/// * `dest` - Destination path to copy to
///
/// # Returns
/// * `Ok(())` if copy was successful
/// * `Err(std::io::Error)` if copy failed
///
/// # Examples
/// ```
/// use crate::utils::explorer::copy_entry;
/// 
/// let result = copy_entry(Path::new("src/file.txt"), Path::new("dest/file.txt"));
/// match result {
///     Ok(_) => println!("Entry copied successfully"),
///     Err(e) => println!("Failed to copy entry: {}", e),
/// }
/// ```
fn copy_entry(src: &Path, dest: &Path) -> Result<(), Error> {
    if src.is_file() {
        // Ensure parent directory exists
        if let Some(parent) = dest.parent() {
            create_dir_all(parent)?;
        }

        copy(src, dest).map(|_| ()).map_err(|e| Error::new(
            e.kind(),
            format!("Failed to copy file '{}' to '{}': {}", 
                src.display(), 
                dest.display(), 
                e
            )
        ))?;
    } else if src.is_dir() {
        create_dir_all(dest)?;

        // Copy .gitignore file if it exists
        let ignore_file = src.join(".gitignore");
        if ignore_file.exists() {
            copy(&ignore_file, &dest.join(".gitignore")).map_err(|e| Error::new(
                e.kind(),
                format!("Failed to copy .gitignore file: {}", e)
            ))?;
        }
    }

    Ok(())
}


#[allow(dead_code)]
/// Copies a directory while respecting .gitignore rules
///
/// # Arguments
/// * `from` - Source directory path
/// * `to` - Destination directory path
///
/// # Returns
/// * `Ok(())` if copy was successful
/// * `Err(std::io::Error)` if copy failed
///
/// # Examples
/// ```
/// use crate::utils::explorer::copy_dir_with_ignore;
///
/// let result = copy_dir_with_ignore("source/dir", "dest/dir");
/// match result {
///     Ok(_) => println!("Directory copied successfully"),
///     Err(e) => println!("Failed to copy directory: {}", e),
/// }
/// ```
pub fn copy_dir_with_ignore(
    from: impl AsRef<Path>,
    to: impl AsRef<Path>
) -> Result<(), Error> {
    let src = from.as_ref();
    let dest = to.as_ref();

    // Validate source and destination
    validate_paths(src, dest)?;

    // Create the base destination directory
    create_dir_all(dest)?;

    // Walk through the source directory using ignore rules
    for entry_result in Walk::new(src) {
        let entry = entry_result.map_err(|e| Error::new(
            ErrorKind::InvalidData,
            format!("Failed to process entry: {}", e)
        ))?;

        let src_path = entry.path();
        let relative_path = src_path.strip_prefix(src).map_err(|e| Error::new(
            ErrorKind::InvalidInput,
            format!("Failed to compute relative path: {}", e)
        ))?;
        let dest_path = dest.join(relative_path);

        copy_entry(src_path, &dest_path)?;
    }

    Ok(())
}

/// Checks if a path should be ignored based on patterns
///
/// # Arguments
/// * `path` - Path to check
/// * `patterns` - List of patterns to match against
///
/// # Returns
/// * `Ok(bool)` indicating if path should be ignored
/// * `Err(std::io::Error)` if pattern matching fails
///
/// # Examples
/// ```
/// use crate::utils::explorer::should_ignore;
/// 
/// let patterns = vec!["node_modules".to_string(), ".git".to_string()];
/// match should_ignore(Path::new("path/node_modules/file.txt"), &patterns) {
///     Ok(true) => println!("Path should be ignored"),
///     Ok(false) => println!("Path should not be ignored"),
///     Err(e) => println!("Error checking path: {}", e),
/// }
/// ```
fn should_ignore(path: &Path, patterns: &[String]) -> Result<bool, Error> {
    let path_str = path.to_str().ok_or_else(|| Error::new(
        ErrorKind::InvalidData,
        "Path contains invalid Unicode"
    ))?;

    Ok(patterns.iter().any(|pattern| path_str.contains(pattern)))
}

#[allow(dead_code)]
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
pub fn copy_dir_with_custom_ignores(
    from: impl AsRef<Path>,
    to: impl AsRef<Path>,
    options: &CopyOptions,
) -> Result<(), Error> {
    let src = from.as_ref();
    let dest = to.as_ref();

    // Validate paths
    validate_paths(src, dest)?;

    // Configure and create WalkDir iterator
    let mut walker = WalkDir::new(src);
    
    if let Some(depth) = options.max_depth {
        walker = walker.max_depth(depth);
    }
    walker = walker.follow_links(options.follow_links);

    // Create base destination directory
    create_dir_all(dest)?;

    // Process each entry
    for entry_result in walker {
        let entry = entry_result.map_err(|e| Error::new(
            ErrorKind::InvalidData,
            format!("Failed to access entry: {}", e)
        ))?;

        let src_path = entry.path();
        
        // Check if path should be ignored
        if should_ignore(src_path, &options.ignore_patterns)? {
            continue;
        }

        let relative_path = src_path.strip_prefix(src).map_err(|e| Error::new(
            ErrorKind::InvalidInput,
            format!("Failed to compute relative path: {}", e)
        ))?;
        let dest_path = dest.join(relative_path);

        copy_entry(src_path, &dest_path)?;
    }

    Ok(())
}

/******************************************* Restrict Functions ********************************************/

/// Represents a directory entry with its creation time
#[derive(Clone)]
struct DirEntry {
    path: PathBuf,
    size: u64,
    created_time: SystemTime,
}

/// Gets the size of a file
///
/// # Arguments
/// * `path` - Path to the file
///
/// # Returns
/// * `Ok(u64)` containing the file size in bytes
/// * `Err(std::io::Error)` if size calculation fails
///
/// # Examples
/// ```
/// use crate::utils::explorer::get_file_size;
/// 
/// match get_file_size(Path::new("path/to/file.txt")) {
///     Ok(size) => println!("File size: {} bytes", size),
///     Err(e) => println!("Failed to get file size: {}", e),
/// }
/// ```
pub fn get_file_size(path: &Path) -> Result<u64, Error> {
    fs::metadata(path)
        .map(|metadata| metadata.len())
        .map_err(|e| Error::new(
            e.kind(),
            format!("Failed to get file size for '{}': {}", path.display(), e)
        ))
}

/// Gets the size of a directory
///
/// # Arguments
/// * `path` - Path to the directory
///
/// # Returns
/// * `Ok(u64)` containing the directory size in bytes
/// * `Err(std::io::Error)` if size calculation fails
///
/// # Examples
/// ```
/// use crate::utils::explorer::get_directory_size;
/// 
/// match get_directory_size(Path::new("path/to/dir")) {
///     Ok(size) => println!("Directory size: {} bytes", size),
///     Err(e) => println!("Failed to get directory size: {}", e),
/// }
/// ```
fn get_directory_size(path: &Path) -> Result<u64, Error> {
    get_size(path).map_err(|e| Error::new(
        ErrorKind::Other,
        format!("Failed to get directory size for '{}': {}", path.display(), e)
    ))
}

#[allow(dead_code)]
/// Gets the size of a file or directory in bytes
///
/// # Arguments
/// * `path` - Path to the file or directory
///
/// # Returns
/// * `Ok(u64)` - Size in bytes
/// * `Err(std::io::Error)` - If the operation fails
///
/// # Examples
/// ```
/// use crate::utils::explorer::get_path_size;
///
/// match get_path_size("path/to/file_or_dir") {
///     Ok(size) => println!("Size: {} bytes", size),
///     Err(e) => println!("Error getting size: {}", e),
/// }
/// ```
pub fn get_path_size(path: impl AsRef<Path>) -> Result<u64, Error> {
    let path = path.as_ref();

    // Validate path exists
    if !path.exists() {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!("Path does not exist: {}", path.display())
        ));
    }

    match path.is_file() {
        true => get_file_size(path),
        false => get_directory_size(path),
    }
}

/// Collects directory entries with their metadata
///
/// # Arguments
/// * `path` - Path to the directory to collect entries from
///
/// # Returns
/// * `Ok(Vec<DirEntry>)` containing collected entries with metadata
/// * `Err(std::io::Error)` if collection fails
///
/// # Examples
/// ```
/// use crate::utils::explorer::collect_dir_entries;
/// 
/// match collect_dir_entries(Path::new("path/to/dir")) {
///     Ok(entries) => println!("Collected {} entries", entries.len()),
///     Err(e) => println!("Failed to collect entries: {}", e),
/// }
/// ```
fn collect_dir_entries(path: &Path) -> Result<Vec<DirEntry>, Error> {
    let mut entries = Vec::new();

    for entry_result in read_dir(path)? {
        let entry = entry_result.map_err(|e| Error::new(
            e.kind(),
            format!("Failed to read directory entry: {}", e)
        ))?;

        let path = entry.path();
        let metadata = metadata(&path).map_err(|e| Error::new(
            e.kind(),
            format!("Failed to get metadata for '{}': {}", path.display(), e)
        ))?;

        let size = if path.is_file() {
            metadata.len()
        } else {
            get_size(&path).unwrap_or(0)
        };

        let created_time = metadata.created().map_err(|e| Error::new(
            e.kind(),
            format!("Failed to get creation time for '{}': {}", path.display(), e)
        ))?;

        entries.push(DirEntry {
            path,
            size,
            created_time,
        });
    }

    Ok(entries)
}

/// Removes the specified entries
///
/// # Arguments
/// * `entries` - List of entries to remove
///
/// # Returns
/// * `Ok(())` if removal was successful
/// * `Err(std::io::Error)` if removal fails
///
/// # Examples
/// ```
/// use crate::utils::explorer::remove_entries;
/// 
/// let entries = collect_dir_entries(Path::new("path/to/dir"))?;
/// match remove_entries(&entries[..]) {
///     Ok(_) => println!("Entries removed successfully"),
///     Err(e) => println!("Failed to remove entries: {}", e),
/// }
/// ```
fn remove_entries(entries: &[DirEntry]) -> Result<(), Error> {
    for entry in entries {
        remove_all(&entry.path).map_err(|e| Error::new(
            e.kind(),
            format!("Failed to remove '{}': {}", entry.path.display(), e)
        ))?;
    }
    Ok(())
}

#[allow(dead_code)]
/// Restricts the number of items in a directory by removing older items
///
/// # Arguments
/// * `path` - Path to the directory
/// * `max_count` - Maximum number of items to keep (keeps the newest ones)
///
/// # Returns
/// * `Ok(())` if the operation was successful
/// * `Err(std::io::Error)` if any operation fails
///
/// # Examples
/// ```
/// use crate::utils::explorer::restrict_dir_items_count;
///
/// match restrict_dir_items_count("path/to/dir", 10) {
///     Ok(_) => println!("Successfully restricted directory items"),
///     Err(e) => println!("Error restricting directory items: {}", e),
/// }
/// ```
pub fn restrict_dir_items_count(
    path: impl AsRef<Path>,
    max_count: usize,
) -> Result<(), Error> {
    let path = path.as_ref();

    // Validate directory
    if !path.exists() {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!("Directory does not exist: {}", path.display())
        ));
    }

    if !path.is_dir() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!("Path is not a directory: {}", path.display())
        ));
    }

    // Collect directory entries with their creation times
    let mut entries = collect_dir_entries(path)?;

    // If we don't need to remove anything, return early
    if entries.len() <= max_count {
        return Ok(());
    }

    // Sort by creation time (newest first)
    entries.sort_by(|a, b| b.created_time.cmp(&a.created_time));

    // Remove older entries
    remove_entries(&entries[max_count..])
}

#[allow(dead_code)]
/// Restricts the total size of items in a directory by removing older items
///
/// # Arguments
/// * `path` - Path to the directory
/// * `max_size` - Maximum total size in bytes
///
/// # Returns
/// * `Ok(())` if the operation was successful
/// * `Err(std::io::Error)` if any operation fails
///
/// # Examples
/// ```
/// use crate::utils::explorer::restrict_dir_items_size;
///
/// // Restrict directory to 1GB
/// match restrict_dir_items_size("path/to/dir", 1024 * 1024 * 1024) {
///     Ok(_) => println!("Successfully restricted directory size"),
///     Err(e) => println!("Error restricting directory size: {}", e),
/// }
/// ```
pub fn restrict_dir_items_size(
    path: impl AsRef<Path>,
    max_size: u64,
) -> Result<(), Error> {
    let path = path.as_ref();

    // Validate directory
    if !path.exists() {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!("Directory does not exist: {}", path.display())
        ));
    }

    if !path.is_dir() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!("Path is not a directory: {}", path.display())
        ));
    }

    // Get current directory size
    let current_size = get_size(path).map_err(|e| Error::new(
        ErrorKind::Other,
        format!("Failed to get directory size: {}", e)
    ))?;

    // If we're already under the limit, return early
    if current_size <= max_size {
        return Ok(());
    }

    // Collect and sort entries by creation time
    let mut entries = collect_dir_entries(path)?;
    entries.sort_by(|a, b| b.created_time.cmp(&a.created_time));

    // Remove oldest entries until we're under the size limit
    let mut current_total = current_size;
    let mut entries_to_remove = Vec::new();

    for entry in entries.iter().rev() {
        if current_total <= max_size {
            break;
        }
        current_total = current_total.saturating_sub(entry.size);
        entries_to_remove.push(entry.clone());
    }

    // Remove the selected entries
    remove_entries(&entries_to_remove)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use std::thread::sleep;
    use std::time::Duration;
    use tempfile::tempdir;

    // Helper function to create test files
    fn create_test_file(path: &Path, content: &[u8]) -> Result<(), Error> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = File::create(path)?;
        file.write_all(content)?;
        Ok(())
    }

    #[test]
    fn test_remove_all() -> Result<(), Error> {
        let temp_dir = tempdir()?;
        
        // Test file removal
        let file_path = temp_dir.path().join("test.txt");
        create_test_file(&file_path, b"test content")?;
        assert!(remove_all(&file_path).is_ok());
        assert!(!file_path.exists());

        // Test directory removal
        let dir_path = temp_dir.path().join("test_dir");
        fs::create_dir(&dir_path)?;
        create_test_file(&dir_path.join("file.txt"), b"test content")?;
        assert!(remove_all(&dir_path).is_ok());
        assert!(!dir_path.exists());

        Ok(())
    }

    #[test]
    fn test_create_all() -> Result<(), Error> {
        let temp_dir = tempdir()?;

        // Test file creation
        let file_path = temp_dir.path().join("nested/test.txt");
        assert!(create_all(&file_path, EntryType::File).is_ok());
        assert!(file_path.exists());
        assert!(file_path.is_file());

        // Test directory creation
        let dir_path = temp_dir.path().join("nested/test_dir");
        assert!(create_all(&dir_path, EntryType::Directory).is_ok());
        assert!(dir_path.exists());
        assert!(dir_path.is_dir());

        Ok(())
    }

    #[test]
    fn test_get_path_size() -> Result<(), Error> {
        let temp_dir = tempdir()?;

        // Test file size
        let file_path = temp_dir.path().join("test.txt");
        let content = b"Hello, World!";
        create_test_file(&file_path, content)?;
        assert_eq!(get_path_size(&file_path)?, content.len() as u64);

        // Test directory size
        let dir_path = temp_dir.path().join("test_dir");
        fs::create_dir(&dir_path)?;
        create_test_file(&dir_path.join("file1.txt"), b"File 1")?;
        create_test_file(&dir_path.join("file2.txt"), b"File 2")?;
        assert!(get_path_size(&dir_path)? > 0);

        Ok(())
    }

    #[test]
    fn test_copy_operations() -> Result<(), Error> {
        let temp_dir = tempdir()?;
        let source = temp_dir.path().join("source");
        let dest = temp_dir.path().join("dest");

        // Create source structure
        fs::create_dir(&source)?;
        create_test_file(&source.join("file1.txt"), b"File 1")?;
        create_test_file(&source.join("file2.txt"), b"File 2")?;

        // Test basic copy
        assert!(copy_all(&source, &dest).is_ok());
        assert!(dest.exists());
        assert!(dest.join("file1.txt").exists());

        // Test copy with ignore patterns
        let dest_ignore = temp_dir.path().join("dest_ignore");
        let options = CopyOptions {
            ignore_patterns: vec!["file2.txt".to_string()],
            ..Default::default()
        };
        assert!(copy_dir_with_custom_ignores(&source, &dest_ignore, &options).is_ok());
        assert!(dest_ignore.join("file1.txt").exists());
        assert!(!dest_ignore.join("file2.txt").exists());

        Ok(())
    }

    #[test]
    fn test_restrict_operations() -> Result<(), Error> {
        let temp_dir = tempdir()?;
        let test_dir = temp_dir.path().join("test_dir");
        fs::create_dir(&test_dir)?;

        // Create test files with different timestamps
        for i in 0..5 {
            let file_path = test_dir.join(format!("file{}.txt", i));
            create_test_file(&file_path, &vec![b'a'; 1024 * (i + 1)])?;
            sleep(Duration::from_millis(100));
        }

        // Test count restriction
        assert!(restrict_dir_items_count(&test_dir, 3).is_ok());
        let entries: Vec<_> = fs::read_dir(&test_dir)?.collect();
        assert_eq!(entries.len(), 3);

        // Test size restriction
        let size_dir = temp_dir.path().join("size_dir");
        fs::create_dir(&size_dir)?;
        for i in 0..3 {
            let file_path = size_dir.join(format!("file{}.txt", i));
            create_test_file(&file_path, &vec![b'a'; 1024 * (i + 1)])?;
        }
        assert!(restrict_dir_items_size(&size_dir, 2048).is_ok()); // 2KB limit
        assert!(get_path_size(&size_dir)? <= 2048);

        Ok(())
    }

    #[test]
    fn test_error_cases() {
        // Test nonexistent path
        assert!(matches!(
            remove_all("nonexistent/path").unwrap_err().kind(),
            ErrorKind::NotFound
        ));

        // // Test invalid entry type
        // assert!(matches!(
        //     create_all("test.txt", "invalid_type").unwrap_err().kind(),
        //     ErrorKind::InvalidInput
        // ));

        // Test copy to existing destination
        let temp_dir = tempdir().unwrap();
        let dest = temp_dir.path().join("existing");
        fs::create_dir(&dest).unwrap();
        assert!(matches!(
            copy_all(temp_dir.path(), &dest).unwrap_err().kind(),
            ErrorKind::AlreadyExists
        ));
    }
}
