use crate::emu::object_handle::windows_path::WindowsPath;
use ahash::{AHashMap, HashMap, HashSet, HashSetExt};
use dunce::canonicalize;
use soft_canonicalize::soft_canonicalize;
use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::{env, fs};

#[cfg(target_os = "windows")]
use windows_sys::Win32::Storage::FileSystem::GetLogicalDrives;

/*
* Example of used:
fn main() -> std::io::Result<()> {
    // Initialize the filesystem with a root
    init_file_system_root("C:\\")?;

    // Use the filesystem
    let drives = list_available_drives()?;
    println!("Available drives: {:?}", drives);

    // Translate a Windows path
    let win_path = WindowsPath::from_string("C:\\Windows\\System32");
    let local_path = translate_path(&win_path)?;
    println!("Local path: {}", local_path.display());

    // Add a mapping
    map_path(
        WindowsPath::from_string("D:\\Games"),
        PathBuf::from("/mnt/d/Games")
    );

    Ok(())
}

// In a different module
pub mod utils {
    use super::*;

    pub fn process_path(path: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let win_path = WindowsPath::from_string(path);
        with_fs!(|fs| fs.translate(&win_path))
    }
}
Here we initialize and then set the FILE_SYSTEM static structure in the main when emulate.
 */

// --- WinAPI Constants (Commonly Used) ---
pub const INVALID_HANDLE_VALUE: usize = !0; // -1 as usize
pub const GENERIC_READ: u32 = 0x80000000;
pub const GENERIC_WRITE: u32 = 0x40000000;
pub const GENERIC_EXECUTE: u32 = 0x20000000;
pub const GENERIC_ALL: u32 = 0x10000000;

pub const CREATE_NEW: usize = 1;
pub const CREATE_ALWAYS: usize = 2;
pub const OPEN_EXISTING: usize = 3;
pub const OPEN_ALWAYS: usize = 4;
pub const TRUNCATE_EXISTING: usize = 5;

pub const FILE_ATTRIBUTE_READONLY: usize = 0x1;
pub const FILE_ATTRIBUTE_HIDDEN: usize = 0x2;
pub const FILE_ATTRIBUTE_SYSTEM: usize = 0x4;
pub const FILE_ATTRIBUTE_DIRECTORY: usize = 0x10;
pub const FILE_ATTRIBUTE_ARCHIVE: usize = 0x20;
pub const FILE_ATTRIBUTE_NORMAL: usize = 0x80;
pub const FILE_ATTRIBUTE_TEMPORARY: usize = 0x100;
pub const FILE_ATTRIBUTE_COMPRESSED: usize = 0x800;
pub const FILE_ATTRIBUTE_OFFLINE: usize = 0x1000;
pub const FILE_ATTRIBUTE_NOT_CONTENT_INDEXED: usize = 0x2000;
pub const FILE_ATTRIBUTE_ENCRYPTED: usize = 0x4000;

pub const FILE_FLAG_WRITE_THROUGH: usize = 0x80000000;
pub const FILE_FLAG_OVERLAPPED: usize = 0x40000000;
pub const FILE_FLAG_NO_BUFFERING: usize = 0x20000000;
pub const FILE_FLAG_RANDOM_ACCESS: usize = 0x10000000;
pub const FILE_FLAG_SEQUENTIAL_SCAN: usize = 0x8000000;
pub const FILE_FLAG_DELETE_ON_CLOSE: usize = 0x4000000;
pub const FILE_FLAG_BACKUP_SEMANTICS: usize = 0x2000000;
pub const FILE_FLAG_POSIX_SEMANTICS: usize = 0x1000000;
pub const FILE_FLAG_OPEN_REPARSE_POINT: usize = 0x200000;
pub const FILE_FLAG_OPEN_NO_RECALL: usize = 0x100000;
pub const FILE_FLAG_FIRST_PIPE_INSTANCE: usize = 0x80000;
struct FileHandleManagement {
    handle_management: HashMap<u32, FileHandle>, // Fixed typo: handleMagemement -> handle_management
}

