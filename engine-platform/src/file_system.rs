use std::path::{Path, PathBuf};
use std::fs;

pub trait FileSystem: Send + Sync {
    fn read(&self, path: &Path) -> Result<Vec<u8>, std::io::Error>;
    fn read_string(&self, path: &Path) -> Result<String, std::io::Error>;
    fn write(&self, path: &Path, bytes: &[u8]) -> Result<(), std::io::Error>;
    fn write_string(&self, path: &Path, s: &str) -> Result<(), std::io::Error>;
    fn exists(&self, path: &Path) -> bool;
    fn list_dir(&self, path: &Path) -> Result<Vec<PathBuf>, std::io::Error>;
    fn create_dir_all(&self, path: &Path) -> Result<(), std::io::Error>;
    fn remove_file(&self, path: &Path) -> Result<(), std::io::Error>;
    fn is_dir(&self, path: &Path) -> bool;
    fn canonicalize(&self, path: &Path) -> Result<PathBuf, std::io::Error>;
}

pub struct NativeFileSystem;

impl FileSystem for NativeFileSystem {
    fn read(&self, path: &Path) -> Result<Vec<u8>, std::io::Error> {
        fs::read(path)
    }

    fn read_string(&self, path: &Path) -> Result<String, std::io::Error> {
        fs::read_to_string(path)
    }

    fn write(&self, path: &Path, bytes: &[u8]) -> Result<(), std::io::Error> {
        fs::write(path, bytes)
    }

    fn write_string(&self, path: &Path, s: &str) -> Result<(), std::io::Error> {
        fs::write(path, s)
    }

    fn exists(&self, path: &Path) -> bool {
        path.exists()
    }

    fn list_dir(&self, path: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
        let mut result = Vec::new();
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            result.push(entry.path());
        }
        Ok(result)
    }

    fn create_dir_all(&self, path: &Path) -> Result<(), std::io::Error> {
        fs::create_dir_all(path)
    }

    fn remove_file(&self, path: &Path) -> Result<(), std::io::Error> {
        fs::remove_file(path)
    }

    fn is_dir(&self, path: &Path) -> bool {
        path.is_dir()
    }

    fn canonicalize(&self, path: &Path) -> Result<PathBuf, std::io::Error> {
        fs::canonicalize(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn fs_read_write() {
        let temp_dir = tempdir().unwrap();
        let path = temp_dir.path().join("test.txt");
        let fs = NativeFileSystem;

        fs.write_string(&path, "hello world").unwrap();
        let content = fs.read_string(&path).unwrap();
        assert_eq!(content, "hello world");
    }

    #[test]
    fn fs_exists() {
        let temp_dir = tempdir().unwrap();
        let exists_path = temp_dir.path().join("exists.txt");
        let not_exists_path = temp_dir.path().join("not_exists.txt");
        let fs = NativeFileSystem;

        fs::write(&exists_path, "content").unwrap();
        assert!(fs.exists(&exists_path));
        assert!(!fs.exists(&not_exists_path));
    }

    #[test]
    fn fs_list_dir() {
        let temp_dir = tempdir().unwrap();
        let fs = NativeFileSystem;

        fs::write(temp_dir.path().join("a.txt"), "a").unwrap();
        fs::write(temp_dir.path().join("b.txt"), "b").unwrap();
        
        let files = fs.list_dir(temp_dir.path()).unwrap();
        assert_eq!(files.len(), 2);
    }

    #[test]
    fn fs_create_dir_all() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().join("nested/dir/path");
        let fs = NativeFileSystem;

        fs.create_dir_all(&dir_path).unwrap();
        assert!(fs.is_dir(&dir_path));
    }

    #[test]
    fn fs_remove_file() {
        let temp_dir = tempdir().unwrap();
        let path = temp_dir.path().join("to_remove.txt");
        let fs = NativeFileSystem;

        fs.write_string(&path, "content").unwrap();
        assert!(fs.exists(&path));
        
        fs.remove_file(&path).unwrap();
        assert!(!fs.exists(&path));
    }

    #[test]
    fn fs_is_dir() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().join("test_dir");
        let file_path = temp_dir.path().join("test.txt");
        let fs = NativeFileSystem;

        fs.create_dir_all(&dir_path).unwrap();
        fs.write_string(&file_path, "content").unwrap();
        
        assert!(fs.is_dir(&dir_path));
        assert!(!fs.is_dir(&file_path));
    }
}
