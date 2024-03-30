use walkdir::{DirEntry, WalkDir};
use std::path::PathBuf;
use pathdiff::diff_paths;
use crate::docker_files::cache::CACHE_NANOSERVICES_DIR;


fn is_cargo_toml(entry: &DirEntry) -> bool {
    entry.file_name() == "Cargo.toml"
}


pub fn find_all_cargos() -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let current_dir = std::env::current_dir()?;
    let excluded_dir = current_dir.join(".nanoservices_cache");

    let mut paths = Vec::new();

    for entry in WalkDir::new(&current_dir)
        .into_iter()
        .filter_map(Result::ok) // Ignore any errors during iteration
        .filter(|e| !e.path().starts_with(&excluded_dir) && is_cargo_toml(e)) // Exclude specified directory and check for Cargo.toml
    {
        let relative_path = entry.path().strip_prefix(&current_dir).unwrap();
        paths.push(PathBuf::from(relative_path));
    }
    Ok(paths)
}


pub fn calculate_relative_path(cargo_path: &PathBuf, image: String, entry: String) -> PathBuf {
    let current_dir = std::env::current_dir().unwrap();
    let base_path = cargo_path.parent().unwrap();
    let target_path = CACHE_NANOSERVICES_DIR.clone();
    let stripped_target_path = target_path.strip_prefix(&current_dir).unwrap();
    let processed_image = image.replace("/", "_").replace(":", "_");
    let stripped_target_path = stripped_target_path.join(processed_image);
    let relative_path = diff_paths(
        stripped_target_path, base_path).unwrap();
    relative_path.join(entry)
}
