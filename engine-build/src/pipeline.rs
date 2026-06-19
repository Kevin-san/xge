//! Build pipeline and artifacts
//!
//! Provides the main build pipeline and artifact management.

use crate::{
    asset::AssetManifest, BuildCache, BuildConfig, BuildError, BuildLogger, BuildReport,
    BuildResult, PlatformTarget,
};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Instant;
use walkdir::WalkDir;

/// Build pipeline for coordinating the build process
pub struct BuildPipeline {
    config: BuildConfig,
    #[allow(dead_code)]
    logger: BuildLogger,
    #[allow(dead_code)]
    cache: BuildCache,
}

impl BuildPipeline {
    /// Create new build pipeline with config
    pub fn new(config: BuildConfig) -> BuildResult<Self> {
        let logger = BuildLogger::new(false);
        let cache = BuildCache::new(config.temp_dir.join("cache"))?;
        Ok(Self {
            config,
            logger,
            cache,
        })
    }

    /// Get build config
    pub fn config(&self) -> &BuildConfig {
        &self.config
    }

    /// Get platform target
    pub fn platform_target(&self) -> PlatformTarget {
        self.config.platform_target
    }

    /// Get profile
    pub fn profile(&self) -> crate::Profile {
        self.config.profile
    }

    /// Execute full build process
    pub fn build(&self) -> BuildResult<BuildArtifact> {
        let start = Instant::now();
        let mut report = BuildReport::new();

        self.logger.info(&format!(
            "Starting build for {} ({})",
            self.config.app_name,
            self.config.platform_target.target_triple()
        ));

        // Stage 1: Initialize
        self.logger.progress(10, "Initializing build...");
        self.init_build()?;
        report.add_stage("Init", start.elapsed(), 0);

        // Stage 2: Process assets
        let asset_start = Instant::now();
        self.logger.progress(30, "Processing assets...");
        let manifest = self.process_assets()?;
        let asset_size = manifest.entries().iter().map(|e| e.size).sum();
        report.add_stage("ProcessAssets", asset_start.elapsed(), asset_size);

        // Stage 3: Package
        let pkg_start = Instant::now();
        self.logger.progress(60, "Creating package...");
        let artifact = self.package(&manifest)?;
        report.add_stage("Package", pkg_start.elapsed(), artifact.size);

        // Stage 4: Done
        self.logger.progress(100, "Build complete!");
        report.add_stage("Done", start.elapsed(), artifact.size);

        self.logger.info(&format!(
            "Build completed in {:.2}s, output: {}",
            report.total_duration().as_secs_f64(),
            artifact.path.display()
        ));

        Ok(artifact)
    }

    /// Clean build artifacts
    pub fn clean(&self) -> BuildResult<()> {
        self.logger.info("Cleaning build artifacts...");
        if self.config.output_dir.exists() {
            fs::remove_dir_all(&self.config.output_dir)?;
        }
        if self.config.temp_dir.exists() {
            fs::remove_dir_all(&self.config.temp_dir)?;
        }
        self.logger.info("Clean complete");
        Ok(())
    }

    /// Build and run (only for native targets)
    pub fn run(&self) -> BuildResult<()> {
        let artifact = self.build()?;
        if !self.config.platform_target.supported() {
            return Err(BuildError::unsupported_platform(
                self.config.platform_target,
            ));
        }

        // Execute the binary (native targets only)
        match self.config.platform_target {
            PlatformTarget::Windows | PlatformTarget::MacOS | PlatformTarget::Linux => {
                let executable = artifact.path.join(&self.config.app_name);
                #[cfg(target_family = "unix")]
                {
                    std::process::Command::new(&executable)
                        .current_dir(&artifact.path)
                        .spawn()?;
                }
                #[cfg(target_family = "windows")]
                {
                    std::process::Command::new(format!("{}.exe", executable.display()))
                        .current_dir(&artifact.path)
                        .spawn()?;
                }
            }
            _ => {
                self.logger
                    .warn("Run is only supported for native platforms");
            }
        }
        Ok(())
    }

