//! Hot update and differential patching
//!
//! Provides differential update capability for incremental asset updates.

use crate::{AssetManifest, BuildResult, DiffResult};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Hot update manager
pub struct HotUpdate;

impl HotUpdate {
    /// Calculate diff between two manifests and generate patch
    pub fn diff(old_manifest: &AssetManifest, new_manifest: &AssetManifest) -> HotUpdatePatch {
        let diff_result = old_manifest.diff(new_manifest);
        let file_changes = Self::convert_diff_to_changes(diff_result);
        let total_size = file_changes.iter().map(|c| c.size()).sum();

        HotUpdatePatch {
            version: new_manifest.version.clone(),
            new_manifest: new_manifest.clone(),
            file_changes,
            size_bytes: total_size,
        }
    }

    /// Apply patch to current directory
    pub fn apply(current_dir: impl AsRef<Path>, patch: &HotUpdatePatch) -> BuildResult<()> {
        let dir = current_dir.as_ref();

        for change in &patch.file_changes {
            match change {
                FileChange::Added {
                    path,
                    size: _,
                    hash: _,
                } => {
                    // In real implementation, would download file
                    // For now, just log the operation
                    let dest = dir.join(path);
                    if let Some(parent) = dest.parent() {
                        if !parent.exists() {
                            fs::create_dir_all(parent)?;
                        }
                    }
                    // Placeholder: create empty file (real impl would download)
                    fs::write(&dest, b"")?;
                }
                FileChange::Modified {
                    path,
                    diff,
                    size: _,
                } => {
                    let dest = dir.join(path);
                    // Apply modification (simplified: write diff content)
                    if !diff.is_empty() {
                        fs::write(&dest, diff)?;
                    }
                }
                FileChange::Removed { path } => {
                    let dest = dir.join(path);
                    if dest.exists() {
                        fs::remove_file(dest)?;
                    }
                }
            }
        }

        // Update manifest
        patch.new_manifest.save(dir.join("assets.manifest"))?;

        Ok(())
    }

    /// Convert DiffResult to FileChange list
    fn convert_diff_to_changes(diff: DiffResult) -> Vec<FileChange> {
        let mut changes = Vec::new();

        for entry in diff.added {
            changes.push(FileChange::Added {
                path: entry.path,
                size: entry.size,
                hash: entry.hash,
            });
        }

        for entry in diff.modified {
            // Simplified: store empty diff (real impl would compute binary diff)
            changes.push(FileChange::Modified {
                path: entry.path,
                diff: Vec::new(),
                size: entry.size,
            });
        }

        for path in diff.removed {
            changes.push(FileChange::Removed { path });
        }

        changes
    }
}

/// Hot update patch containing changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotUpdatePatch {
    /// Target version
    pub version: String,
    /// New manifest
    pub new_manifest: AssetManifest,
    /// File changes
    pub file_changes: Vec<FileChange>,
    /// Total size in bytes
    pub size_bytes: u64,
}

impl HotUpdatePatch {
    /// Create new patch
    pub fn new(
        version: String,
        new_manifest: AssetManifest,
        file_changes: Vec<FileChange>,
    ) -> Self {
        let size_bytes = file_changes.iter().map(|c| c.size()).sum();
        Self {
            version,
            new_manifest,
            file_changes,
            size_bytes,
        }
    }

    /// Get version
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Get new manifest
    pub fn new_manifest(&self) -> &AssetManifest {
        &self.new_manifest
    }

    /// Get changes
    pub fn changes(&self) -> &[FileChange] {
        &self.file_changes
    }

    /// Get total size
    pub fn size_bytes(&self) -> u64 {
        self.size_bytes
    }

    /// Serialize to bytes (JSON)
    pub fn to_bytes(&self) -> BuildResult<Vec<u8>> {
        let json = serde_json::to_vec(self)?;
        Ok(json)
    }

    /// Deserialize from bytes
    pub fn from_bytes(bytes: &[u8]) -> BuildResult<Self> {
        serde_json::from_slice(bytes).map_err(|e| crate::BuildError::parse_error(e.to_string()))
    }

    /// Save to file
    pub fn save(&self, path: impl AsRef<Path>) -> BuildResult<()> {
        let bytes = self.to_bytes()?;
        fs::write(path.as_ref(), bytes)?;
        Ok(())
    }

    /// Load from file
    pub fn load(path: impl AsRef<Path>) -> BuildResult<Self> {
        let bytes = fs::read(path.as_ref())?;
        Self::from_bytes(&bytes)
    }
}

