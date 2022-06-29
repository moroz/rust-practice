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

    let children = fs::read_dir(&target_directory);

    if let Err(_) = children {
        println!("Could not list directory {}", &target_directory.display());
        std::process::exit(1);
    }

    let paths: Vec<_> = children
        .unwrap()
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
        let original_path = path.clone();
        let uuid = &path.file_name().unwrap().to_str().unwrap();
        let dir_target = target_directory.clone().join(&uuid[0..2]);
        if !&dir_target.is_dir() {
            println!("Creating directory {}", &dir_target.display());
            fs::create_dir(&dir_target).expect(
                format!("Could not create directory at {}", &dir_target.display()).as_str(),
            );
        }
        let child_target = &dir_target.join(uuid);
        match fs::rename(&original_path, &child_target) {
            Ok(_) => {
                println!(
                    "Successfully moved {} to {}",
                    &original_path.display(),
                    &child_target.display()
                )
            }
            _ => {
                println!(
                    "Could not move the directory from {} to {}",
                    &original_path.display(),
                    &child_target.display()
                )
            }
        }
    }
}
