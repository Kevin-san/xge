//! Asset pipeline and manifest
//!
//! Provides asset scanning, processing, and manifest management.

use crate::{BuildCache, BuildResult, Hash};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Asset pipeline for processing resources
pub struct AssetPipeline {
    asset_dir: PathBuf,
    entries: Vec<AssetEntry>,
    #[allow(dead_code)]
    cache: BuildCache,
}

impl AssetPipeline {
    /// Create new asset pipeline
    pub fn new(asset_dir: impl AsRef<Path>) -> Self {
        let asset_dir = asset_dir.as_ref().to_path_buf();
        let cache = BuildCache::new(asset_dir.join(".cache")).unwrap_or_else(|_| BuildCache::new_default());
        Self {
            asset_dir,
            entries: Vec::new(),
            cache,
        }
    }

    /// Scan asset directory for resources
    pub fn scan(&mut self) -> BuildResult<()> {
        if !self.asset_dir.exists() {
            return Ok(());
        }

        for entry in WalkDir::new(&self.asset_dir) {
            let entry = entry?;
            if entry.file_type().is_file() {
                let rel_path = entry.path().strip_prefix(&self.asset_dir)?;
                let path = rel_path.to_path_buf();
                let kind = AssetKind::from_extension(path.extension());
                let size = entry.metadata()?.len();
                let hash = Hash::hash_file(entry.path())?;

                self.entries.push(AssetEntry {
                    path,
                    hash,
                    size,
                    kind,
                    dependencies: Vec::new(),
                });
            }
        }
        Ok(())
    }

    /// Import all assets
    pub fn import_all(&mut self) -> BuildResult<()> {
        // Basic import - just validates entries exist
        for entry in &mut self.entries {
            let src_path = self.asset_dir.join(&entry.path);
            if src_path.exists() {
                // Update hash if needed
                entry.hash = Hash::hash_file(&src_path)?;
            }
        }
        Ok(())
    }

    /// Import only changed assets (incremental)
    pub fn reimport_changed(&mut self) -> BuildResult<()> {
        for entry in &mut self.entries {
            let src_path = self.asset_dir.join(&entry.path);
            if src_path.exists() {
                let current_hash = Hash::hash_file(&src_path)?;
                if current_hash != entry.hash {
                    entry.hash = current_hash;
                }
            }
        }
        Ok(())
    }

    /// Process all assets (compress, encrypt, etc.)
    pub fn process_all(&mut self) -> BuildResult<()> {
        // Basic processing placeholder
        // TextureProcessor and AudioProcessor would be called here
        Ok(())
    }

    /// Alias for process_all
    pub fn process(&mut self) -> BuildResult<()> {
        self.process_all()
    }

    /// Alias for import_all
    pub fn import(&mut self) -> BuildResult<()> {
        self.import_all()
    }

    /// Package assets to output directory
    pub fn package(&mut self, out_dir: impl AsRef<Path>) -> BuildResult<PathBuf> {
        let out_path = out_dir.as_ref();
        if !out_path.exists() {
            fs::create_dir_all(out_path)?;
        }

        for entry in &self.entries {
            let src_path = self.asset_dir.join(&entry.path);
            let dest_path = out_path.join(&entry.path);
            if src_path.exists() {
                if let Some(parent) = dest_path.parent() {
                    if !parent.exists() {
                        fs::create_dir_all(parent)?;
                    }
                }
                fs::copy(&src_path, &dest_path)?;
            }
        }

        // Save manifest
        let manifest = self.build_manifest();
        manifest.save(out_path.join("assets.manifest"))?;

        Ok(out_path.to_path_buf())
    }

    /// Build asset manifest
    pub fn build_manifest(&self) -> AssetManifest {
        AssetManifest {
            version: "1.0.0".to_string(),
            entries: self.entries.clone(),
        }
    }

    /// Compute incremental hash for all assets
    pub fn incremental_hash(&self) -> String {
        let combined = self
            .entries
            .iter()
            .map(|e| e.hash.clone())
            .collect::<Vec<_>>()
            .join(",");
        Hash::sha256(combined.as_bytes())
    }

    /// Calculate diff between two manifests
    pub fn diff(
        &self,
        from_manifest: &AssetManifest,
        to_manifest: &AssetManifest,
    ) -> DiffResult {
        from_manifest.diff(to_manifest)
    }