/// File change type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum FileChange {
    /// Added file
    Added {
        path: PathBuf,
        size: u64,
        hash: String,
    },
    /// Modified file
    Modified {
        path: PathBuf,
        diff: Vec<u8>,
        size: u64,
    },
    /// Removed file
    Removed { path: PathBuf },
}

impl FileChange {
    /// Get file path
    pub fn path(&self) -> &Path {
        match self {
            FileChange::Added { path, .. } => path,
            FileChange::Modified { path, .. } => path,
            FileChange::Removed { path } => path,
        }
    }

    /// Get file size (0 for removed)
    pub fn size(&self) -> u64 {
        match self {
            FileChange::Added { size, .. } => *size,
            FileChange::Modified { size, .. } => *size,
            FileChange::Removed { .. } => 0,
        }
    }

    /// Check if added
    pub fn is_added(&self) -> bool {
        matches!(self, FileChange::Added { .. })
    }

    /// Check if modified
    pub fn is_modified(&self) -> bool {
        matches!(self, FileChange::Modified { .. })
    }

    /// Check if removed
    pub fn is_removed(&self) -> bool {
        matches!(self, FileChange::Removed { .. })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AssetEntry, AssetKind};
    use tempfile::tempdir;

    fn create_test_manifest(version: &str, entries: Vec<(String, String, u64)>) -> AssetManifest {
        let mut manifest = AssetManifest::new();
        manifest.version = version.to_string();
        for (path, hash, size) in entries {
            manifest.add(AssetEntry {
                path: PathBuf::from(path),
                hash,
                size,
                kind: AssetKind::Texture,
                dependencies: Vec::new(),
            });
        }
        manifest
    }

    #[test]
    fn test_hot_update_diff_added() {
        let old = create_test_manifest("1.0.0", vec![]);
        let new = create_test_manifest("1.1.0", vec![("a.png".into(), "hash_a".into(), 100)]);
        let patch = HotUpdate::diff(&old, &new);
        assert_eq!(patch.version, "1.1.0");
        assert_eq!(patch.file_changes.len(), 1);
        assert!(patch.file_changes[0].is_added());
    }

    #[test]
    fn test_hot_update_diff_modified() {
        let old = create_test_manifest("1.0.0", vec![("a.png".into(), "hash1".into(), 100)]);
        let new = create_test_manifest("1.1.0", vec![("a.png".into(), "hash2".into(), 150)]);
        let patch = HotUpdate::diff(&old, &new);
        assert_eq!(patch.file_changes.len(), 1);
        assert!(patch.file_changes[0].is_modified());
    }

    #[test]
    fn test_hot_update_diff_removed() {
        let old = create_test_manifest("1.0.0", vec![("a.png".into(), "hash1".into(), 100)]);
        let new = create_test_manifest("1.1.0", vec![]);
        let patch = HotUpdate::diff(&old, &new);
        assert_eq!(patch.file_changes.len(), 1);
        assert!(patch.file_changes[0].is_removed());
    }

    #[test]
    fn test_hot_update_patch_serialization() {
        let manifest = create_test_manifest("1.1.0", vec![("a.png".into(), "hash".into(), 100)]);
        let patch = HotUpdatePatch::new(
            "1.1.0".to_string(),
            manifest,
            vec![FileChange::Added {
                path: PathBuf::from("a.png"),
                size: 100,
                hash: "hash".to_string(),
            }],
        );
        let bytes = patch.to_bytes().unwrap();
        let parsed = HotUpdatePatch::from_bytes(&bytes).unwrap();
        assert_eq!(patch.version, parsed.version);
    }

    #[test]
    fn test_file_change_path() {
        let change = FileChange::Added {
            path: PathBuf::from("test.png"),
            size: 100,
            hash: "abc".to_string(),
        };
        assert_eq!(change.path(), Path::new("test.png"));
        assert_eq!(change.size(), 100);
        assert!(change.is_added());
        assert!(!change.is_modified());
        assert!(!change.is_removed());
    }

    #[test]
    fn test_hot_update_apply() {
        let dir = tempdir().unwrap();
        let new_manifest = create_test_manifest("1.1.0", vec![]);
        let patch = HotUpdatePatch::new(
            "1.1.0".to_string(),
            new_manifest,
            vec![FileChange::Added {
                path: PathBuf::from("new.png"),
                size: 0,
                hash: "".to_string(),
            }],
        );
        HotUpdate::apply(dir.path(), &patch).unwrap();
        assert!(dir.path().join("assets.manifest").exists());
    }
}
