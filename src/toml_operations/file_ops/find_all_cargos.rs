//! Defines the actions around finding all the Cargo.toml files in the current directory.
use walkdir::{DirEntry, WalkDir};
use std::path::PathBuf;
use nanoservices_utils::safe_eject;
use nanoservices_utils::errors::{NanoServiceError, NanoServiceErrorStatus};


/// The interface for finding all the Cargo.toml files in the current directory.
/// 
/// # Notes
/// This would be better to refactor out the `include_cache` directly in the `find_all_cargos` function later
/// or just remove the `include_cache` parameter completely and always check the cache too.
/// 
/// # Arguments
/// * `include_cache`: A boolean value indicating if the `.nanoservices_cache` directory should be included.
/// 
/// # Returns
/// A vector of all the paths to the Cargo.toml files.
pub fn find_all_cargos_interface(include_cache: bool) -> Result<Vec<PathBuf>, NanoServiceError> {
    match include_cache {
        false => {
            return safe_eject!(
                find_all_cargos(std::env::current_dir().unwrap()),
                NanoServiceErrorStatus::Unknown,
                "getting all cargo paths in prep_nanoservices_once"
            )
        },
        true => {
            return safe_eject!(
                find_all_cargos_inc_cache(std::env::current_dir().unwrap()),
                NanoServiceErrorStatus::Unknown,
                "getting all cargo paths in prep_nanoservices_once"
            )
        }
    };
}


/// Checks if the entry is a Cargo.toml file.
///
/// # Arguments
/// * `entry` - A reference to the DirEntry to check.
///
/// # Returns
/// A boolean value indicating if the entry is a Cargo.toml file.
fn is_cargo_toml(entry: &DirEntry) -> bool {
    entry.file_name() == "Cargo.toml"
}


/// Finds all the Cargo.toml files in the current directory and all subdirectories as long as they
/// are not in the `.nanoservices_cache directory`.
/// 
/// # Arguments
/// * `base_path`: The path in which we will perform our search of all subdirectories (can be std::env::current_dir() for cli-tool)
///
/// # Returns
/// A vector of all the paths to the Cargo.toml files.
pub fn find_all_cargos(base_path: PathBuf) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let excluded_dir = base_path.join(".nanoservices_cache");

    let mut paths = Vec::new();

    for entry in WalkDir::new(&base_path)
        .into_iter()
        .filter_map(Result::ok) // Ignore any errors during iteration
        .filter(|e| !e.path().starts_with(&excluded_dir) && is_cargo_toml(e)) // Exclude specified directory and check for Cargo.toml
    {
        let relative_path = entry.path().strip_prefix(&base_path).unwrap();
        paths.push(PathBuf::from(relative_path));
    }
    Ok(paths)
}


/// Finds all the Cargo.toml files in the current directory and all subdirectories including the `.nanoservices_cache directory`.
/// 
/// # Arguments
/// * `base_path`: The path in which we will perform our search of all subdirectories (can be std::env::current_dir() for cli-tool)
///
/// # Returns
/// A vector of all the paths to the Cargo.toml files.
pub fn find_all_cargos_inc_cache(base_path: PathBuf) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut paths = Vec::new();

    for entry in WalkDir::new(&base_path)
        .into_iter()
        .filter_map(Result::ok) // Ignore any errors during iteration
        .filter(|e| is_cargo_toml(e)) // Exclude specified directory and check for Cargo.toml
    {
        let relative_path = entry.path().strip_prefix(&base_path).unwrap();
        paths.push(PathBuf::from(relative_path));
    }
    Ok(paths)
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_find_all_cargos_at_base() {
        let base_path = PathBuf::from("./tests/path_testing");
        let expected_path = vec![
            PathBuf::from("one/Cargo.toml"), 
            PathBuf::from("one/three/Cargo.toml"), 
            PathBuf::from("two/Cargo.toml")
        ];

        assert_eq!(
            expected_path,
            find_all_cargos(base_path).unwrap()
        );
    }

    #[test]
    fn test_find_all_cargos_at_one() {
        let base_path = PathBuf::from("./tests/path_testing/one");

        let expected_path = vec![
            PathBuf::from("Cargo.toml"), 
            PathBuf::from("three/Cargo.toml"), 
        ];
        
        assert_eq!(
            expected_path,
            find_all_cargos(base_path).unwrap()
        );
    }

}