pub static FILE_SYSTEM: OnceLock<FileSystem> = OnceLock::new();

pub fn init_file_system<P: AsRef<Path>>(file_root: Option<P>) -> io::Result<()> {
    // Resolve the root path
    let root = if let Some(path) = file_root {
        let path = path.as_ref();
        if path.is_absolute() {
            path.to_path_buf()
        } else {
            // Relative to executable directory
            let exe_dir = env::current_exe()?
                .parent()
                .ok_or_else(|| io::Error::other("Failed to get exe directory"))?
                .to_path_buf();
            exe_dir.join(path)
        }
    } else {
        // Default: "file_root" folder next to executable
        let exe_dir = env::current_exe()?
            .parent()
            .ok_or_else(|| io::Error::other("Failed to get exe directory"))?
            .to_path_buf();
        exe_dir.join("file_root")
    };

    if !root.as_os_str().is_empty() {
        fs::create_dir_all(&root)?;
    }

    // Build filesystem without hardcoded mappings (add them dynamically if needed)
    let builder = FileSystemBuilder::new().with_root(&root);
    // If you need the System32 mapping, make it relative to root:
    // .with_mapping(
    //     WindowsPath::from_string("C:\\Windows\\System32"),
    //     root.join("c/Windows/System32")
    // );

    let fs = builder.build()?;

    FILE_SYSTEM
        .set(fs)
        .map_err(|_| io::Error::other("FileSystem already initialized"))
}

// Represents the state and metadata associated with a Windows file handle
pub struct FileHandle {
    // --- Core File/Directory Info ---
    name: String,  // Original name used to open the handle
    path: PathBuf, // Resolved path
    is_dir: bool,  // Whether this handle represents a directory
    // --- Rust File/ReadDir Objects ---
    file: Option<File>, // Actual Rust file handle (for files)
    // --- WinAPI Handle State ---
    access_mode: u32,          // Access flags (e.g., GENERIC_READ, GENERIC_WRITE)
    creation_disposition: u32, // How the file was opened (CREATE_ALWAYS, OPEN_EXISTING, etc.)
    flags_and_attributes: u32, // File attributes and flags (FILE_ATTRIBUTE_HIDDEN, FILE_FLAG_SEQUENTIAL_SCAN, etc.)
    // --- File Position and State ---
    file_position: u64, // Current position in the file (for seeking/reading/writing)
    // --- Sharing and Security (Simplified for Emulation) ---
    sharing_mode: u32, // Sharing flags (FILE_SHARE_READ, FILE_SHARE_WRITE, etc.)
    // --- Additional State ---
    is_valid: bool, // Whether the handle is considered valid (e.g., not closed)
    is_eof: bool,   // End-of-file flag
    // --- Example: Potential for caching or other emulation-specific data ---
    // cache: Option<SomeCacheType>,
    pub file_size: u64,
}

// Corrected function signature and implementation

