use std::env;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    // 获取 git commit hash
    let git_commit_hash = get_git_commit_hash().unwrap_or_else(|| "unknown".to_string());

    // 获取构建时间
    let build_timestamp = get_build_timestamp();

    // 设置环境变量供 build_info.rs 使用
    println!("cargo:rustc-env=BUILD_COMMIT_HASH={}", git_commit_hash);
    println!("cargo:rustc-env=BUILD_TIMESTAMP={}", build_timestamp);

    // 生成 artifacts.json
    let artifacts_json = format!(
        r#"{{"commit_hash":"{}","timestamp":"{}"}}"#,
        git_commit_hash, build_timestamp
    );

    let dest_file = std::path::Path::new(&out_dir).join("artifacts.json");
    std::fs::write(&dest_file, artifacts_json).ok();
}

fn get_git_commit_hash() -> Option<String> {
    Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
            } else {
                None
            }
        })
}

fn get_build_timestamp() -> String {
    chrono::Local::now()
        .format("%Y-%m-%d %H:%M:%S")
        .to_string()
}
