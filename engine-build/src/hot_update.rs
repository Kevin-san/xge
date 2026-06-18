//! Hot update and differential patching
//!
//! Provides differential update capability for incremental asset updates.
//! Includes Ed25519 signature verification to ensure patch authenticity.

use crate::{AssetManifest, BuildResult, DiffResult};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Component, Path, PathBuf};

/// 安全地连接根目录和相对路径，防止路径穿越攻击。
fn safe_join(root: &Path, rel: &Path) -> BuildResult<PathBuf> {
    if rel.is_absolute() {
        return Err(crate::BuildError::Path(
            "Absolute paths are not allowed".to_string(),
        ));
    }
    if rel.components().any(|c| matches!(c, Component::ParentDir)) {
        return Err(crate::BuildError::Path(
            "Parent directory references (..) are not allowed".to_string(),
        ));
    }
    // 规范化根目录（要求 root 必须存在于文件系统中）。
    let canon_root = std::fs::canonicalize(root).map_err(|e| {
        crate::BuildError::Path(format!(
            "Failed to canonicalize root directory {}: {}",
            root.display(),
            e
        ))
    })?;

    // 在 root 下拼接 rel。注意 rel 不得包含 ..，否则上面已拒绝。
    let dest = canon_root.join(rel);

    // 使用逻辑规范化去除可能的 "." 组件（但不允许 ".."，已在上方过滤）。
    // 之后对 dest 的父目录做 canonicalize 以防止 symlink 跳转到根目录之外。
    // 如果文件不存在但父目录存在于 canon_root 之下，也视为合法目标。
    let verified = if dest.exists() {
        let canon = std::fs::canonicalize(&dest).map_err(|e| {
            crate::BuildError::Path(format!(
                "Failed to canonicalize path {}: {}",
                dest.display(),
                e
            ))
        })?;
        canon
    } else {
        // 目标文件尚不存在：校验父目录位于 canon_root 之内。
        // 同时禁止直接以根目录作为目标，防止写入根目录之外的默认文件。
        match dest.parent() {
            Some(parent) if parent != Path::new("") => {
                let parent_canon = std::fs::canonicalize(parent)
                    .or_else(|_| std::fs::canonicalize(&canon_root))?;
                if !parent_canon.starts_with(&canon_root) {
                    return Err(crate::BuildError::Path(format!(
                        "Path {} would escape root directory",
                        dest.display()
                    )));
                }
                dest.clone()
            }
            _ => dest.clone(),
        }
    };

    // 最终确保规范化后的路径落在根目录范围内。
    if !verified.starts_with(&canon_root) {
        return Err(crate::BuildError::Path(format!(
            "Path {} would escape root directory",
            dest.display()
        )));
    }
    Ok(dest)
}

/// Encode bytes to base64.
fn base64_encode(data: impl AsRef<[u8]>) -> String {
    BASE64.encode(data)
}

