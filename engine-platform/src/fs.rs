use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

pub struct FileSystem {
    search_paths: Vec<PathBuf>,
    mounted_roots: HashMap<String, PathBuf>,
}

impl Default for FileSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl FileSystem {
    pub fn new() -> Self {
        Self {
            search_paths: Vec::new(),
            mounted_roots: HashMap::new(),
        }
    }

    pub fn add_search_path(&mut self, path: impl Into<PathBuf>) {
        self.search_paths.push(path.into());
    }

    pub fn mount(&mut self, name: &str, path: impl Into<PathBuf>) {
        self.mounted_roots.insert(name.to_string(), path.into());
    }

    pub fn read_file(&self, path: impl AsRef<Path>) -> std::io::Result<Vec<u8>> {
        let path = self.resolve_path(path.as_ref())?;
        let mut file = File::open(&path)?;
        let mut content = Vec::new();
        file.read_to_end(&mut content)?;
        Ok(content)
    }

    pub fn read_file_to_string(&self, path: impl AsRef<Path>) -> std::io::Result<String> {
        let bytes = self.read_file(path)?;
        String::from_utf8(bytes).map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("UTF-8 decode error: {}", e),
            )
        })
    }

    pub fn write_file(&self, path: impl AsRef<Path>, content: &[u8]) -> std::io::Result<()> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = File::create(path)?;
        file.write_all(content)?;
        Ok(())
    }

    pub fn write_file_from_string(
        &self,
        path: impl AsRef<Path>,
        content: &str,
    ) -> std::io::Result<()> {
        self.write_file(path, content.as_bytes())
    }

    pub fn list_dir(&self, path: impl AsRef<Path>) -> std::io::Result<Vec<PathBuf>> {
        let path = self.resolve_path(path.as_ref())?;
        let mut entries = Vec::new();
        for entry in fs::read_dir(path)? {
            entries.push(entry?.path());
        }
        entries.sort();
        Ok(entries)
    }

    pub fn exists(&self, path: impl AsRef<Path>) -> bool {
        match self.resolve_path(path.as_ref()) {
            Ok(resolved) => resolved.exists(),
            Err(_) => false,
        }
    }

    pub fn is_dir(&self, path: impl AsRef<Path>) -> bool {
        match self.resolve_path(path.as_ref()) {
            Ok(resolved) => resolved.is_dir(),
            Err(_) => false,
        }
    }

    pub fn is_file(&self, path: impl AsRef<Path>) -> bool {
        match self.resolve_path(path.as_ref()) {
            Ok(resolved) => resolved.is_file(),
            Err(_) => false,
        }
    }

    pub fn create_dir_all(&self, path: impl AsRef<Path>) -> std::io::Result<()> {
        fs::create_dir_all(path.as_ref())
    }

    pub fn remove_file(&self, path: impl AsRef<Path>) -> std::io::Result<()> {
        fs::remove_file(path.as_ref())
    }

    pub fn remove_dir_all(&self, path: impl AsRef<Path>) -> std::io::Result<()> {
        fs::remove_dir_all(path.as_ref())
    }

    pub fn canonicalize(&self, path: impl AsRef<Path>) -> std::io::Result<PathBuf> {
        fs::canonicalize(path.as_ref())
    }

    pub fn current_dir(&self) -> std::io::Result<PathBuf> {
        std::env::current_dir()
    }

    pub fn set_current_dir(&self, path: impl AsRef<Path>) -> std::io::Result<()> {
        std::env::set_current_dir(path.as_ref())
    }

    fn resolve_path(&self, path: &Path) -> std::io::Result<PathBuf> {
        if path.is_absolute() {
            return Ok(path.to_path_buf());
        }

        for search_path in &self.search_paths {
            let full_path = search_path.join(path);
            if full_path.exists() {
                return Ok(full_path);
            }
        }

        Ok(path.to_path_buf())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_read_file_to_string() {
        let fs = FileSystem::new();
        let path = env::current_dir().unwrap().join("Cargo.toml");
        if path.exists() {
            let content = fs.read_file_to_string(&path).unwrap();
            assert!(!content.is_empty());
        }
    }

    #[test]
    fn test_exists() {
        let fs = FileSystem::new();
        assert!(fs.exists("."));
    }

    #[test]
    fn test_is_dir() {
        let fs = FileSystem::new();
        assert!(fs.is_dir("."));
    }

    #[test]
    fn test_add_search_path() {
        let mut fs = FileSystem::new();
        fs.add_search_path(".");
        assert!(fs.exists("."));
    }
}
