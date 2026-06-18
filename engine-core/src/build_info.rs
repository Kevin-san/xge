use std::env;

pub const BUILD_COMMIT_HASH: &str = env!("BUILD_COMMIT_HASH");
pub const BUILD_TIMESTAMP: &str = env!("BUILD_TIMESTAMP");
pub const ENGINE_VERSION: &str = env!("CARGO_PKG_VERSION");
