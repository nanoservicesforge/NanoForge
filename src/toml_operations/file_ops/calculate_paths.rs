//! Defines the actions around calculating paths of files
use std::path::PathBuf;
use pathdiff::diff_paths;
use std::io::ErrorKind;
use crate::docker_files::cache::process_image_name;

/// Calculates the relative path to the extracted nanoservice and the Cargo.toml file.
///
/// # Notes
/// entrypoint of the nanoservice is also added to the relative path at the end.
///
/// # Arguments
/// * `cargo_path` - The path to the Cargo.toml file.
/// * `image` - The name of the Docker image.
/// * `entry` - The entrypoint of the nanoservice.
/// * `nanoservices_path` - Where the nanoservices is (for cli-tool use `CACHE_NANOSERVICES_DIR.clone()`)
///
/// # Returns
/// The relative path to the extracted nanoservice.
pub fn calculate_relative_path(
        cargo_path: &PathBuf, 
        image: &String, 
        entry: &String,
        nanoservices_path: &PathBuf
    ) -> std::io::Result<PathBuf> {
    let current_dir = std::env::current_dir()?;
    let base_path = cargo_path.parent().unwrap();

    let target_path = nanoservices_path;

    // need to strip the current directory from the two paths in order to get an accurate compare
    // as both paths need to be from the same starting point for the `diff_paths` function to work
    // otherwise the `diff_paths` function will return a `None`
    let stripped_target_path = target_path.strip_prefix(&current_dir).unwrap();
    let stripped_base_path = match base_path.strip_prefix(&current_dir) {
        Ok(path) => path,
        Err(_) => base_path
    };

    let processed_image = process_image_name(&image);
    let stripped_target_path = stripped_target_path.join(processed_image);

    let relative_path = match diff_paths(stripped_target_path, stripped_base_path) {
        Some(path) => path,
        None => return Err(
            std::io::Error::new(
                ErrorKind::NotFound, 
                "diff paths not calculated for the calculation of relative paths"
            )
        ) 
    };
    Ok(relative_path.join(entry))
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_calculate_relative_path_for_full_paths() {

        let cargo_path = PathBuf::from(
            "/Users/maxwellflitton/Documents/github/personal/nanoservices/NanoForge/tests/path_testing/one/three/Cargo.toml"
        );
        let nanoservice_cache_path = PathBuf::from(
            "/Users/maxwellflitton/Documents/github/personal/nanoservices/NanoForge/tests/path_testing/.nanoservices_cache/domain_services/nanoservices/"
        );

        let outcome = calculate_relative_path(
            &cargo_path, 
            &"two".to_string(), 
            &".".to_string(), 
            &nanoservice_cache_path
        ).unwrap();

        assert_eq!(
            "../../.nanoservices_cache/domain_services/nanoservices/two/.",
            outcome.to_str().unwrap()
        );

    }

    #[test]
    fn test_calculate_relative_path_for_relative_cargo_path() {

        let cargo_path = PathBuf::from(
            "tests/path_testing/one/three/Cargo.toml"
        );
        let nanoservice_cache_path = PathBuf::from(
            "/Users/maxwellflitton/Documents/github/personal/nanoservices/NanoForge/tests/path_testing/.nanoservices_cache/domain_services/nanoservices/"
        );

        let outcome = calculate_relative_path(
            &cargo_path, 
            &"two".to_string(), 
            &".".to_string(), 
            &nanoservice_cache_path
        ).unwrap();

        assert_eq!(
            "../../.nanoservices_cache/domain_services/nanoservices/two/.",
            outcome.to_str().unwrap()
        );

    }

}