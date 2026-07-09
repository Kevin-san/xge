use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub struct FileSystem;

impl Default for FileSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl FileSystem {
    pub fn new() -> Self {
        Self
    }

    pub fn read(&self, path: &Path) -> io::Result<Vec<u8>> {
        fs::read(path)
    }

    pub fn read_string(&self, path: &Path) -> io::Result<String> {
        fs::read_to_string(path)
    }

    pub fn write(&self, path: &Path, bytes: &[u8]) -> io::Result<()> {
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)?;
            }
        }
        fs::write(path, bytes)
    }

    pub fn write_string(&self, path: &Path, s: &str) -> io::Result<()> {
        self.write(path, s.as_bytes())
    }

    pub fn exists(&self, path: &Path) -> bool {
        path.exists()
    }

    pub fn list_dir(&self, path: &Path) -> io::Result<Vec<PathBuf>> {
        let mut entries = Vec::new();
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            entries.push(entry.path());
        }
        Ok(entries)
    }

    pub fn create_dir_all(&self, path: &Path) -> io::Result<()> {
        fs::create_dir_all(path)
    }

    pub fn remove_file(&self, path: &Path) -> io::Result<()> {
        fs::remove_file(path)
    }

    pub fn is_dir(&self, path: &Path) -> bool {
        path.is_dir()
    }

    pub fn is_file(&self, path: &Path) -> bool {
        path.is_file()
    }

    pub fn canonicalize(&self, path: &Path) -> io::Result<PathBuf> {
        fs::canonicalize(path)
    }

    pub fn copy(&self, from: &Path, to: &Path) -> io::Result<u64> {
        if let Some(parent) = to.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)?;
            }
        }
        fs::copy(from, to)
    }

    pub fn remove_dir_all(&self, path: &Path) -> io::Result<()> {
        fs::remove_dir_all(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn temp_dir() -> PathBuf {
        let mut dir = env::temp_dir();
        dir.push(format!("engine_fs_test_{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn test_new() {
        let fs = FileSystem::new();
        assert!(fs.exists(&env::current_dir().unwrap()));
    }

    #[test]
    fn test_default() {
        let _fs: FileSystem = Default::default();
    }

    #[test]
    fn test_write_and_read_bytes() {
        let dir = temp_dir();
        let path = dir.join("test.bin");
        let fs = FileSystem::new();

        let data = vec![1, 2, 3, 4, 5];
        fs.write(&path, &data).unwrap();
        let read = fs.read(&path).unwrap();
        assert_eq!(read, data);

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_write_and_read_string() {
        let dir = temp_dir();
        let path = dir.join("test.txt");
        let fs = FileSystem::new();

        let content = "Hello, World!";
        fs.write_string(&path, content).unwrap();
        let read = fs.read_string(&path).unwrap();
        assert_eq!(read, content);

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_exists() {
        let dir = temp_dir();
        let path = dir.join("exists.txt");
        let fs = FileSystem::new();

        assert!(!fs.exists(&path));
        fs.write_string(&path, "test").unwrap();
        assert!(fs.exists(&path));

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_is_dir_and_is_file() {
        let dir = temp_dir();
        let file_path = dir.join("test.txt");
        let fs = FileSystem::new();

        assert!(fs.is_dir(&dir));
        assert!(!fs.is_file(&dir));

        fs.write_string(&file_path, "test").unwrap();
        assert!(fs.is_file(&file_path));
        assert!(!fs.is_dir(&file_path));

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_list_dir() {
        let dir = temp_dir();
        let fs = FileSystem::new();

        let file1 = dir.join("a.txt");
        let file2 = dir.join("b.txt");
        fs.write_string(&file1, "a").unwrap();
        fs.write_string(&file2, "b").unwrap();

        let entries = fs.list_dir(&dir).unwrap();
        assert_eq!(entries.len(), 2);

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_create_dir_all() {
        let dir = temp_dir();
        let nested = dir.join("a").join("b").join("c");
        let fs = FileSystem::new();

        fs.create_dir_all(&nested).unwrap();
        assert!(fs.is_dir(&nested));

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_remove_file() {
        let dir = temp_dir();
        let path = dir.join("remove.txt");
        let fs = FileSystem::new();

        fs.write_string(&path, "test").unwrap();
        assert!(fs.exists(&path));
        fs.remove_file(&path).unwrap();
        assert!(!fs.exists(&path));

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_copy() {
        let dir = temp_dir();
        let src = dir.join("src.txt");
        let dst = dir.join("dst.txt");
        let fs = FileSystem::new();

        fs.write_string(&src, "copy test").unwrap();
        fs.copy(&src, &dst).unwrap();
        assert!(fs.exists(&dst));
        assert_eq!(fs.read_string(&dst).unwrap(), "copy test");

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_remove_dir_all() {
        let dir = temp_dir();
        let nested = dir.join("nested");
        let file = nested.join("file.txt");
        let fs = FileSystem::new();

        fs.create_dir_all(&nested).unwrap();
        fs.write_string(&file, "test").unwrap();
        assert!(fs.exists(&file));

        fs.remove_dir_all(&nested).unwrap();
        assert!(!fs.exists(&nested));

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_write_creates_parent_dirs() {
        let dir = temp_dir();
        let path = dir.join("a").join("b").join("c.txt");
        let fs = FileSystem::new();

        fs.write_string(&path, "nested").unwrap();
        assert!(fs.exists(&path));
        assert_eq!(fs.read_string(&path).unwrap(), "nested");

        let _ = fs::remove_dir_all(&dir);
    }
}
