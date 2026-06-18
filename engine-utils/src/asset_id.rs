use core::fmt;
use core::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::path::Path;

/// 资源唯一标识符
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct AssetId {
    uuid: u64,
    path_hash: u64,
}

impl AssetId {
    pub fn new(uuid: u64) -> Self {
        Self { uuid, path_hash: 0 }
    }

    pub fn from_path(path: &Path) -> Self {
        use core::hash::Hash;

        let path_str = path.to_string_lossy();
        let mut hasher = DefaultHasher::new();
        path_str.hash(&mut hasher);
        let path_hash = hasher.finish();

        Self {
            uuid: path_hash,
            path_hash,
        }
    }

    pub fn null() -> Self {
        Self {
            uuid: 0,
            path_hash: 0,
        }
    }

    pub fn is_null(&self) -> bool {
        self.uuid == 0 && self.path_hash == 0
    }
}

impl Hash for AssetId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.uuid.hash(state);
    }
}

impl fmt::Debug for AssetId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AssetId {{ uuid: {}, path_hash: {} }}",
            self.uuid, self.path_hash
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let id = AssetId::new(123);
        assert!(!id.is_null());
    }

    #[test]
    fn test_null() {
        let id = AssetId::null();
        assert!(id.is_null());
    }

    #[test]
    fn test_from_path() {
        let path = Path::new("test.png");
        let id1 = AssetId::from_path(path);
        let id2 = AssetId::from_path(path);
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_different_paths() {
        let id1 = AssetId::from_path(Path::new("a.png"));
        let id2 = AssetId::from_path(Path::new("b.png"));
        assert_ne!(id1, id2);
    }
}
