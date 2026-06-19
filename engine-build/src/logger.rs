//! Build logging and reporting
//!
//! Provides build progress tracking and report generation.

use std::fs;
use std::path::Path;
use std::time::{Duration, Instant};

/// Build stage enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuildStage {
    Init,
    Compile,
    ProcessAssets,
    Package,
    Sign,
    Done,
}

impl BuildStage {
    /// Get stage name
    pub fn name(&self) -> &'static str {
        match self {
            BuildStage::Init => "Initialize",
            BuildStage::Compile => "Compile",
            BuildStage::ProcessAssets => "Process Assets",
            BuildStage::Package => "Package",
            BuildStage::Sign => "Sign",
            BuildStage::Done => "Done",
        }
    }

    /// Get stage index (for progress calculation)
    pub fn index(&self) -> u8 {
        match self {
            BuildStage::Init => 0,
            BuildStage::Compile => 1,
            BuildStage::ProcessAssets => 2,
            BuildStage::Package => 3,
            BuildStage::Sign => 4,
            BuildStage::Done => 5,
        }
    }

    /// Total number of stages
    pub const fn total() -> u8 {
        6
    }
}

/// Build logger for output
pub struct BuildLogger {
    verbose: bool,
    start: Instant,
}

impl BuildLogger {
    /// Create new logger
    pub fn new(verbose: bool) -> Self {
        Self {
            verbose,
            start: Instant::now(),
        }
    }

    /// Log info message
    pub fn info(&self, msg: &str) {
        println!("[INFO] {}", msg);
    }

    /// Log warning message
    pub fn warn(&self, msg: &str) {
        println!("[WARN] {}", msg);
    }

    /// Log error message
    pub fn error(&self, msg: &str) {
        println!("[ERROR] {}", msg);
    }

    /// Log progress
    pub fn progress(&self, percent: u8, msg: &str) {
        let elapsed = self.start.elapsed();
        println!("[{:3}%] {} ({}s)", percent, msg, elapsed.as_secs());
    }

    /// Log verbose message (only if verbose mode)
    pub fn verbose(&self, msg: &str) {
        if self.verbose {
            println!("[VERBOSE] {}", msg);
        }
    }

    /// Get elapsed time
    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }
}

/// Build progress tracker
pub struct BuildProgress {
    stage: BuildStage,
    percent: u8,
    message: String,
}

impl BuildProgress {
    /// Create new progress tracker
    pub fn new() -> Self {
        Self {
            stage: BuildStage::Init,
            percent: 0,
            message: String::new(),
        }
    }

    /// Set current stage
    pub fn set_stage(&mut self, stage: BuildStage) {
        self.stage = stage;
        self.percent = (stage.index() * 100) / BuildStage::total();
    }

    /// Set progress percent
    pub fn set_percent(&mut self, percent: u8) {
        self.percent = percent.min(100);
    }

    /// Set message
    pub fn set_message(&mut self, message: &str) {
        self.message = message.to_string();
    }

    /// Get current stage
    pub fn stage(&self) -> BuildStage {
        self.stage
    }

    /// Get percent
    pub fn percent(&self) -> u8 {
        self.percent
    }

    /// Get message
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl Default for BuildProgress {
    fn default() -> Self {
        Self::new()
    }
}

/// Stage report entry
#[derive(Debug, Clone)]
pub struct StageReport {
    name: String,
    duration: Duration,
    size: u64,
}

/// Build report
pub struct BuildReport {
    stages: Vec<StageReport>,
    warnings: u32,
    errors: u32,
}

impl BuildReport {
    /// Create new report
    pub fn new() -> Self {
        Self {
            stages: Vec::new(),
            warnings: 0,
            errors: 0,
        }
    }

    /// Add stage report
    pub fn add_stage(&mut self, name: &str, duration: Duration, size: u64) {
        self.stages.push(StageReport {
            name: name.to_string(),
            duration,
            size,
        });
    }

    /// Add warning
    pub fn add_warning(&mut self) {
        self.warnings += 1;
    }

