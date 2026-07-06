use std::collections::LinkedList;
use std::convert::TryFrom;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::string::FromUtf16Error;

const UNC_PREFIX: &str = "\\\\?\\";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowsPath {
    drive: Option<char>,
    folders: LinkedList<String>,
}

impl WindowsPath {
    /// Creates a new empty WindowsPath
    pub fn new() -> Self {
        Self {
            drive: None,
            folders: LinkedList::new(),
        }
    }

    /// Creates a WindowsPath from a std::path::Path
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, FromUtf16Error> {
        let path = path.as_ref();

        // Convert path to UTF-16 string (Windows native encoding)
        let mut utf16_chars = Vec::new();
        for c in path.to_string_lossy().chars() {
            let mut buf = [0; 2];
            let encoded = c.encode_utf16(&mut buf);
            for &mut code_unit in encoded {
                utf16_chars.push(code_unit);
            }
        }

        Self::from_utf16(&utf16_chars)
    }

    /// Creates a WindowsPath from UTF-16 bytes
    pub fn from_utf16(bytes: &[u16]) -> Result<Self, FromUtf16Error> {
        let string = String::from_utf16(bytes)?;
        Ok(Self::from_string(&string))
    }

    /// Creates a WindowsPath from a string
    pub fn from_string(s: &str) -> Self {
        let stripped = Self::strip_unc_prefix(s);
        let mut path = Self::new();
        let mut current_folder = String::new();

        let mut chars = stripped.chars().peekable();

        // Handle drive letter
        if let Some(first_char) = chars.peek().copied() {
            if first_char.is_ascii_alphabetic() {
                if let Some(':') = chars.clone().nth(1) {
                    path.drive = Some(first_char.to_ascii_lowercase());
                    chars.next(); // Skip drive letter
                    chars.next(); // Skip colon

                    // Skip leading slash if present
                    if chars.peek() == Some(&'\\') || chars.peek() == Some(&'/') {
                        chars.next();
                    }
                }
            }
        }

        // Parse folders
        for c in chars {
            if c == '\\' || c == '/' {
                if !current_folder.is_empty() {
                    path.folders.push_back(current_folder.to_ascii_lowercase());
                    current_folder = String::new();
                }
            } else {
                current_folder.push(c);
            }
        }

        // Add last folder if any
        if !current_folder.is_empty() {
            path.folders.push_back(current_folder.to_ascii_lowercase());
        }

        // If absolute path with no folders, ensure root is represented
        if path.drive.is_some() && path.folders.is_empty() {
            path.folders.push_back(String::new()); // Represent root as empty folder
        }

        path
    }

    /// Creates a WindowsPath from drive and folders
    pub fn from_parts(drive: Option<char>, folders: LinkedList<String>) -> Self {
        let mut path = Self { drive, folders };
        path.canonicalize();
        path
    }

    /// Checks if the path is absolute (has a drive letter)
    pub fn is_absolute(&self) -> bool {
        self.drive.is_some()
    }

    /// Checks if the path is relative (no drive letter)
    pub fn is_relative(&self) -> bool {
        !self.is_absolute()
    }

    /// Returns the path as a UTF-16 string
    pub fn to_utf16_string(&self) -> Vec<u16> {
        self.to_string().encode_utf16().collect()
    }

    fn as_windows_string(&self) -> String {
        let mut path = String::new();

        if let Some(drive) = self.drive {
            path.push(drive);
            path.push_str(":\\");
        }

        let mut first = true;
        for folder in &self.folders {
            if !first || (self.is_absolute() && folder.is_empty()) {
                path.push('\\');
            }
            first = false;

            if !folder.is_empty() {
                path.push_str(folder);
            }
        }

        // Handle root path (drive:\)
        if self.is_absolute() && path.len() == 2 {
            path.push('\\');
        }

        path
    }

    /// Returns the UNC path representation
    pub fn to_unc_path(&self) -> String {
        if self.is_relative() {
            self.to_string()
        } else {
            format!("{}{}", UNC_PREFIX, self)
        }
    }

    /// Returns a portable path (with forward slashes)
    pub fn to_portable_path(&self) -> PathBuf {
        let mut path = String::new();

        if let Some(drive) = self.drive {
            path.push(drive);
        }

        for folder in &self.folders {
            path.push('/');

            if !folder.is_empty() {
                path.push_str(folder);
            }
        }

        PathBuf::from(path)
    }

