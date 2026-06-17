use std::path::{Path, PathBuf};

#[cfg(not(target_arch = "wasm32"))]
use std::fs as std_fs;

#[cfg(not(target_arch = "wasm32"))]
use std::io::{Read, Write};

#[derive(Debug)]
pub enum FileSystemError {
    IoError(std::io::Error),
    NotFound,
    PermissionDenied,
    InvalidPath,
    #[cfg(target_arch = "wasm32")]
    WebError(String),
}

impl std::fmt::Display for FileSystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileSystemError::IoError(e) => write!(f, "IO error: {}", e),
            FileSystemError::NotFound => write!(f, "File not found"),
            FileSystemError::PermissionDenied => write!(f, "Permission denied"),
            FileSystemError::InvalidPath => write!(f, "Invalid path"),
            #[cfg(target_arch = "wasm32")]
            FileSystemError::WebError(s) => write!(f, "Web error: {}", s),
        }
    }
}

impl std::error::Error for FileSystemError {}

impl From<std::io::Error> for FileSystemError {
    fn from(e: std::io::Error) -> Self {
        match e.kind() {
            std::io::ErrorKind::NotFound => FileSystemError::NotFound,
            std::io::ErrorKind::PermissionDenied => FileSystemError::PermissionDenied,
            _ => FileSystemError::IoError(e),
        }
    }
}

pub type Result<T> = std::result::Result<T, FileSystemError>;

pub trait FileSystem: Send + Sync {
    fn read(&self, path: &Path) -> Result<Vec<u8>>;
    fn read_string(&self, path: &Path) -> Result<String>;
    fn write(&self, path: &Path, bytes: &[u8]) -> Result<()>;
    fn write_string(&self, path: &Path, s: &str) -> Result<()>;
    fn exists(&self, path: &Path) -> bool;
    fn list_dir(&self, path: &Path) -> Result<Vec<PathBuf>>;
    fn create_dir_all(&self, path: &Path) -> Result<()>;
    fn remove_file(&self, path: &Path) -> Result<()>;
    fn is_dir(&self, path: &Path) -> bool;
    fn canonicalize(&self, path: &Path) -> Result<PathBuf>;
    fn file_size(&self, path: &Path) -> Result<u64>;
    fn last_modified(&self, path: &Path) -> Result<std::time::SystemTime>;
}

#[derive(Debug, Clone)]
pub struct StandardFileSystem;

#[cfg(not(target_arch = "wasm32"))]
impl FileSystem for StandardFileSystem {
    fn read(&self, path: &Path) -> Result<Vec<u8>> {
        Ok(std_fs::read(path)?)
    }

    fn read_string(&self, path: &Path) -> Result<String> {
        Ok(std_fs::read_to_string(path)?)
    }

    fn write(&self, path: &Path, bytes: &[u8]) -> Result<()> {
        std_fs::write(path, bytes)?;
        Ok(())
    }

    fn write_string(&self, path: &Path, s: &str) -> Result<()> {
        std_fs::write(path, s)?;
        Ok(())
    }

    fn exists(&self, path: &Path) -> bool {
        path.exists()
    }

    fn list_dir(&self, path: &Path) -> Result<Vec<PathBuf>> {
        let mut entries = Vec::new();
        for entry in std_fs::read_dir(path)? {
            entries.push(entry?.path());
        }
        Ok(entries)
    }

    fn create_dir_all(&self, path: &Path) -> Result<()> {
        std_fs::create_dir_all(path)?;
        Ok(())
    }

    fn remove_file(&self, path: &Path) -> Result<()> {
        std_fs::remove_file(path)?;
        Ok(())
    }

    fn is_dir(&self, path: &Path) -> bool {
        path.is_dir()
    }

    fn canonicalize(&self, path: &Path) -> Result<PathBuf> {
        Ok(std_fs::canonicalize(path)?)
    }

    fn file_size(&self, path: &Path) -> Result<u64> {
        Ok(std_fs::metadata(path)?.len())
    }

    fn last_modified(&self, path: &Path) -> Result<std::time::SystemTime> {
        Ok(std_fs::metadata(path)?.modified()?)
    }
}

#[cfg(target_arch = "wasm32")]
impl FileSystem for StandardFileSystem {
    fn read(&self, path: &Path) -> Result<Vec<u8>> {
        Err(FileSystemError::WebError(
            "Not implemented in WASM".to_string(),
        ))
    }

    fn read_string(&self, path: &Path) -> Result<String> {
        Err(FileSystemError::WebError(
            "Not implemented in WASM".to_string(),
        ))
    }

