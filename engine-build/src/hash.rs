//! Hash computation utilities
//!
//! Provides SHA256 hashing for files and directories.

use crate::BuildResult;
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Read;
use std::path::Path;
use walkdir::WalkDir;

/// Hash computation utilities
pub struct Hash;

impl Hash {
    /// Compute SHA256 hash of bytes
    pub fn sha256(bytes: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(bytes);
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    /// Compute SHA256 hash of a file
    pub fn hash_file(path: impl AsRef<Path>) -> BuildResult<String> {
        let mut file = fs::File::open(path.as_ref())?;
        let mut hasher = Sha256::new();
        let mut buffer = [0u8; 8192];

        loop {
            let n = file.read(&mut buffer)?;
            if n == 0 {
                break;
            }
            hasher.update(&buffer[..n]);
        }

        let result = hasher.finalize();
        Ok(format!("{:x}", result))
    }

    /// Compute SHA256 hash of a directory (all files combined)
    pub fn hash_dir(path: impl AsRef<Path>) -> BuildResult<String> {
        let mut combined_hash = String::new();
        let mut entries: Vec<_> = WalkDir::new(path.as_ref())
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .collect();

        // Sort for deterministic order
        entries.sort_by_key(|e| e.path().to_path_buf());

        for entry in &entries {
            let file_hash = Self::hash_file(entry.path())?;
            combined_hash.push_str(&file_hash);
        }

        Ok(Self::sha256(combined_hash.as_bytes()))
    }

    /// Compute hash with prefix for identification
    pub fn hash_with_prefix(prefix: &str, bytes: &[u8]) -> String {
        let combined = format!("{}:{}", prefix, Self::sha256(bytes));
        Self::sha256(combined.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_sha256_bytes() {
        let hash = Hash::sha256(b"hello");
        assert_eq!(hash.len(), 64);
        // Known SHA256 of "hello"
        assert_eq!(
            hash,
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
    }

    #[test]
    fn test_sha256_empty() {
        let hash = Hash::sha256(b"");
        assert_eq!(
            hash,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn test_hash_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        fs::write(&file_path, b"test content").unwrap();

        let hash = Hash::hash_file(&file_path).unwrap();
        assert_eq!(hash.len(), 64);
    }

    #[test]
    fn test_hash_dir() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("a.txt"), b"content a").unwrap();
        fs::write(dir.path().join("b.txt"), b"content b").unwrap();

        let hash = Hash::hash_dir(dir.path()).unwrap();
        assert_eq!(hash.len(), 64);
    }

    #[test]
    fn test_hash_consistency() {
        let hash1 = Hash::sha256(b"test");
        let hash2 = Hash::sha256(b"test");
        assert_eq!(hash1, hash2);
    }
}
