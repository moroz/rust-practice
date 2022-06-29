use regex::Regex;
use std::env;
use std::fs;
use std::path::PathBuf;

fn resolve_target_directory() -> PathBuf {
    let path = env::var("ASSETS_STORAGE_DIR").unwrap_or("/usr/local/share/gg/uploads".to_string());
    let mut buf = PathBuf::from(&path[..]);
    buf.push("images");
    return buf;
}

fn main() {
    let uuid_regex =
        Regex::new("^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$").unwrap();
    let target_directory = resolve_target_directory();

    let children = fs::read_dir(&target_directory).unwrap();

    let paths: Vec<_> = children
        .into_iter()
        .map(|path| path.unwrap().path())
        .filter(|path| uuid_regex.is_match(&path.file_name().unwrap().to_str().unwrap()))
        .collect();

    println!(
        "Preparing to move {} directories inside {}",
        &paths.len(),
        &target_directory.display()
    );

    for path in paths {
        let mut target = target_directory.clone();
        let uuid_prefix = &path.file_name().unwrap().to_str().unwrap()[0..2];
        target.push(uuid_prefix);
        if !&target.is_dir() {
            println!("Creating directory {}", &target.display())
        }
    }
}