    fn write(&self, path: &Path, bytes: &[u8]) -> Result<()> {
        Err(FileSystemError::WebError(
            "Not implemented in WASM".to_string(),
        ))
    }

    fn write_string(&self, path: &Path, s: &str) -> Result<()> {
        Err(FileSystemError::WebError(
            "Not implemented in WASM".to_string(),
        ))
    }

    fn exists(&self, path: &Path) -> bool {
        false
    }

    fn list_dir(&self, path: &Path) -> Result<Vec<PathBuf>> {
        Err(FileSystemError::WebError(
            "Not implemented in WASM".to_string(),
        ))
    }

    fn create_dir_all(&self, path: &Path) -> Result<()> {
        Err(FileSystemError::WebError(
            "Not implemented in WASM".to_string(),
        ))
    }

    fn remove_file(&self, path: &Path) -> Result<()> {
        Err(FileSystemError::WebError(
            "Not implemented in WASM".to_string(),
        ))
    }

    fn is_dir(&self, path: &Path) -> bool {
        false
    }

    fn canonicalize(&self, path: &Path) -> Result<PathBuf> {
        Err(FileSystemError::WebError(
            "Not implemented in WASM".to_string(),
        ))
    }

    fn file_size(&self, path: &Path) -> Result<u64> {
        Err(FileSystemError::WebError(
            "Not implemented in WASM".to_string(),
        ))
    }

    fn last_modified(&self, path: &Path) -> Result<std::time::SystemTime> {
        Err(FileSystemError::WebError(
            "Not implemented in WASM".to_string(),
        ))
    }
}

impl Default for StandardFileSystem {
    fn default() -> Self {
        Self
    }
}

pub fn normalize_path(path: &Path) -> PathBuf {
    let mut components = Vec::new();
    for component in path.components() {
        match component {
            std::path::Component::ParentDir => {
                components.pop();
            }
            std::path::Component::CurDir => {}
            c => components.push(c.as_os_str()),
        }
    }

    let mut result = PathBuf::new();
    for component in components {
        result.push(component);
    }
    result
}

pub fn join_paths(base: &Path, path: &str) -> PathBuf {
    let normalized = normalize_path(Path::new(path));
    base.join(normalized)
}

#[cfg(test)]
#[cfg(not(target_arch = "wasm32"))]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_read_write() {
        let fs = StandardFileSystem;
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.txt");

        fs.write_string(&path, "Hello World").unwrap();
        let content = fs.read_string(&path).unwrap();
        assert_eq!(content, "Hello World");
    }

    #[test]
    fn test_exists() {
        let fs = StandardFileSystem;
        let dir = tempdir().unwrap();
        let existing = dir.path().join("existing.txt");
        fs.write_string(&existing, "test").unwrap();

        assert!(fs.exists(&existing));
        assert!(!fs.exists(&dir.path().join("nonexistent.txt")));
    }

    #[test]
    fn test_list_dir() {
        let fs = StandardFileSystem;
        let dir = tempdir().unwrap();
        fs.write_string(&dir.path().join("a.txt"), "a").unwrap();
        fs.write_string(&dir.path().join("b.txt"), "b").unwrap();

        let entries = fs.list_dir(dir.path()).unwrap();
        assert_eq!(entries.len(), 2);
    }

    #[test]
    fn test_create_dir_all() {
        let fs = StandardFileSystem;
        let dir = tempdir().unwrap();
        let nested = dir.path().join("a").join("b").join("c");

        fs.create_dir_all(&nested).unwrap();
        assert!(fs.is_dir(&nested));
    }

    #[test]
    fn test_remove_file() {
        let fs = StandardFileSystem;
        let dir = tempdir().unwrap();
        let path = dir.path().join("to_remove.txt");
        fs.write_string(&path, "test").unwrap();

        assert!(fs.exists(&path));
        fs.remove_file(&path).unwrap();
        assert!(!fs.exists(&path));
    }

    #[test]
    fn test_file_size() {
        let fs = StandardFileSystem;
        let dir = tempdir().unwrap();
        let path = dir.path().join("size_test.txt");
        fs.write_string(&path, "12345").unwrap();

        let size = fs.file_size(&path).unwrap();
        assert_eq!(size, 5);
    }

    #[test]
    fn test_normalize_path() {
        let path = Path::new("/a/b/../c");
        let normalized = normalize_path(path);
        assert_eq!(normalized, Path::new("/a/c"));
    }

    #[test]
    fn test_is_dir() {
        let fs = StandardFileSystem;
        let dir = tempdir().unwrap();
        assert!(fs.is_dir(dir.path()));
        assert!(!fs.is_dir(&dir.path().join("file.txt")));
    }
}