    /// Get files changed since a timestamp
    pub fn changed_files(&self, since: std::time::SystemTime) -> Vec<PathBuf> {
        self.entries
            .iter()
            .filter_map(|e| {
                let path = self.asset_dir.join(&e.path);
                if let Ok(meta) = fs::metadata(&path) {
                    if let Ok(modified) = meta.modified() {
                        if modified > since {
                            return Some(e.path.clone());
                        }
                    }
                }
                None
            })
            .collect()
    }

    /// Incremental build since timestamp
    pub fn incremental_build(
        &mut self,
        since: std::time::SystemTime,
    ) -> BuildResult<AssetManifest> {
        let changed = self.changed_files(since);
        for path in &changed {
            if let Some(entry) = self.entries.iter_mut().find(|e| e.path == *path) {
                let src_path = self.asset_dir.join(&entry.path);
                if src_path.exists() {
                    entry.hash = Hash::hash_file(&src_path)?;
                    entry.size = fs::metadata(&src_path)?.len();
                }
            }
        }
        Ok(self.build_manifest())
    }
}

/// Asset manifest containing all asset entries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetManifest {
    /// Manifest version
    pub version: String,
    /// Asset entries
    pub entries: Vec<AssetEntry>,
}

impl Default for AssetManifest {
    fn default() -> Self {
        Self {
            version: "1.0.0".to_string(),
            entries: Vec::new(),
        }
    }
}

impl AssetManifest {
    /// Create new manifest
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an entry
    pub fn add(&mut self, entry: AssetEntry) {
        self.entries.push(entry);
    }

    /// Get entries
    pub fn entries(&self) -> &[AssetEntry] {
        &self.entries
    }

    /// Serialize to JSON
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }

    /// Deserialize from JSON
    pub fn from_json(json: &str) -> BuildResult<Self> {
        serde_json::from_str(json).map_err(|e| crate::BuildError::parse_error(e.to_string()))
    }

    /// Save to file
    pub fn save(&self, path: impl AsRef<Path>) -> BuildResult<()> {
        let json = self.to_json();
        fs::write(path.as_ref(), json)?;
        Ok(())
    }

    /// Load from file
    pub fn load(path: impl AsRef<Path>) -> BuildResult<Self> {
        let content = fs::read_to_string(path.as_ref())?;
        Self::from_json(&content)
    }

    /// Calculate diff with another manifest
    pub fn diff(&self, other: &AssetManifest) -> DiffResult {
        let mut added = Vec::new();
        let mut modified = Vec::new();
        let mut removed = Vec::new();

        // Find entries in self but not in other (removed)
        for entry in &self.entries {
            if !other.entries.iter().any(|e| e.path == entry.path) {
                removed.push(entry.path.clone());
            }
        }

        // Find entries in other but not in self (added) or with different hash (modified)
        for entry in &other.entries {
            let self_entry = self.entries.iter().find(|e| e.path == entry.path);
            match self_entry {
                None => added.push(entry.clone()),
                Some(old) if old.hash != entry.hash => modified.push(entry.clone()),
                _ => {}
            }
        }

        DiffResult { added, modified, removed }
    }
}

/// Asset entry information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetEntry {
    /// Relative path
    pub path: PathBuf,
    /// SHA256 hash
    pub hash: String,
    /// Size in bytes
    pub size: u64,
    /// Asset type
    pub kind: AssetKind,
    /// Dependencies
    pub dependencies: Vec<PathBuf>,
}

impl AssetEntry {
    /// Get path
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Get hash
    pub fn hash(&self) -> &str {
        &self.hash
    }

    /// Get size
    pub fn size(&self) -> u64 {
        self.size
    }

    /// Get kind
    pub fn kind(&self) -> AssetKind {
        self.kind.clone()
    }
}

/// Asset type enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AssetKind {
    Texture,
    Audio,
    Model,
    Scene,
    Prefab,
    Font,
    Custom(String),
}

impl AssetKind {
    /// Get kind as string
    pub fn as_str(&self) -> &str {
        match self {
            AssetKind::Texture => "texture",
            AssetKind::Audio => "audio",
            AssetKind::Model => "model",
            AssetKind::Scene => "scene",
            AssetKind::Prefab => "prefab",
            AssetKind::Font => "font",
            AssetKind::Custom(s) => s,
        }
    }

