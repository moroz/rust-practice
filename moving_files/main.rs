use std::env;
use std::fs;

fn resolve_target_directory() -> String {
    return env::var("ASSETS_STORAGE_DIR").unwrap_or("/usr/local/share/gg/uploads/".to_string());
}

fn main() {
    let target_directory = resolve_target_directory();
}