impl FileHandle {
    // Creates a new FileHandle instance based on a Windows path and parameters
    // This would typically be called from an emulated CreateFile function
    pub fn new(
        name: String,
        access_mode: u32,
        creation_disposition: u32,
        flags_and_attributes: u32,
        sharing_mode: u32,
    ) -> Result<FileHandle, Box<dyn std::error::Error>> {
        // Return Result for better error handling

        let windows_path = WindowsPath::from_path(&name)?;
        let resolved_path = FILE_SYSTEM
            .get()
            .and_then(|file_system| file_system.translate(&windows_path).ok())
            .unwrap();

        let metadata = fs::metadata(&resolved_path)?;

        // Determine if it's a directory *after* potential creation logic (simplified here)
        let is_dir = metadata.is_dir();

        let file = if is_dir {
            // we haven't support opening a directory yet
            return Err(Box::new(io::Error::new(
                ErrorKind::InvalidInput,
                "Calling handle other than File isn't support yet",
            )));
        } else {
            // It's a file, attempt to open/create based on creation_disposition and access_mode
            let file_result = match creation_disposition {
                1 => File::create_new(&resolved_path), // CREATE_NEW
                2 => File::create(&resolved_path),     // CREATE_ALWAYS
                3 => File::open(&resolved_path),       // OPEN_EXISTING
                5 => File::options()
                    .read(true)
                    .write(true)
                    .truncate(true)
                    .open(&resolved_path), // TRUNCATE_EXISTING
                4 => File::options()
                    .read(true)
                    .write(true)
                    .create(true)
                    .open(&resolved_path), // OPEN_ALWAYS
                _ => {
                    return Err(Box::new(io::Error::new(
                        ErrorKind::InvalidInput,
                        "Unknown or unsupported access mode",
                    )));
                }
            };
            Some(file_result?)
        };

        let file_size = metadata.len();
        Ok(FileHandle {
            name,
            path: resolved_path,
            is_dir,
            file,
            access_mode,
            creation_disposition,
            flags_and_attributes,
            file_position: 0,
            sharing_mode,
            is_valid: true, // Assume valid upon creation
            is_eof: false,  // Assume not at EOF initially
            file_size,
        })
    }

    // Example methods that might be called by emulated WinAPI functions
    pub fn read(&mut self, buffer: &mut [u8]) -> Result<usize, io::Error> {
        if !self.is_valid {
            return Err(io::Error::new(
                ErrorKind::InvalidInput,
                "Invalid handle or operation for directory",
            ));
        }
        if let Some(ref mut f) = self.file {
            let bytes_read = f.read(buffer)?;
            self.file_position += bytes_read as u64;
            // Update EOF flag if necessary
            self.is_eof = self.file_position >= self.file_size && bytes_read == 0;
            Ok(bytes_read)
        } else {
            Err(io::Error::new(
                ErrorKind::InvalidInput,
                "No file object associated",
            ))
        }
    }

    pub fn write(&mut self, buffer: &[u8]) -> Result<usize, io::Error> {
        if !(self.is_valid && !self.is_dir) {
            return Err(io::Error::new(
                ErrorKind::InvalidInput,
                "Invalid handle or operation for directory",
            ));
        }
        if let Some(ref mut f) = self.file {
            let bytes_written = f.write(buffer)?;
            self.file_position += bytes_written as u64;
            self.is_eof = false; // Writing typically means not at EOF
            Ok(bytes_written)
        } else {
            Err(io::Error::new(
                ErrorKind::InvalidInput,
                "No file object associated",
            ))
        }
    }

    pub fn seek(&mut self, pos: SeekFrom) -> Result<u64, io::Error> {
        if !self.is_valid || self.is_dir {
            return Err(io::Error::new(
                ErrorKind::InvalidInput,
                "Invalid handle or operation for directory",
            ));
        }
        if let Some(ref mut f) = self.file {
            let new_pos = f.seek(pos)?;
            self.file_position = new_pos;
            self.is_eof = false; // Seeking usually means not at EOF
            Ok(new_pos)
        } else {
            Err(io::Error::new(
                ErrorKind::InvalidInput,
                "No file object associated",
            ))
        }
    }

    pub fn close(&mut self) {
        // Release resources
        self.file.take(); // Drops the File, closing it
        self.is_valid = false;
        // Other cleanup if necessary
    }

    // Getter methods
    pub fn is_valid(&self) -> bool {
        self.is_valid
    }
    pub fn is_dir(&self) -> bool {
        self.is_dir
    }
    pub fn get_path(&self) -> &Path {
        &self.path
    }
    pub fn get_position(&self) -> u64 {
        self.file_position
    }
    pub fn is_eof(&self) -> bool {
        self.is_eof
    }