    /// Initialize build directories
    fn init_build(&self) -> BuildResult<()> {
        // Create output directory
        if !self.config.output_dir.exists() {
            fs::create_dir_all(&self.config.output_dir)?;
        }
        // Create temp directory
        if !self.config.temp_dir.exists() {
            fs::create_dir_all(&self.config.temp_dir)?;
        }
        Ok(())
    }

    /// Process assets
    fn process_assets(&self) -> BuildResult<AssetManifest> {
        use crate::asset::AssetPipeline;
        let mut pipeline = AssetPipeline::new(&self.config.assets_dir);
        pipeline.scan()?;
        pipeline.import_all()?;
        pipeline.process_all()?;
        Ok(pipeline.build_manifest())
    }

    /// Package build artifacts
    fn package(&self, manifest: &AssetManifest) -> BuildResult<BuildArtifact> {
        let mut pkg = Package::new(&self.config.output_dir)?;
        pkg.add_manifest(manifest.clone());

        // Add asset files
        for entry in manifest.entries() {
            let src_path = self.config.assets_dir.join(&entry.path);
            if src_path.exists() {
                pkg.add_file(&entry.path, fs::read(&src_path)?);
            }
        }

        pkg.build(PackageFormat::Dir)
    }
}

/// Build artifact information
#[derive(Debug, Clone)]
pub struct BuildArtifact {
    /// Path to the artifact
    pub path: PathBuf,
    /// Size in bytes
    pub size: u64,
    /// Target platform
    pub platform: PlatformTarget,
    /// Version string
    pub version: String,
    /// Signing information (optional)
    pub sign_info: Option<SignInfo>,
}

impl BuildArtifact {
    /// Get artifact path
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Get artifact size
    pub fn size(&self) -> u64 {
        self.size
    }

    /// Get target platform
    pub fn platform(&self) -> PlatformTarget {
        self.platform
    }

    /// Get version string
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Get sign info
    pub fn sign_info(&self) -> Option<&SignInfo> {
        self.sign_info.as_ref()
    }
}

/// Signing information
#[derive(Debug, Clone)]
pub struct SignInfo {
    /// Signature bytes
    pub signature: Vec<u8>,
    /// Certificate bytes
    pub certificate: Vec<u8>,
    /// Timestamp
    pub timestamp: Option<String>,
}

/// Package format type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PackageFormat {
    /// Directory format
    Dir,
    /// ZIP archive
    Zip,
    /// Android APK
    Apk,
    /// iOS IPA
    Ipa,
    /// WebAssembly
    Wasm,
    /// Mini app package
    MiniApp,
}

/// Package builder
pub struct Package {
    output_dir: PathBuf,
    files: HashMap<PathBuf, Vec<u8>>,
    manifest: Option<AssetManifest>,
}

impl Package {
    /// Create new package
    pub fn new(output_dir: impl AsRef<Path>) -> BuildResult<Self> {
        let dir = output_dir.as_ref();
        if !dir.exists() {
            fs::create_dir_all(dir)?;
        }
        Ok(Self {
            output_dir: dir.to_path_buf(),
            files: HashMap::new(),
            manifest: None,
        })
    }

    /// Add a file to the package
    pub fn add_file(&mut self, pkg_path: impl AsRef<Path>, bytes: impl Into<Vec<u8>>) {
        self.files
            .insert(pkg_path.as_ref().to_path_buf(), bytes.into());
    }

    /// Add a directory to the package
    pub fn add_directory(
        &mut self,
        prefix: impl AsRef<Path>,
        dir: impl AsRef<Path>,
    ) -> BuildResult<()> {
        let prefix = prefix.as_ref();
        for entry in WalkDir::new(dir.as_ref()) {
            let entry = entry?;
            if entry.file_type().is_file() {
                let rel_path = entry.path().strip_prefix(dir.as_ref())?;
                let pkg_path = prefix.join(rel_path);
                self.add_file(pkg_path, fs::read(entry.path())?);
            }
        }
        Ok(())
    }

