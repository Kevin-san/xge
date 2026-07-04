use std::path::{Path, PathBuf};

pub trait FileSystem: Send + Sync {
    fn read(&self, path: &Path) -> std::io::Result<Vec<u8>>;

    fn read_string(&self, path: &Path) -> std::io::Result<String> {
        let bytes = self.read(path)?;
        String::from_utf8(bytes).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }

    fn write(&self, path: &Path, bytes: &[u8]) -> std::io::Result<()>;

    fn write_string(&self, path: &Path, s: &str) -> std::io::Result<()> {
        self.write(path, s.as_bytes())
    }

    fn exists(&self, path: &Path) -> bool;

    fn list_dir(&self, path: &Path) -> std::io::Result<Vec<PathBuf>>;

    fn create_dir_all(&self, path: &Path) -> std::io::Result<()>;

    fn remove_file(&self, path: &Path) -> std::io::Result<()>;

    fn is_dir(&self, path: &Path) -> bool;

    fn canonicalize(&self, path: &Path) -> std::io::Result<PathBuf>;
}

pub struct NativeFileSystem;

impl FileSystem for NativeFileSystem {
    fn read(&self, path: &Path) -> std::io::Result<Vec<u8>> {
        std::fs::read(path)
    }

    fn write(&self, path: &Path, bytes: &[u8]) -> std::io::Result<()> {
        std::fs::write(path, bytes)
    }

    fn exists(&self, path: &Path) -> bool {
        std::fs::metadata(path).map(|m| m.is_file() || m.is_dir()).unwrap_or(false)
    }

    fn list_dir(&self, path: &Path) -> std::io::Result<Vec<PathBuf>> {
        let mut entries = Vec::new();
        for entry in std::fs::read_dir(path)? {
            entries.push(entry?.path());
        }
        Ok(entries)
    }

    fn create_dir_all(&self, path: &Path) -> std::io::Result<()> {
        std::fs::create_dir_all(path)
    }

    fn remove_file(&self, path: &Path) -> std::io::Result<()> {
        std::fs::remove_file(path)
    }

    fn is_dir(&self, path: &Path) -> bool {
        std::fs::metadata(path).map(|m| m.is_dir()).unwrap_or(false)
    }

    fn canonicalize(&self, path: &Path) -> std::io::Result<PathBuf> {
        std::fs::canonicalize(path)
    }
}

impl Default for NativeFileSystem {
    fn default() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::temp_dir;

    #[test]
    fn test_write_and_read() {
        let fs = NativeFileSystem;
        let temp_path = temp_dir().join("engine_fs_test.txt");
        
        fs.write_string(&temp_path, "Hello, FileSystem!").unwrap();
        let content = fs.read_string(&temp_path).unwrap();
        assert_eq!(content, "Hello, FileSystem!");
        
        std::fs::remove_file(&temp_path).ok();
    }

    #[test]
    fn test_exists() {
        let fs = NativeFileSystem;
        let temp_path = temp_dir().join("nonexistent_file_xyz123.txt");
        assert!(!fs.exists(&temp_path));
        
        fs.write_string(&temp_path, "test").unwrap();
        assert!(fs.exists(&temp_path));
        
        std::fs::remove_file(&temp_path).ok();
    }

    #[test]
    fn test_create_dir_all() {
        let fs = NativeFileSystem;
        let temp_dir = temp_dir().join("engine_fs_test_dir");
        
        fs.create_dir_all(&temp_dir).unwrap();
        assert!(fs.is_dir(&temp_dir));
        
        std::fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_is_dir() {
        let fs = NativeFileSystem;
        let temp_dir = temp_dir().join("engine_fs_is_dir_test");
        fs.create_dir_all(&temp_dir).unwrap();
        
        assert!(fs.is_dir(&temp_dir));
        assert!(!fs.is_dir(&temp_dir.join("nonexistent.txt")));
        
        std::fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_list_dir() {
        let fs = NativeFileSystem;
        let temp_dir = temp_dir().join("engine_fs_list_test");
        fs.create_dir_all(&temp_dir).unwrap();
        
        fs.write_string(&temp_dir.join("file1.txt"), "a").unwrap();
        fs.write_string(&temp_dir.join("file2.txt"), "b").unwrap();
        
        let entries = fs.list_dir(&temp_dir).unwrap();
        assert!(entries.len() >= 2);
        
        std::fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_remove_file() {
        let fs = NativeFileSystem;
        let temp_path = temp_dir().join("engine_fs_remove_test.txt");
        
        fs.write_string(&temp_path, "test").unwrap();
        assert!(fs.exists(&temp_path));
        
        fs.remove_file(&temp_path).unwrap();
        assert!(!fs.exists(&temp_path));
    }
}