    /// Returns the device path (e.g., \Device\HarddiskVolumeX\...)
    pub fn to_device_path(&self) -> Result<String, &'static str> {
        if self.is_relative() {
            return Err("Device path cannot be computed for relative paths!");
        }

        let drive_index = self.drive.unwrap() as u8 - b'a';
        let drive_number = (drive_index + 1).to_string();

        let mut path = format!("\\Device\\HarddiskVolume{}\\", drive_number);

        let relative_path = self.without_drive().to_string();
        path.push_str(&relative_path);
        Ok(path)
    }

    /// Returns the drive letter if present
    pub fn get_drive(&self) -> Option<char> {
        self.drive
    }

    /// Returns a path without the drive letter
    pub fn without_drive(&self) -> Self {
        Self {
            drive: None,
            folders: self.folders.clone(),
        }
    }

    /// Returns the parent path
    pub fn parent(&self) -> Self {
        let mut folders = self.folders.clone();
        if !folders.is_empty() {
            folders.pop_back();
        }

        Self {
            drive: self.drive,
            folders,
        }
    }

    /// Returns the last component of the path
    pub fn leaf(&self) -> Option<&str> {
        self.folders.back().map(|s| s.as_str())
    }

    /// Appends another path to this one
    pub fn join(&self, other: &Self) -> Self {
        if other.is_absolute() {
            return other.clone();
        }

        let mut folders = self.folders.clone();
        for folder in &other.folders {
            folders.push_back(folder.clone());
        }

        Self {
            drive: self.drive,
            folders,
        }
    }

    /// Checks if the path is empty
    pub fn is_empty(&self) -> bool {
        self.is_relative() && self.folders.is_empty()
    }

    /// Canonicalizes the path (lowercases drive and folders)
    fn canonicalize(&mut self) {
        if let Some(drive) = &mut self.drive {
            *drive = drive.to_ascii_lowercase();
        }

        for folder in &mut self.folders {
            *folder = folder.to_ascii_lowercase();
        }
    }

    /// Strips UNC prefix from a string
    fn strip_unc_prefix(s: &str) -> &str {
        if s.starts_with(UNC_PREFIX) {
            &s[UNC_PREFIX.len()..]
        } else {
            s
        }
    }
}

impl Default for WindowsPath {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for WindowsPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_windows_string())
    }
}

impl Hash for WindowsPath {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.drive.hash(state);
        for folder in &self.folders {
            folder.hash(state);
        }
    }
}

impl std::ops::Div for &WindowsPath {
    type Output = WindowsPath;

    fn div(self, rhs: Self) -> Self::Output {
        self.join(rhs)
    }
}

impl std::ops::DivAssign for WindowsPath {
    fn div_assign(&mut self, rhs: Self) {
        *self = self.join(&rhs);
    }
}

// TryFrom implementations for common types
impl TryFrom<&Path> for WindowsPath {
    type Error = FromUtf16Error;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        Self::from_path(value)
    }
}

impl TryFrom<PathBuf> for WindowsPath {
    type Error = FromUtf16Error;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        Self::from_path(value)
    }
}

impl TryFrom<&str> for WindowsPath {
    type Error = FromUtf16Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self::from_string(value))
    }
}

impl TryFrom<String> for WindowsPath {
    type Error = FromUtf16Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self::from_string(&value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_absolute_path() {
        let path = WindowsPath::from_string("C:\\Windows\\System32");
        assert!(path.is_absolute());
        assert_eq!(path.get_drive(), Some('c'));
        assert_eq!(path.leaf(), Some("system32"));
        assert_eq!(path.to_string(), "c:\\windows\\system32");
    }

    #[test]
    fn test_relative_path() {
        let path = WindowsPath::from_string("Users\\John\\Documents");
        assert!(path.is_relative());
        assert_eq!(path.get_drive(), None);
        assert_eq!(path.leaf(), Some("documents"));
    }

    #[test]
    fn test_unc_path() {
        let path = WindowsPath::from_string("C:\\Windows");
        assert_eq!(path.to_unc_path(), "\\\\?\\c:\\windows");
    }

    #[test]
    fn test_join_paths() {
        let path1 = WindowsPath::from_string("C:\\Windows");
        let path2 = WindowsPath::from_string("System32");
        let joined = path1.join(&path2);
        assert_eq!(joined.to_string(), "c:\\windows\\system32");
    }

    #[test]
    fn test_parent() {
        let path = WindowsPath::from_string("C:\\Windows\\System32");
        let parent = path.parent();
        assert_eq!(parent.to_string(), "c:\\windows");
    }
}
