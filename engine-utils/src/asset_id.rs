use uuid::Uuid;
use std::path::Path;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct AssetId {
    uuid: Uuid,
    path_hash: u64,
}

impl AssetId {
    pub fn new(uuid: Uuid) -> Self {
        Self {
            uuid,
            path_hash: 0,
        }
    }

    pub fn from_path(path: &Path) -> Self {
        let mut hasher = DefaultHasher::new();
        path.hash(&mut hasher);
        let path_hash = hasher.finish();
        
        let mut bytes = [0u8; 16];
        bytes[0..8].copy_from_slice(&path_hash.to_le_bytes());
        
        Self {
            uuid: Uuid::from_bytes(bytes),
            path_hash,
        }
    }

    pub fn null() -> Self {
        Self {
            uuid: Uuid::nil(),
            path_hash: 0,
        }
    }

    pub fn is_null(&self) -> bool {
        self.uuid.is_nil()
    }

    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    pub fn path_hash(&self) -> u64 {
        self.path_hash
    }
}

impl PartialEq for AssetId {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

impl Eq for AssetId {}

impl Hash for AssetId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.uuid.hash(state);
    }
}

impl std::fmt::Debug for AssetId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AssetId({})", self.uuid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn assetid_new() {
        let uuid = Uuid::new_v4();
        let id = AssetId::new(uuid);
        assert_eq!(id.uuid(), &uuid);
    }

    #[test]
    fn assetid_from_path() {
        let id = AssetId::from_path(Path::new("test/path.txt"));
        assert!(!id.is_null());
    }

    #[test]
    fn assetid_null() {
        let id = AssetId::null();
        assert!(id.is_null());
    }

    #[test]
    fn assetid_from_path_different() {
        let id1 = AssetId::from_path(Path::new("a.txt"));
        let id2 = AssetId::from_path(Path::new("b.txt"));
        assert_ne!(id1, id2);
    }

    #[test]
    fn assetid_from_path_same() {
        let id1 = AssetId::from_path(Path::new("same/path.txt"));
        let id2 = AssetId::from_path(Path::new("same/path.txt"));
        assert_eq!(id1, id2);
    }
}
