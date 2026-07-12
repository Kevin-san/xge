use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct FileSystemError(pub String);

impl std::fmt::Display for FileSystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for FileSystemError {}

pub struct FileSystem;

impl FileSystem {
    pub fn new() -> Self {
        Self
    }

    pub fn read(&self, path: impl AsRef<Path>) -> Result<Vec<u8>, FileSystemError> {
        std::fs::read(path.as_ref())
            .map_err(|e| FileSystemError(format!("Failed to read file: {}", e)))
    }

    pub fn read_string(&self, path: impl AsRef<Path>) -> Result<String, FileSystemError> {
        std::fs::read_to_string(path.as_ref())
            .map_err(|e| FileSystemError(format!("Failed to read file as string: {}", e)))
    }

    pub fn write(&self, path: impl AsRef<Path>, bytes: &[u8]) -> Result<(), FileSystemError> {
        std::fs::write(path.as_ref(), bytes)
            .map_err(|e| FileSystemError(format!("Failed to write file: {}", e)))
    }

    pub fn write_string(&self, path: impl AsRef<Path>, s: &str) -> Result<(), FileSystemError> {
        std::fs::write(path.as_ref(), s.as_bytes())
            .map_err(|e| FileSystemError(format!("Failed to write string to file: {}", e)))
    }

    pub fn exists(&self, path: impl AsRef<Path>) -> bool {
        path.as_ref().exists()
    }

    pub fn list_dir(&self, path: impl AsRef<Path>) -> Result<Vec<PathBuf>, FileSystemError> {
        let entries = std::fs::read_dir(path.as_ref())
            .map_err(|e| FileSystemError(format!("Failed to read directory: {}", e)))?;

        let mut paths = Vec::new();
        for entry in entries {
            let entry = entry.map_err(|e| FileSystemError(format!("Failed to read directory entry: {}", e)))?;
            paths.push(entry.path());
        }
        Ok(paths)
    }

    pub fn create_dir_all(&self, path: impl AsRef<Path>) -> Result<(), FileSystemError> {
        std::fs::create_dir_all(path.as_ref())
            .map_err(|e| FileSystemError(format!("Failed to create directory: {}", e)))
    }

    pub fn remove_file(&self, path: impl AsRef<Path>) -> Result<(), FileSystemError> {
        std::fs::remove_file(path.as_ref())
            .map_err(|e| FileSystemError(format!("Failed to remove file: {}", e)))
    }

    pub fn is_dir(&self, path: impl AsRef<Path>) -> bool {
        path.as_ref().is_dir()
    }

    pub fn canonicalize(&self, path: impl AsRef<Path>) -> Result<PathBuf, FileSystemError> {
        std::fs::canonicalize(path.as_ref())
            .map_err(|e| FileSystemError(format!("Failed to canonicalize path: {}", e)))
    }
}

impl Default for FileSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_read_write() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.txt");
        let content = b"hello world";

        let fs = FileSystem::new();
        fs.write(&path, content).unwrap();
        let read_content = fs.read(&path).unwrap();

        assert_eq!(read_content, content);
    }

    #[test]
    fn test_read_string() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.txt");
        
        fs::write(&path, "hello world").unwrap();
        
        let fs = FileSystem::new();
        let content = fs.read_string(&path).unwrap();

        assert_eq!(content, "hello world");
    }

    #[test]
    fn test_write_string() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.txt");

        let fs = FileSystem::new();
        fs.write_string(&path, "hello world").unwrap();

        let content = fs::read_to_string(&path).unwrap();
        assert_eq!(content, "hello world");
    }

    #[test]
    fn test_exists() {
        let fs = FileSystem::new();
        assert!(fs.exists("Cargo.toml"));
        assert!(!fs.exists("nonexistent_file_xyz123.txt"));
    }

    #[test]
    fn test_list_dir() {
        let dir = tempdir().unwrap();
        
        fs::write(dir.path().join("a.txt"), "").unwrap();
        fs::write(dir.path().join("b.txt"), "").unwrap();
        
        let fs = FileSystem::new();
        let paths = fs.list_dir(dir.path()).unwrap();

        assert_eq!(paths.len(), 2);
    }

    #[test]
    fn test_create_dir_all() {
        let dir = tempdir().unwrap();
        let subdir = dir.path().join("a").join("b").join("c");

        let fs = FileSystem::new();
        fs.create_dir_all(&subdir).unwrap();

        assert!(subdir.exists());
    }

    #[test]
    fn test_remove_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.txt");
        
        fs::write(&path, "content").unwrap();
        assert!(path.exists());

        let fs = FileSystem::new();
        fs.remove_file(&path).unwrap();

        assert!(!path.exists());
    }

    #[test]
    fn test_is_dir() {
        let dir = tempdir().unwrap();
        
        let fs = FileSystem::new();
        assert!(fs.is_dir(dir.path()));
        assert!(!fs.is_dir("Cargo.toml"));
    }

    #[test]
    fn test_canonicalize() {
        let fs = FileSystem::new();
        let path = fs.canonicalize(".").unwrap();
        assert!(path.is_absolute());
    }
}