    /// Add error
    pub fn add_error(&mut self) {
        self.errors += 1;
    }

    /// Get total duration
    pub fn total_duration(&self) -> Duration {
        self.stages.iter().map(|s| s.duration).sum()
    }

    /// Get total size
    pub fn total_size(&self) -> u64 {
        self.stages.iter().map(|s| s.size).sum()
    }

    /// Get warning count
    pub fn warnings(&self) -> u32 {
        self.warnings
    }

    /// Get error count
    pub fn errors(&self) -> u32 {
        self.errors
    }

    /// Generate HTML report
    pub fn to_html(&self) -> String {
        let mut html = String::new();
        html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str("<title>Build Report</title>\n");
        html.push_str("<style>\n");
        html.push_str("body { font-family: Arial, sans-serif; margin: 20px; }\n");
        html.push_str("table { border-collapse: collapse; width: 100%; }\n");
        html.push_str("th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }\n");
        html.push_str("th { background-color: #4CAF50; color: white; }\n");
        html.push_str(".summary { margin-bottom: 20px; }\n");
        html.push_str("</style>\n");
        html.push_str("</head>\n<body>\n");

        html.push_str("<h1>Build Report</h1>\n");
        html.push_str("<div class=\"summary\">\n");
        html.push_str(&format!(
            "<p><strong>Total Duration:</strong> {:.2}s</p>\n",
            self.total_duration().as_secs_f64()
        ));
        html.push_str(&format!(
            "<p><strong>Total Size:</strong> {} bytes</p>\n",
            self.total_size()
        ));
        html.push_str(&format!(
            "<p><strong>Warnings:</strong> {}</p>\n",
            self.warnings
        ));
        html.push_str(&format!(
            "<p><strong>Errors:</strong> {}</p>\n",
            self.errors
        ));
        html.push_str("</div>\n");

        html.push_str("<h2>Build Stages</h2>\n");
        html.push_str("<table>\n");
        html.push_str("<tr><th>Stage</th><th>Duration</th><th>Size</th></tr>\n");
        for stage in &self.stages {
            html.push_str(&format!(
                "<tr><td>{}</td><td>{:.2}s</td><td>{} bytes</td></tr>\n",
                stage.name,
                stage.duration.as_secs_f64(),
                stage.size
            ));
        }
        html.push_str("</table>\n");

        html.push_str("</body>\n</html>");
        html
    }

    /// Save HTML report to file
    pub fn save_html(&self, path: impl AsRef<Path>) -> crate::BuildResult<()> {
        let html = self.to_html();
        fs::write(path.as_ref(), html)?;
        Ok(())
    }
}

impl Default for BuildReport {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_stage_name() {
        assert_eq!(BuildStage::Init.name(), "Initialize");
        assert_eq!(BuildStage::Compile.name(), "Compile");
        assert_eq!(BuildStage::ProcessAssets.name(), "Process Assets");
        assert_eq!(BuildStage::Package.name(), "Package");
        assert_eq!(BuildStage::Sign.name(), "Sign");
        assert_eq!(BuildStage::Done.name(), "Done");
    }

    #[test]
    fn test_build_stage_index() {
        assert_eq!(BuildStage::Init.index(), 0);
        assert_eq!(BuildStage::Compile.index(), 1);
        assert_eq!(BuildStage::ProcessAssets.index(), 2);
        assert_eq!(BuildStage::Package.index(), 3);
        assert_eq!(BuildStage::Sign.index(), 4);
        assert_eq!(BuildStage::Done.index(), 5);
    }

    #[test]
    fn test_build_stage_total() {
        assert_eq!(BuildStage::total(), 6);
    }

    #[test]
    fn test_build_stage_debug() {
        let s = format!("{:?}", BuildStage::Compile);
        assert!(s.contains("Compile"));
    }

    #[test]
    fn test_build_logger_new() {
        let logger = BuildLogger::new(true);
        assert!(logger.verbose);
        let logger2 = BuildLogger::new(false);
        assert!(!logger2.verbose);
    }

