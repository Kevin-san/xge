use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    
    let commit_hash = get_commit_hash();
    println!("cargo:rustc-env=BUILD_COMMIT_HASH={}", commit_hash);
    
    let timestamp = get_build_timestamp();
    println!("cargo:rustc-env=BUILD_TIMESTAMP={}", timestamp);
}

fn get_commit_hash() -> String {
    if let Ok(output) = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
    {
        if output.status.success() {
            if let Ok(commit) = String::from_utf8(output.stdout) {
                return commit.trim().to_string();
            }
        }
    }
    "unknown".to_string()
}

fn get_build_timestamp() -> String {
    if let Ok(output) = Command::new("date")
        .args(["+%Y-%m-%dT%H:%M:%S%z"])
        .output()
    {
        if output.status.success() {
            if let Ok(date) = String::from_utf8(output.stdout) {
                return date.trim().to_string();
            }
        }
    }
    "unknown".to_string()
}