    /// Determine kind from file extension
    pub fn from_extension(ext: Option<&std::ffi::OsStr>) -> Self {
        let ext_str = ext.and_then(|e| e.to_str()).map(|s| s.to_lowercase());
        match ext_str.as_deref() {
            Some("png") | Some("jpg") | Some("jpeg") | Some("tga") | Some("bmp") | Some("gif") => AssetKind::Texture,
            Some("wav") | Some("ogg") | Some("mp3") | Some("flac") | Some("aac") => AssetKind::Audio,
            Some("glb") | Some("gltf") | Some("obj") | Some("fbx") => AssetKind::Model,
            Some("scene") => AssetKind::Scene,
            Some("prefab") => AssetKind::Prefab,
            Some("ttf") | Some("otf") | Some("fnt") => AssetKind::Font,
            Some(s) => AssetKind::Custom(s.to_string()),
            None => AssetKind::Custom("unknown".to_string()),
        }
    }
}

/// Compression algorithm type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AssetCompress {
    #[default]
    None,
    Zstd,
    Gzip,
    Brotli,
    LZ4,
}

/// Encryption algorithm type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AssetEncrypt {
    #[default]
    None,
    AesGcm128,
    AesGcm256,
    XorChaCha20,
}

/// Diff result between two manifests
#[derive(Debug, Clone)]
pub struct DiffResult {
    /// Added entries
    pub added: Vec<AssetEntry>,
    /// Modified entries
    pub modified: Vec<AssetEntry>,
    /// Removed paths
    pub removed: Vec<PathBuf>,
}

impl DiffResult {
    /// Check if diff is empty
    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.modified.is_empty() && self.removed.is_empty()
    }

    /// Get total change count
    pub fn total_changes(&self) -> usize {
        self.added.len() + self.modified.len() + self.removed.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_asset_kind_from_extension() {
        assert_eq!(AssetKind::from_extension(Some("png".as_ref())), AssetKind::Texture);
        assert_eq!(AssetKind::from_extension(Some("wav".as_ref())), AssetKind::Audio);
        assert_eq!(AssetKind::from_extension(Some("glb".as_ref())), AssetKind::Model);
    }

    #[test]
    fn test_asset_manifest_new() {
        let manifest = AssetManifest::new();
        assert_eq!(manifest.version, "1.0.0");
        assert!(manifest.entries.is_empty());
    }

    #[test]
    fn test_asset_manifest_json_roundtrip() {
        let mut manifest = AssetManifest::new();
        manifest.add(AssetEntry {
            path: PathBuf::from("test.png"),
            hash: "abc123".to_string(),
            size: 1024,
            kind: AssetKind::Texture,
            dependencies: Vec::new(),
        });
        let json = manifest.to_json();
        let parsed = AssetManifest::from_json(&json).unwrap();
        assert_eq!(manifest.entries.len(), parsed.entries.len());
        assert_eq!(manifest.entries[0].path, parsed.entries[0].path);
    }

    #[test]
    fn test_diff_result_empty() {
        let diff = DiffResult {
            added: Vec::new(),
            modified: Vec::new(),
            removed: Vec::new(),
        };
        assert!(diff.is_empty());
        assert_eq!(diff.total_changes(), 0);
    }

    #[test]
    fn test_asset_manifest_diff() {
        let mut manifest1 = AssetManifest::new();
        manifest1.add(AssetEntry {
            path: PathBuf::from("a.png"),
            hash: "hash1".to_string(),
            size: 100,
            kind: AssetKind::Texture,
            dependencies: Vec::new(),
        });
        manifest1.add(AssetEntry {
            path: PathBuf::from("b.png"),
            hash: "hash2".to_string(),
            size: 200,
            kind: AssetKind::Texture,
            dependencies: Vec::new(),
        });

        let mut manifest2 = AssetManifest::new();
        manifest2.add(AssetEntry {
            path: PathBuf::from("a.png"),
            hash: "hash1_modified".to_string(),
            size: 150,
            kind: AssetKind::Texture,
            dependencies: Vec::new(),
        });
        manifest2.add(AssetEntry {
            path: PathBuf::from("c.png"),
            hash: "hash3".to_string(),
            size: 300,
            kind: AssetKind::Texture,
            dependencies: Vec::new(),
        });

        let diff = manifest1.diff(&manifest2);
        assert_eq!(diff.added.len(), 1);
        assert_eq!(diff.modified.len(), 1);
        assert_eq!(diff.removed.len(), 1);
    }

    #[test]
    fn test_asset_pipeline_scan() {
        let dir = tempdir().unwrap();
        // Create test files
        fs::create_dir_all(dir.path().join("textures")).unwrap();
        fs::write(dir.path().join("textures/test.png"), b"fake_png").unwrap();

        let mut pipeline = AssetPipeline::new(dir.path());
        pipeline.scan().unwrap();
        assert!(!pipeline.entries.is_empty());
    }
}