    /// Add manifest to the package
    pub fn add_manifest(&mut self, manifest: AssetManifest) {
        self.manifest = Some(manifest);
    }

    /// Build the package with specified format
    pub fn build(&self, format: PackageFormat) -> BuildResult<BuildArtifact> {
        match format {
            PackageFormat::Dir => self.build_dir(&self.output_dir),
            PackageFormat::Zip => self.build_zip(self.output_dir.join("package.zip")),
            _ => self.build_dir(&self.output_dir),
        }
    }

    /// Build as directory
    pub fn build_dir(&self, out: impl AsRef<Path>) -> BuildResult<BuildArtifact> {
        let out_dir = out.as_ref();
        if !out_dir.exists() {
            fs::create_dir_all(out_dir)?;
        }

        let mut total_size = 0u64;
        for (path, bytes) in &self.files {
            let file_path = out_dir.join(path);
            if let Some(parent) = file_path.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent)?;
                }
            }
            fs::write(&file_path, bytes)?;
            total_size += bytes.len() as u64;
        }

        // Write manifest
        if let Some(manifest) = &self.manifest {
            manifest.save(out_dir.join("assets.manifest"))?;
            total_size += manifest.to_json().len() as u64;
        }

        Ok(BuildArtifact {
            path: out_dir.to_path_buf(),
            size: total_size,
            platform: PlatformTarget::current(),
            version: "1.0.0".to_string(),
            sign_info: None,
        })
    }

    /// Build as ZIP archive
    pub fn build_zip(&self, out: impl AsRef<Path>) -> BuildResult<BuildArtifact> {
        use std::io::BufWriter;
        let out_path = out.as_ref();

        let file = File::create(out_path)?;
        let mut writer = BufWriter::new(file);

        // Simple ZIP creation (without external zip crate dependency)
        // Write files as uncompressed entries
        let mut total_size = 0u64;
        for (path, bytes) in &self.files {
            // Local file header signature
            writer.write_all(&[0x50, 0x4B, 0x03, 0x04])?;
            // Version needed
            writer.write_all(&[0x0A, 0x00])?;
            // General purpose flag
            writer.write_all(&[0x00, 0x00])?;
            // Compression method (stored)
            writer.write_all(&[0x00, 0x00])?;
            // File modification time/date (placeholder)
            writer.write_all(&[0x00, 0x00, 0x00, 0x00])?;
            // CRC-32 (placeholder)
            writer.write_all(&[0x00, 0x00, 0x00, 0x00])?;
            // Compressed size
            let size_bytes = (bytes.len() as u32).to_le_bytes();
            writer.write_all(&size_bytes)?;
            // Uncompressed size
            writer.write_all(&size_bytes)?;
            // File name length
            let name_bytes = path.to_string_lossy().into_owned();
            let name_len = (name_bytes.len() as u16).to_le_bytes();
            writer.write_all(&name_len)?;
            // Extra field length
            writer.write_all(&[0x00, 0x00])?;
            // File name
            writer.write_all(name_bytes.as_bytes())?;
            // File data
            writer.write_all(bytes)?;
            total_size += bytes.len() as u64;
        }

        writer.flush()?;
        total_size += out_path.metadata()?.len();

        Ok(BuildArtifact {
            path: out_path.to_path_buf(),
            size: total_size,
            platform: PlatformTarget::current(),
            version: "1.0.0".to_string(),
            sign_info: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_package_new() {
        let dir = tempdir().unwrap();
        let pkg = Package::new(dir.path()).unwrap();
        assert!(pkg.output_dir.exists());
    }

    #[test]
    fn test_package_add_file() {
        let dir = tempdir().unwrap();
        let mut pkg = Package::new(dir.path()).unwrap();
        pkg.add_file("test.txt", b"Hello".to_vec());
        assert_eq!(pkg.files.len(), 1);
    }

    #[test]
    fn test_package_add_file_multiple() {
        let dir = tempdir().unwrap();
        let mut pkg = Package::new(dir.path()).unwrap();
        pkg.add_file("a.txt", b"Hello".to_vec());
        pkg.add_file("b.txt", b"World".to_vec());
        pkg.add_file("nested/c.txt", b"Nested".to_vec());
        assert_eq!(pkg.files.len(), 3);
    }

    #[test]
    fn test_package_add_directory() {
        let dir = tempdir().unwrap();
        let src_dir = dir.path().join("src");
        std::fs::create_dir_all(&src_dir).unwrap();
        std::fs::write(src_dir.join("file1.txt"), b"one").unwrap();
        std::fs::write(src_dir.join("file2.txt"), b"two").unwrap();

        let out_dir = dir.path().join("out");
        let mut pkg = Package::new(&out_dir).unwrap();
        pkg.add_directory("prefix", &src_dir).unwrap();
        // 至少两个文件被添加
        assert!(pkg.files.len() >= 2);
    }

    #[test]
    fn test_package_build_dir() {
        let dir = tempdir().unwrap();
        let mut pkg = Package::new(dir.path()).unwrap();
        pkg.add_file("test.txt", b"Hello".to_vec());
        let artifact = pkg.build(PackageFormat::Dir).unwrap();
        assert!(artifact.path.exists());
        assert!(artifact.path.join("test.txt").exists());
    }

    #[test]
    fn test_package_build_dir_nested() {
        let dir = tempdir().unwrap();
        let mut pkg = Package::new(dir.path()).unwrap();
        pkg.add_file("nested/deep/file.txt", b"nested".to_vec());
        let artifact = pkg.build(PackageFormat::Dir).unwrap();
        let written = artifact.path.join("nested").join("deep").join("file.txt");
        assert!(written.exists());
    }

    #[test]
    fn test_package_build_zip() {
        let dir = tempdir().unwrap();
        let mut pkg = Package::new(dir.path()).unwrap();
        pkg.add_file("hello.txt", b"world".to_vec());
        let artifact = pkg.build(PackageFormat::Zip).unwrap();
        assert!(artifact.path.exists());
        // zip 文件内容非零字节
        let meta = std::fs::metadata(&artifact.path).unwrap();
        assert!(meta.len() > 0);
    }

    #[test]
    fn test_artifact_fields() {
        let dir = tempdir().unwrap();
        let mut pkg = Package::new(dir.path()).unwrap();
        pkg.add_file("a.txt", b"aaa".to_vec());
        let artifact = pkg.build(PackageFormat::Dir).unwrap();
        assert!(artifact.size() > 0);
        assert_eq!(artifact.path(), dir.path());
        // 输出为当前平台
        assert_eq!(artifact.platform(), PlatformTarget::current());
        assert_eq!(artifact.version(), "1.0.0");
        assert!(artifact.sign_info().is_none());
    }

    #[test]
    fn test_artifact_debug() {
        let dir = tempdir().unwrap();
        let pkg = Package::new(dir.path()).unwrap();
        let a = pkg.build(PackageFormat::Dir).unwrap();
        let s = format!("{:?}", a);
        assert!(s.contains("BuildArtifact"));
    }

    #[test]
    fn test_artifact_clone() {
        let dir = tempdir().unwrap();
        let pkg = Package::new(dir.path()).unwrap();
        let a = pkg.build(PackageFormat::Dir).unwrap();
        let b = a.clone();
        assert_eq!(a.size(), b.size());
    }

    #[test]
    fn test_package_add_manifest() {
        let dir = tempdir().unwrap();
        let mut pkg = Package::new(dir.path()).unwrap();
        let manifest = AssetManifest::new();
        pkg.add_manifest(manifest);
        let artifact = pkg.build(PackageFormat::Dir).unwrap();
        // 应当写入 assets.manifest 文件
        assert!(artifact.path.join("assets.manifest").exists());
    }

    #[test]
    fn test_package_format_debug() {
        let f = PackageFormat::Dir;
        let s = format!("{:?}", f);
        assert!(s.contains("Dir"));
    }

    #[test]
    fn test_build_pipeline_new() {
        let tmp = tempdir().unwrap();
        let assets_dir = tmp.path().join("assets");
        let output_dir = tmp.path().join("output");
        std::fs::create_dir_all(&assets_dir).unwrap();
        std::fs::write(assets_dir.join("a.txt"), b"aaa").unwrap();

        let config = BuildConfig::new()
            .with_assets_dir(&assets_dir)
            .with_output_dir(&output_dir)
            .with_temp_dir(tmp.path().join("temp"));

        let pipeline = BuildPipeline::new(config).unwrap();
        assert!(!pipeline.config().output_dir().as_os_str().is_empty());
    }

    #[test]
    fn test_build_pipeline_config_ref() {
        let tmp = tempdir().unwrap();
        let config = BuildConfig::new()
            .with_assets_dir(tmp.path().join("assets"))
            .with_output_dir(tmp.path().join("output"))
            .with_temp_dir(tmp.path().join("temp"));
        std::fs::create_dir_all(tmp.path().join("assets")).unwrap();

        let pipeline = BuildPipeline::new(config.clone()).unwrap();
        assert_eq!(pipeline.config().app_name(), config.app_name());
    }

    #[test]
    fn test_build_pipeline_build_output() {
        let tmp = tempdir().unwrap();
        let assets_dir = tmp.path().join("assets");
        let output_dir = tmp.path().join("output");
        std::fs::create_dir_all(&assets_dir).unwrap();
        std::fs::write(assets_dir.join("a.txt"), b"aaa").unwrap();
        std::fs::write(assets_dir.join("b.txt"), b"bbb").unwrap();

        let config = BuildConfig::new()
            .with_assets_dir(&assets_dir)
            .with_output_dir(&output_dir)
            .with_temp_dir(tmp.path().join("temp"));

        let pipeline = BuildPipeline::new(config).unwrap();
        let artifact = pipeline.build().unwrap();
        assert!(output_dir.exists());
        assert!(artifact.path.join("a.txt").exists());
        assert!(artifact.path.join("b.txt").exists());
    }

    #[test]
    fn test_build_pipeline_clean() {
        let tmp = tempdir().unwrap();
        let assets_dir = tmp.path().join("assets");
        let output_dir = tmp.path().join("output");
        let temp_dir = tmp.path().join("temp");
        std::fs::create_dir_all(&assets_dir).unwrap();
        std::fs::write(assets_dir.join("a.txt"), b"aaa").unwrap();
        std::fs::create_dir_all(&output_dir).unwrap();
        std::fs::write(output_dir.join("placeholder"), b"x").unwrap();
        std::fs::create_dir_all(&temp_dir).unwrap();

        let config = BuildConfig::new()
            .with_assets_dir(&assets_dir)
            .with_output_dir(&output_dir)
            .with_temp_dir(&temp_dir);
        let pipeline = BuildPipeline::new(config).unwrap();
        pipeline.clean().unwrap();
        assert!(!output_dir.exists());
        assert!(!temp_dir.exists());
    }

    #[test]
    fn test_build_pipeline_platform_target() {
        let tmp = tempdir().unwrap();
        let config = BuildConfig::new()
            .with_assets_dir(tmp.path().join("assets"))
            .with_output_dir(tmp.path().join("output"))
            .with_temp_dir(tmp.path().join("temp"));
        std::fs::create_dir_all(tmp.path().join("assets")).unwrap();

        let pipeline = BuildPipeline::new(config).unwrap();
        assert_eq!(pipeline.platform_target(), PlatformTarget::current());
    }

    #[test]
    fn test_sign_info_debug_and_fields() {
        let info = SignInfo {
            signature: vec![1, 2, 3],
            certificate: vec![4, 5, 6],
            timestamp: Some("2024".to_string()),
        };
        let s = format!("{:?}", info);
        assert!(s.contains("SignInfo"));
    }
}
