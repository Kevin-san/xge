//! Build cache for incremental builds
//!
//! Provides caching mechanism to speed up incremental builds.

use crate::BuildResult;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Build cache for incremental builds
pub struct BuildCache {
    cache_dir: PathBuf,
    index: HashMap<String, PathBuf>,
}

impl BuildCache {
    /// Create new build cache
    pub fn new(cache_dir: impl AsRef<Path>) -> BuildResult<Self> {
        let dir = cache_dir.as_ref().to_path_buf();
        if !dir.exists() {
            fs::create_dir_all(&dir)?;
        }
        Ok(Self {
            cache_dir: dir,
            index: HashMap::new(),
        })
    }

    /// Create default cache (in-memory only)
    pub fn new_default() -> Self {
        Self {
            cache_dir: PathBuf::new(),
            index: HashMap::new(),
        }
    }

    /// Compute hash for cache key
    pub fn hash(&self, key: &str) -> String {
        crate::Hash::sha256(key.as_bytes())
    }

    /// Get cached item
    pub fn get(&self, key: &str) -> Option<PathBuf> {
        let hash = self.hash(key);
        self.index.get(&hash).cloned().or_else(|| {
            let cache_path = self.cache_dir.join(&hash);
            if cache_path.exists() {
                Some(cache_path)
            } else {
                None
            }
        })
    }

    /// Put item into cache
    pub fn put(&mut self, key: &str, path: impl AsRef<Path>) {
        let hash = self.hash(key);
        let cache_path = self.cache_dir.join(&hash);
        let src_path = path.as_ref();

        if src_path.exists() && !self.cache_dir.as_os_str().is_empty() {
            // Copy file to cache
            if fs::copy(src_path, &cache_path).is_ok() {
                self.index.insert(hash, cache_path);
            }
        } else {
            // Just store the path reference
            self.index.insert(hash, src_path.to_path_buf());
        }
    }

    /// Check if key exists in cache
    pub fn contains(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    /// Clear cache
    pub fn clean(&mut self) {
        self.index.clear();
        if self.cache_dir.exists() && fs::remove_dir_all(&self.cache_dir).is_ok() {
            fs::create_dir_all(&self.cache_dir).ok();
        }
    }

    /// Get cache directory
    pub fn cache_dir(&self) -> &Path {
        &self.cache_dir
    }

    /// Get cache size (number of entries)
    pub fn size(&self) -> usize {
        self.index.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_build_cache_new() {
        let dir = tempdir().unwrap();
        let _cache = BuildCache::new(dir.path()).unwrap();
        assert!(dir.path().exists());
    }

    #[test]
    fn test_build_cache_hash() {
        let cache = BuildCache::new_default();
        let hash1 = cache.hash("test_key");
        let hash2 = cache.hash("test_key");
        assert_eq!(hash1, hash2);
        assert_eq!(hash1.len(), 64); // SHA256 hex length
    }

    #[test]
    fn test_build_cache_put_get() {
        let mut cache = BuildCache::new_default();
        cache.put("test_key", PathBuf::from("/test/path"));
        assert!(cache.contains("test_key"));
        let result = cache.get("test_key");
        assert!(result.is_some());
    }

    #[test]
    fn test_build_cache_clean() {
        let mut cache = BuildCache::new_default();
        cache.put("key1", PathBuf::from("/path1"));
        cache.put("key2", PathBuf::from("/path2"));
        assert_eq!(cache.size(), 2);
        cache.clean();
        assert_eq!(cache.size(), 0);
    }
}
