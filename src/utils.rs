use std::{env, fs, os::unix::fs::PermissionsExt, path::Path};

pub fn validate_file(name: &str) -> Option<String> {
    let path = env::var("PATH").unwrap_or_default();
    for dir in path.split(':') {
        let full_path = Path::new(dir).join(name);
        if let Ok(meta) = full_path.metadata() {
            if meta.is_file() && meta.permissions().mode() & 0o111 != 0 {
                return Some(full_path.to_string_lossy().into_owned());
            }
        }
    }
    None
}

pub fn find_executable() -> Vec<String> {
    let mut results = Vec::new();
    let path = env::var("PATH").unwrap_or_default();
    for dir in path.split(':') {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                if let Ok(meta) = entry.metadata() {
                    if meta.is_file() && meta.permissions().mode() & 0o111 != 0 {
                        results.push(entry.file_name().to_string_lossy().into_owned());
                    }
                }
            }
        }
    }
    results.sort();
    results
}
