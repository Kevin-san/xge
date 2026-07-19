use std::io::Result;
use std::path::{Path, PathBuf};

/// 文件系统抽象 trait
///
/// 提供跨平台的文件读写和目录操作接口
pub trait FileSystem: Send + Sync {
    fn read(&self, path: &Path) -> Result<Vec<u8>>;
    fn read_string(&self, path: &Path) -> Result<String>;
    fn write(&self, path: &Path, data: &[u8]) -> Result<()>;
    fn write_string(&self, path: &Path, content: &str) -> Result<()>;
    fn exists(&self, path: &Path) -> bool;
    fn list_dir(&self, path: &Path) -> Result<Vec<PathBuf>>;
    fn create_dir_all(&self, path: &Path) -> Result<()>;
    fn remove_file(&self, path: &Path) -> Result<()>;
    fn is_dir(&self, path: &Path) -> bool;
    fn canonicalize(&self, path: &Path) -> Result<PathBuf>;
}

/// 默认的基于 std::fs 的文件系统实现
pub struct NativeFileSystem;

impl FileSystem for NativeFileSystem {
    fn read(&self, path: &Path) -> Result<Vec<u8>> {
        std::fs::read(path)
    }

    fn read_string(&self, path: &Path) -> Result<String> {
        std::fs::read_to_string(path)
    }

    fn write(&self, path: &Path, data: &[u8]) -> Result<()> {
        std::fs::write(path, data)
    }

    fn write_string(&self, path: &Path, content: &str) -> Result<()> {
        std::fs::write(path, content)
    }

    fn exists(&self, path: &Path) -> bool {
        path.exists()
    }

    fn list_dir(&self, path: &Path) -> Result<Vec<PathBuf>> {
        let mut entries = Vec::new();
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            entries.push(entry.path());
        }
        Ok(entries)
    }

    fn create_dir_all(&self, path: &Path) -> Result<()> {
        std::fs::create_dir_all(path)
    }

    fn remove_file(&self, path: &Path) -> Result<()> {
        std::fs::remove_file(path)
    }

    fn is_dir(&self, path: &Path) -> bool {
        path.is_dir()
    }

    fn canonicalize(&self, path: &Path) -> Result<PathBuf> {
        std::fs::canonicalize(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_write_and_read() {
        let dir = std::env::temp_dir().join("engine_fs_test_write_read");
        let _ = fs::create_dir_all(&dir);
        let file_path = dir.join("test.txt");

        let fs = NativeFileSystem;
        fs.write_string(&file_path, "hello engine").unwrap();
        let content = fs.read_string(&file_path).unwrap();
        assert_eq!(content, "hello engine");

        let _ = fs::remove_file(&file_path);
        let _ = fs::remove_dir(&dir);
    }

    #[test]
    fn test_exists() {
        let fs = NativeFileSystem;
        assert!(fs.exists(Path::new(".")));
        assert!(!fs.exists(Path::new("/nonexistent_engine_test_path_12345")));
    }

    #[test]
    fn test_is_dir() {
        let fs = NativeFileSystem;
        assert!(fs.is_dir(Path::new(".")));
        assert!(!fs.is_dir(Path::new("Cargo.toml")));
    }

    #[test]
    fn test_create_dir_all_and_remove() {
        let dir = std::env::temp_dir().join("engine_fs_test_mkdir").join("nested");
        let fs = NativeFileSystem;
        fs.create_dir_all(&dir).unwrap();
        assert!(dir.exists());

        let _ = fs::remove_dir_all(dir.parent().unwrap());
    }

    #[test]
    fn test_read_bytes() {
        let dir = std::env::temp_dir().join("engine_fs_test_bytes");
        let _ = fs::create_dir_all(&dir);
        let file_path = dir.join("test.bin");

        let fs = NativeFileSystem;
        let data = vec![0u8, 1, 2, 3, 255];
        fs.write(&file_path, &data).unwrap();
        let read_data = fs.read(&file_path).unwrap();
        assert_eq!(read_data, data);

        let _ = fs::remove_file(&file_path);
        let _ = fs::remove_dir(&dir);
    }

    #[test]
    fn test_list_dir() {
        let fs = NativeFileSystem;
        let result = fs.list_dir(Path::new("src"));
        assert!(result.is_ok());
        assert!(!result.unwrap().is_empty());
    }
}