    // Setter for file_position (e.g., from SetFilePointer)
    pub fn set_position(&mut self, pos: u64) {
        if let Some(ref mut f) = self.file {
            if f.seek(SeekFrom::Start(pos)).is_ok() {
                self.file_position = pos;
                self.is_eof = false; // Seeking usually means not at EOF
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct FileSystem {
    root: PathBuf,
    mappings: AHashMap<WindowsPath, PathBuf>,
}

impl FileSystem {
    /// Creates a new FileSystem with the given root directory
    pub fn new<P: AsRef<Path>>(root: P) -> io::Result<Self> {
        let root = Self::canonical(root.as_ref())?;
        Ok(Self {
            root,
            mappings: AHashMap::with_capacity(10),
        })
    }

    /// Checks if a path is an escaping relative path (starts with ".." or is empty)
    pub fn is_escaping_relative_path<P: AsRef<Path>>(path: P) -> bool {
        let path = path.as_ref();
        path.as_os_str().is_empty()
            || path
                .components()
                .next()
                .map(|c| c == std::path::Component::ParentDir)
                .unwrap_or(false)
    }

    /// Checks if target is a subpath of root
    pub fn is_subpath<P1: AsRef<Path>, P2: AsRef<Path>>(
        normal_root: P1,
        normal_target: P2,
    ) -> bool {
        let normal_root = normal_root.as_ref();
        let normal_target = normal_target.as_ref();

        if let Some(relative_path) = Self::relative(normal_target, normal_root) {
            !Self::is_escaping_relative_path(relative_path)
        } else {
            false
        }
    }

    /// Lists available drives in the filesystem
    pub fn list_drives(&self) -> io::Result<HashSet<char>> {
        let mut drives = HashSet::with_capacity(2);

        // If root is empty and we're on Windows, list logical drives
        #[cfg(target_os = "windows")]
        if self.root.as_os_str().is_empty() {
            // SAFETY: `GetLogicalDrives` is a parameterless Win32 call that just
            // returns a bitmask; there are no pointer or lifetime invariants.
            unsafe {
                let drive_bits = GetLogicalDrives();
                for drive in b'a'..=b'z' {
                    let drive_index = (drive - b'a') as u32;
                    if drive_bits & (1 << drive_index) != 0 {
                        drives.insert(drive as char);
                    }
                }
            }
            return Ok(drives);
        }

        // Otherwise, list directories in root that are single characters
        for entry in fs::read_dir(&self.root)? {
            let entry = entry?;
            let filename = entry.file_name();

            if let Some(filename_str) = filename.to_str() {
                if filename_str.len() == 1 {
                    if let Some(ch) = filename_str.chars().next() {
                        drives.insert(ch.to_ascii_lowercase());
                    }
                }
            }
        }

        Ok(drives)
    }

    /// Translates a WindowsPath to a local filesystem path
    pub fn translate(&self, win_path: &WindowsPath) -> Result<PathBuf, Box<dyn std::error::Error>> {
        if !win_path.is_absolute() {
            return Err(format!("Only absolute paths can be translated: {}", win_path).into());
        }

        // Check if the path is mapped
        if let Some(mapped_path) = self.mappings.get(win_path) {
            return Ok(mapped_path.clone());
        }

        // On Windows, if root is empty, return the path as-is
        #[cfg(target_os = "windows")]
        if self.root.as_os_str().is_empty() {
            return Ok(PathBuf::from(win_path.to_string()));
        }

        // Otherwise, construct path relative to root
        let drive = win_path.get_drive().unwrap_or('c');
        let root_drive = PathBuf::from(format!("{}", drive));
        let root_path = self.root.join(root_drive);

        let mut path = self.root.join(win_path.to_portable_path());
        path = soft_canonicalize(&path)?;

        if Self::is_subpath(&root_path, &path) {
            Ok(path)
        } else {
            Ok(root_path)
        }
    }

    /// Accesses all mapped entries that are children of the given WindowsPath
    pub fn access_mapped_entries<F>(&self, win_path: &WindowsPath, mut accessor: F)
    where
        F: FnMut(&WindowsPath, &PathBuf),
    {
        for (mapped_path, local_path) in &self.mappings {
            if !mapped_path.is_empty() {
                let parent = mapped_path.parent();
                if parent == *win_path {
                    accessor(mapped_path, local_path);
                }
            }
        }
    }

    /// Converts a local path to a WindowsPath relative to the filesystem root
    ///
    /// Given:
    /// - root: `/tmp/vfs`
    /// - local_path: `/tmp/vfs/c/Windows/System32`
    ///
    /// Returns: `c:\Windows\System32`
    pub fn local_to_windows_path<P: AsRef<Path>>(
        &self,
        local_path: P,
    ) -> Result<WindowsPath, Box<dyn std::error::Error>> {
        let local_path = local_path.as_ref();

        // Handle empty path
        if local_path.as_os_str().is_empty() {
            return Err("Path is empty".into());
        }

        // On Windows with empty root, convert directly
        #[cfg(target_os = "windows")]
        if self.root.as_os_str().is_empty() {
            return Ok(WindowsPath::from_path(local_path)?);
        }

        // Get the absolute path (use soft_canonicalize to resolve symlinks consistently)
        // This is important on macOS where /var is a symlink to /private/var
        let abs_local_path = soft_canonicalize(local_path)?;
        let abs_root = soft_canonicalize(&self.root)?;

        // Strip the root prefix to get the relative path
        let relative = abs_local_path
            .strip_prefix(&abs_root)
            .map_err(|_| format!("Path {:?} is not under root {:?}", abs_local_path, abs_root))?;

        // Get the components of the relative path
        let mut components = relative.components();

        // The first component should be the drive letter
        let drive = components
            .next()
            .and_then(|c| c.as_os_str().to_str())
            .and_then(|s| s.chars().next())
            .ok_or("Path does not contain a drive letter component")?;

        // Validate that the drive is a single alphabetic character
        if !drive.is_ascii_alphabetic() {
            return Err(format!("Invalid drive letter: {}", drive).into());
        }

        // Build the Windows path string from remaining components
        let mut path_str = format!("{}:", drive.to_ascii_lowercase());
        for component in components {
            if let Some(s) = component.as_os_str().to_str() {
                path_str.push('\\');
                path_str.push_str(s);
            }
        }

        // If there are no additional components, add the root slash
        if !path_str.contains('\\') {
            path_str.push('\\');
        }

        Ok(WindowsPath::from_string(&path_str))
    }

    /// Maps a WindowsPath to a local filesystem path
    pub fn map(&mut self, src: WindowsPath, dest: PathBuf) {
        self.mappings.insert(src, dest);
    }

    /// Returns the root path of the filesystem
    pub fn root(&self) -> &Path {
        &self.root
    }

    // Helper methods

    fn canonical<P: AsRef<Path>>(path: P) -> io::Result<PathBuf> {
        let path = path.as_ref();
        if path.as_os_str().is_empty() {
            Ok(PathBuf::new())
        } else {
            canonicalize(path)
        }
    }

    fn absolute<P: AsRef<Path>>(path: P) -> io::Result<PathBuf> {
        let path = path.as_ref();
        if path.is_absolute() {
            Ok(path.to_path_buf())
        } else {
            env::current_dir().map(|current| current.join(path))
        }
    }

    fn relative<P1: AsRef<Path>, P2: AsRef<Path>>(path: P1, base: P2) -> Option<PathBuf> {
        let path = path.as_ref();
        let base = base.as_ref();

        // Convert both paths to absolute and canonicalize if possible
        let abs_path = soft_canonicalize(path).unwrap();
        let abs_base = soft_canonicalize(base).unwrap();

        // Strip prefix if path starts with base
        if abs_path.starts_with(&abs_base) {
            let remainder = abs_path
                .strip_prefix(&abs_base)
                .map_err(|_| io::Error::other("Failed to strip prefix"))
                .unwrap();
            Some(PathBuf::from(remainder))
        } else {
            // Return the path as-is if it's not under the base
            None
        }
    }
}

pub struct FileSystemBuilder {
    root: Option<PathBuf>,
    initial_mappings: Vec<(WindowsPath, PathBuf)>,
}

impl Default for FileSystemBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl FileSystemBuilder {
    pub fn new() -> Self {
        Self {
            root: None,
            initial_mappings: Vec::new(),
        }
    }

    pub fn with_root<P: Into<PathBuf>>(mut self, root: P) -> Self {
        self.root = Some(root.into());
        self
    }

    pub fn with_mapping(mut self, src: WindowsPath, dest: PathBuf) -> Self {
        self.initial_mappings.push((src, dest));
        self
    }

    pub fn build(self) -> io::Result<FileSystem> {
        let root = self
            .root
            .ok_or_else(|| io::Error::new(ErrorKind::InvalidInput, "Root path not set"))?;

        let result = FileSystem::new(&root);
        if result.is_err() {
            let err = result.err().unwrap();
            println!("Failed to initialize filesystem with err: {}", err);
            println!(
                "Please check the file location: {}",
                root.to_str().unwrap()
            );
            return Err(err);
        }
        let mut fs = result?;

        for (src, dest) in self.initial_mappings {
            fs.map(src, dest);
        }

        Ok(fs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    // Add to Cargo.toml: serial_test = "3.0"
    use serial_test::serial;

    fn cleanup_dir(path: &Path) {
        if path.exists() {
            fs::remove_dir_all(path).ok();
        }
    }

    #[test]
    fn test_file_system_creation() {
        let temp_dir = tempdir().unwrap();
        let fs = FileSystem::new(temp_dir.path()).unwrap();
        assert!(fs.root().exists());
    }

    #[test]
    fn test_is_escaping_relative_path() {
        assert!(FileSystem::is_escaping_relative_path(""));
        assert!(FileSystem::is_escaping_relative_path(".."));
        assert!(FileSystem::is_escaping_relative_path("../test"));
        assert!(!FileSystem::is_escaping_relative_path("test"));
        assert!(!FileSystem::is_escaping_relative_path("test/path"));
    }

    #[test]
    fn test_is_subpath() {
        let base = Path::new("/home/user");
        let sub = Path::new("/home/user/documents");
        let not_sub = Path::new("/home/other");

        assert!(FileSystem::is_subpath(base, sub));
        assert!(!FileSystem::is_subpath(base, not_sub));
    }

    #[test]
    fn test_mapping() {
        let temp_dir = tempdir().unwrap();
        let mut fs = FileSystem::new(temp_dir.path()).unwrap();

        let win_path = WindowsPath::from_string("C:\\Windows\\System32");
        let local_path = temp_dir.path().join("system32");

        fs.map(win_path.clone(), local_path.clone());

        let translated = fs.translate(&win_path).unwrap();
        assert_eq!(translated, local_path);
    }

    #[test]
    fn test_local_to_windows_path() {
        let temp_dir = tempdir().unwrap();
        let fs = FileSystem::new(temp_dir.path()).unwrap();

        // Create a test directory structure
        let drive_c = temp_dir.path().join("c");
        fs::create_dir_all(&drive_c).unwrap();
        let windows_dir = drive_c.join("Windows");
        fs::create_dir_all(&windows_dir).unwrap();
        assert!(drive_c.exists());
        assert!(windows_dir.exists());

        let windows_path = fs.local_to_windows_path(&windows_dir).unwrap();
        assert_eq!(windows_path.get_drive(), Some('c'));
        assert_eq!(windows_path.leaf(), Some("windows"));
    }

    #[test]
    #[serial] // Prevents parallel execution due to global OnceLock
    fn test_init_with_absolute_path() {
        let temp_dir = tempdir().unwrap();
        let root_path = temp_dir.path().join("vfs_absolute");
        assert!(!root_path.exists());

        // First initialization should succeed (creates root if missing)
        assert!(init_file_system(Some(&root_path)).is_ok());

        // Verify global state
        let fs = FILE_SYSTEM.get().expect("FS should be set");
        assert_eq!(fs.root(), &canonicalize(&root_path).unwrap());

        // Second initialization should fail (OnceLock already set)
        let result = init_file_system(Some(&root_path));
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("already initialized")
        );
    }
}
