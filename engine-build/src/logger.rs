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
    }

    #[test]
    fn test_build_logger_new() {
        let logger = BuildLogger::new(true);
        assert!(logger.verbose);
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
    }

    #[test]
    fn test_build_report_new() {
        let report = BuildReport::new();
        assert!(report.stages.is_empty());
        assert_eq!(report.warnings(), 0);
        assert_eq!(report.errors(), 0);
    }

    #[test]
    fn test_build_report_add_stage() {
        let mut report = BuildReport::new();
        report.add_stage("Init", Duration::from_secs(1), 100);
        assert_eq!(report.stages.len(), 1);
    }

    #[test]
    fn test_build_report_to_html() {
        let mut report = BuildReport::new();
        report.add_stage("Init", Duration::from_secs(1), 100);
        let html = report.to_html();
        assert!(html.contains("Build Report"));
        assert!(html.contains("Init"));
    }
}