    #[test]
    fn test_build_logger_info() {
        let logger = BuildLogger::new(true);
        logger.info("test info message");
    }

    #[test]
    fn test_build_logger_warn() {
        let logger = BuildLogger::new(true);
        logger.warn("test warn message");
    }

    #[test]
    fn test_build_logger_error() {
        let logger = BuildLogger::new(true);
        logger.error("test error message");
    }

    #[test]
    fn test_build_logger_progress() {
        let logger = BuildLogger::new(true);
        logger.progress(50, "halfway");
    }

    #[test]
    fn test_build_logger_verbose_mode() {
        let logger = BuildLogger::new(true);
        logger.verbose("verbose message");
    }

    #[test]
    fn test_build_logger_elapsed() {
        let logger = BuildLogger::new(true);
        let elapsed = logger.elapsed();
        // 应该是非常短的时间
        assert!(elapsed.as_secs() < 10);
    }

    #[test]
    fn test_build_progress_new() {
        let progress = BuildProgress::new();
        assert_eq!(progress.stage(), BuildStage::Init);
        assert_eq!(progress.percent(), 0);
    }

    #[test]
    fn test_build_progress_set_stage() {
        let mut progress = BuildProgress::new();
        progress.set_stage(BuildStage::Compile);
        assert_eq!(progress.stage(), BuildStage::Compile);
        // Compile stage 为 index 1，总 stages 为 6，约 16%
        assert!(progress.percent() > 0);
    }

    #[test]
    fn test_build_progress_set_percent() {
        let mut progress = BuildProgress::new();
        progress.set_percent(50);
        assert_eq!(progress.percent(), 50);
    }

    #[test]
    fn test_build_progress_set_percent_clamped_at_100() {
        let mut progress = BuildProgress::new();
        progress.set_percent(200);
        assert_eq!(progress.percent(), 100);
    }

    #[test]
    fn test_build_progress_set_message() {
        let mut progress = BuildProgress::new();
        progress.set_message("compiling modules");
        assert_eq!(progress.message(), "compiling modules");
    }

    #[test]
    fn test_build_progress_default() {
        let progress: BuildProgress = Default::default();
        assert_eq!(progress.stage(), BuildStage::Init);
    }

    #[test]
    fn test_build_report_new() {
        let report = BuildReport::new();
        assert_eq!(report.warnings(), 0);
        assert_eq!(report.errors(), 0);
        assert_eq!(report.total_duration().as_secs(), 0);
        assert_eq!(report.total_size(), 0);
    }

    #[test]
    fn test_build_report_add_stage() {
        let mut report = BuildReport::new();
        report.add_stage("Init", Duration::from_millis(500), 1024);
        report.add_stage("Compile", Duration::from_millis(1500), 0);
        assert_eq!(report.total_duration().as_millis(), 2000);
        assert_eq!(report.total_size(), 1024);
    }

    #[test]
    fn test_build_report_add_warnings() {
        let mut report = BuildReport::new();
        report.add_warning();
        report.add_warning();
        assert_eq!(report.warnings(), 2);
    }

    #[test]
    fn test_build_report_add_errors() {
        let mut report = BuildReport::new();
        report.add_error();
        report.add_error();
        report.add_error();
        assert_eq!(report.errors(), 3);
    }

    #[test]
    fn test_build_report_to_html() {
        let mut report = BuildReport::new();
        report.add_stage("Init", Duration::from_secs(1), 100);
        report.add_warning();
        let html = report.to_html();
        assert!(html.contains("Build Report"));
        assert!(html.contains("Init"));
        assert!(html.contains("Warnings"));
    }

    #[test]
    fn test_build_report_default() {
        let report: BuildReport = Default::default();
        assert_eq!(report.warnings(), 0);
    }

    #[test]
    fn test_build_report_save_html() {
        let dir = std::env::temp_dir();
        let path = dir.join("test_report.html");
        let report = BuildReport::new();
        report.save_html(&path).unwrap();
        assert!(path.exists());
        let _ = std::fs::remove_file(&path);
    }
}