/// Decode base64 string to bytes.
fn base64_decode(data: &str) -> Result<Vec<u8>, base64::DecodeError> {
    BASE64.decode(data)
}

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
            signature: None,
        }
    }

    /// Apply patch to current directory.
    /// If `public_key` is provided, the patch signature is verified before applying.
    pub fn apply(
        current_dir: impl AsRef<Path>,
        patch: &HotUpdatePatch,
        public_key: Option<&VerifyingKey>,
    ) -> BuildResult<()> {
        // Verify signature if a public key is provided
        if let Some(key) = public_key {
            if let Some(result) = patch.verify_opt(key) {
                result?;
            }
        }

        let dir = current_dir.as_ref();

        for change in &patch.file_changes {
            match change {
                FileChange::Added {
                    path,
                    size: _,
                    hash: _,
                } => {
                    // 使用安全路径连接防止路径穿越
                    let dest = safe_join(dir, path)?;
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
                    // 使用安全路径连接防止路径穿越
                    let dest = safe_join(dir, path)?;
                    // Apply modification (simplified: write diff content)
                    if !diff.is_empty() {
                        fs::write(&dest, diff)?;
                    }
                }
                FileChange::Removed { path } => {
                    // 使用安全路径连接防止路径穿越
                    let dest = safe_join(dir, path)?;
                    if dest.exists() {
                        fs::remove_file(dest)?;
                    }
                }
            }
        }

        // Update manifest (安全路径，固定文件名)
        let manifest_path = safe_join(dir, Path::new("assets.manifest"))?;
        patch.new_manifest.save(manifest_path)?;

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
    /// Ed25519 signature over the canonical patch payload (base64 encoded)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
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
            signature: None,
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

    /// Check if patch is signed
    pub fn is_signed(&self) -> bool {
        self.signature.is_some()
    }

    /// Get signature (base64 encoded)
    pub fn signature(&self) -> Option<&str> {
        self.signature.as_deref()
    }

    /// Get the canonical signing payload for this patch.
    /// Excludes the signature field itself to avoid self-referential signatures.
    fn signing_payload(&self) -> BuildResult<Vec<u8>> {
        #[derive(Serialize)]
        struct SignablePatch<'a> {
            version: &'a str,
            new_manifest: &'a AssetManifest,
            file_changes: &'a [FileChange],
            size_bytes: u64,
        }
        let payload = SignablePatch {
            version: &self.version,
            new_manifest: &self.new_manifest,
            file_changes: &self.file_changes,
            size_bytes: self.size_bytes,
        };
        serde_json::to_vec(&payload).map_err(|e| crate::BuildError::parse_error(e.to_string()))
    }

    /// Sign this patch with an Ed25519 signing key.
    /// Stores the signature (base64 encoded) in the patch.
    pub fn sign(&mut self, signing_key: &SigningKey) -> BuildResult<()> {
        let payload = self.signing_payload()?;
        let sig = signing_key.sign(&payload);
        self.signature = Some(base64_encode(sig.to_bytes()));
        Ok(())
    }

    /// Verify the patch signature against a public key.
    /// Returns Ok(()) if signature is valid, Err if missing or invalid.
    pub fn verify(&self, public_key: &VerifyingKey) -> BuildResult<()> {
        let sig_str = self.signature.as_ref()
            .ok_or_else(|| crate::BuildError::signature_error("Patch has no signature".to_string()))?;

        let sig_bytes = base64_decode(sig_str)
            .map_err(|e| crate::BuildError::signature_error(format!("Invalid base64: {}", e)))?;

        let sig = Signature::from_slice(&sig_bytes)
            .map_err(|_| crate::BuildError::signature_error("Signature has wrong length".to_string()))?;

        let payload = self.signing_payload()?;
        public_key
            .verify(&payload, &sig)
            .map_err(|_| crate::BuildError::signature_error("Signature verification failed".to_string()))
    }

    /// Verify the patch signature, returning None if unsigned, Some(Ok) if valid, Some(Err) if invalid.
    pub fn verify_opt(&self, public_key: &VerifyingKey) -> Option<BuildResult<()>> {
        if let Some(sig_str) = &self.signature {
            let result = (|| {
                let sig_bytes = base64_decode(sig_str)
                    .map_err(|e| crate::BuildError::signature_error(format!("Invalid base64: {}", e)))?;
                let sig = Signature::from_slice(&sig_bytes)
                    .map_err(|_| crate::BuildError::signature_error("Signature has wrong length".to_string()))?;
                let payload = self.signing_payload()?;
                public_key
                    .verify(&payload, &sig)
                    .map_err(|_| crate::BuildError::signature_error("Signature verification failed".to_string()))
            })();
            Some(result)
        } else {
            None
        }
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

/// Generate a new Ed25519 key pair for patch signing.
#[allow(dead_code)]
pub fn generate_signing_keypair() -> (SigningKey, VerifyingKey) {
    let signing_key = SigningKey::generate(&mut OsRng);
    let verifying_key = signing_key.verifying_key();
    (signing_key, verifying_key)
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
        // Apply without signature verification (no key provided)
        HotUpdate::apply(dir.path(), &patch, None).unwrap();
        assert!(dir.path().join("assets.manifest").exists());
    }

    #[test]
    fn test_patch_sign_and_verify() {
        let (signing_key, verifying_key) = generate_signing_keypair();

        let mut patch = HotUpdatePatch::new(
            "2.0.0".to_string(),
            create_test_manifest("2.0.0", vec![]),
            vec![FileChange::Added {
                path: PathBuf::from("new.png"),
                size: 100,
                hash: "abc".to_string(),
            }],
        );
        assert!(!patch.is_signed());

        // Sign the patch
        patch.sign(&signing_key).unwrap();
        assert!(patch.is_signed());
        assert!(patch.signature().is_some());

        // Verify with correct key
        patch.verify(&verifying_key).unwrap();

        // Verify fails with wrong key
        let (_, wrong_key) = generate_signing_keypair();
        assert!(patch.verify(&wrong_key).is_err());
    }

    #[test]
    fn test_patch_verify_unsigned() {
        let (_, verifying_key) = generate_signing_keypair();
        let patch = HotUpdatePatch::new(
            "1.0.0".to_string(),
            create_test_manifest("1.0.0", vec![]),
            vec![],
        );

        // verify_opt returns None for unsigned patch
        assert!(patch.verify_opt(&verifying_key).is_none());

        // verify returns Err for unsigned patch
        assert!(patch.verify(&verifying_key).is_err());
    }

    #[test]
    fn test_patch_apply_requires_valid_signature() {
        let (signing_key, verifying_key) = generate_signing_keypair();
        let dir = tempdir().unwrap();

        let mut patch = HotUpdatePatch::new(
            "2.0.0".to_string(),
            create_test_manifest("2.0.0", vec![]),
            vec![FileChange::Added {
                path: PathBuf::from("signed_file.txt"),
                size: 10,
                hash: "xyz".to_string(),
            }],
        );

        // Unsigned patch can be applied with no key
        HotUpdate::apply(dir.path(), &patch, None).unwrap();

        // Sign patch
        patch.sign(&signing_key).unwrap();

        // Apply with correct key succeeds
        let dir2 = tempdir().unwrap();
        HotUpdate::apply(dir2.path(), &patch, Some(&verifying_key)).unwrap();

        // Apply with wrong key fails
        let (_, wrong_key) = generate_signing_keypair();
        let dir3 = tempdir().unwrap();
        assert!(HotUpdate::apply(dir3.path(), &patch, Some(&wrong_key)).is_err());
    }

    // 路径安全：测试 safe_join 对相对路径、绝对路径和 ".." 的处理。
    // 使用辅助函数通过 HotUpdate::apply 的公开路径间接验证。
    #[test]
    fn test_safe_join_absolute_path_rejected() {
        let dir = tempdir().unwrap();
        let patch = HotUpdatePatch::new(
            "1.0.0".to_string(),
            create_test_manifest("1.0.0", vec![]),
            vec![FileChange::Added {
                path: PathBuf::from("/etc/evil.toml"),
                size: 0,
                hash: String::new(),
            }],
        );
        assert!(HotUpdate::apply(dir.path(), &patch, None).is_err());
    }

    #[test]
    fn test_safe_join_parent_dir_rejected() {
        let dir = tempdir().unwrap();
        let patch = HotUpdatePatch::new(
            "1.0.0".to_string(),
            create_test_manifest("1.0.0", vec![]),
            vec![FileChange::Added {
                path: PathBuf::from("../outside.txt"),
                size: 0,
                hash: String::new(),
            }],
        );
        assert!(HotUpdate::apply(dir.path(), &patch, None).is_err());
    }

    #[test]
    fn test_safe_join_new_file_in_subdir() {
        // 验证在不存在的子目录中创建文件：safe_join 应对父目录进行规范化，
        // 且 HotUpdate::apply 应创建目标子目录与文件。
        let dir = tempdir().unwrap();
        let sub = PathBuf::from("assets/sub/new.png");
        let patch = HotUpdatePatch::new(
            "1.0.0".to_string(),
            create_test_manifest("1.0.0", vec![]),
            vec![FileChange::Added {
                path: sub.clone(),
                size: 0,
                hash: String::new(),
            }],
        );
        HotUpdate::apply(dir.path(), &patch, None).unwrap();
        assert!(dir.path().join(&sub).exists());
        // 同时确保没有文件"漏出"到临时目录之外。
        assert!(!dir.path().parent().unwrap().join("new.png").exists());
    }
}
