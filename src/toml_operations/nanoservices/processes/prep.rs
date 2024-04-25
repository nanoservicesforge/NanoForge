//! Defines the processes around preparing the nanoservices for a build.
use std::path::PathBuf;
use std::collections::HashSet;

use crate::toml_operations::nanoservices::processes::install::install_nanoservices_once;
use crate::toml_operations::nanoservices::processes::config::config_nanoservices_once;

use nanoservices_utils::errors::NanoServiceError;


/// Loops through the all the directories including the nanoservices cache to download the nanoservices and configure the 
/// `Cargo.toml` files. The function will continue to loop until all the nanoservices have been downloaded and configured.
pub fn recursive_prep_nanoservices() -> Result<(), NanoServiceError> {
    let (mut main_cargo_paths, mut main_nano_names) = prep_nanoservices_once(
        true, 
        HashSet::new(),
        HashSet::new(),
        false
    )?;

    loop {
        let (cargo_paths, nano_names) = prep_nanoservices_once(
            false, 
            main_cargo_paths.iter().cloned().collect(),
            main_nano_names.iter().cloned().collect(),
            true
        )?;

        if &cargo_paths.len() == &0 {
            break;
        }
        for cargo_path in cargo_paths {
            main_cargo_paths.push(cargo_path);
        }
        for nano_name in nano_names {
            main_nano_names.insert(nano_name);
        }
    }
    Ok(())
}


/// Gets all the nanoservices from the TOML files in the current directory and subdirectories of the current directory.
/// The function then downloads the nanoservices from docker and writes the new `Cargo.toml` files back to the disk.
/// 
/// # Arguments
/// * `wipe_cache`: If `true` the nanoservices cache will be wiped before the installation of the nanoservices.
/// * `existing_tomls`: A HashSet of all the existing `Cargo.toml` from a previous run of this function to prevent duplication.
/// * `existing_nanoservices`: A HashSet of all the existing nanoservices from a previous run of this function to prevent duplication.
/// * `include_cache`: A boolean value indicating if the `.nanoservices_cache` directory should be included.
/// 
/// # Returns
/// A tuple of all the paths to the `Cargo.toml` files and a HashSet of all the nanoservices found in the `Cargo.toml` files.
pub fn prep_nanoservices_once(
    wipe_cache: bool, 
    existing_tomls: HashSet<PathBuf>,
    existing_nanoservices: HashSet<String>,
    include_cache: bool
) -> Result<(Vec<PathBuf>, HashSet<String>), NanoServiceError> {

    let (cargo_paths_ref, nanoservices_ref, cargo_dependencies) = install_nanoservices_once(
        wipe_cache, 
        existing_tomls,
        existing_nanoservices,
        include_cache
    )?;
    config_nanoservices_once(cargo_dependencies)?;
    Ok((cargo_paths_ref, nanoservices_ref))
}
