//! Defines the processes around just installing nanoservices.
use std::path::PathBuf;
use std::collections::HashSet;

use crate::docker_files::{
    download_nanoservice,
    cache::wipe_and_create_cache
};
use crate::toml_operations::nanoservices::get_all::CargoDependencies;
use crate::toml_operations::nanoservices::processes::get::get_nanoservices_once;

use nanoservices_utils::errors::NanoServiceError;


/// Loops through the all the directories including the nanoservices cache to download the nanoservices.
/// 
/// # Note
/// The function will continue to loop until all the nanoservices have been downloaded.
pub fn recurrsive_install_nanoservices() -> Result<(), NanoServiceError> {
    // wipe and initially install the nanoservices
    let (mut main_cargo_paths, mut main_nano_names, mut main_cargo_dependencies) = install_nanoservices_once(
        true, 
        HashSet::new(),
        HashSet::new(),
        false
    )?;

    loop {
        // include cache and don't wipe the cache on recurring installs
        let (cargo_paths, nano_names, cargo_dependencies) = install_nanoservices_once(
            false, 
            main_cargo_paths.iter().cloned().collect(),
            main_nano_names.iter().cloned().collect(),
            true
        )?;

        // break if no more nanoservices are found
        if &cargo_paths.len() == &0 {
            break;
        }
        // update the cache of cargo paths, nanoservices and dependencies to be reiinserted into the next check
        for cargo_path in cargo_paths {
            main_cargo_paths.push(cargo_path);
        }
        for nano_name in nano_names {
            main_nano_names.insert(nano_name);
        }
        for (cargo_path, nanos) in cargo_dependencies {
            main_cargo_dependencies.insert(cargo_path, nanos);
        }
    }
    Ok(())
}


/// Gets all the nanoservices from the TOML files in the current directory and subdirectories of the current directory.
/// The function then downloads the nanoservices from docker.
/// 
/// # Arguments
/// * `wipe_cache`: If `true` the nanoservices cache will be wiped before the installation of the nanoservices.
/// * `existing_tomls`: A HashSet of all the existing `Cargo.toml` from a previous run of this function to prevent duplication.
/// * `existing_nanoservices`: A HashSet of all the existing nanoservices from a previous run of this function to prevent duplication.
/// * `include_cache`: A boolean value indicating if the `.nanoservices_cache` directory should be included.
/// 
/// # Returns
/// A tuple of all the paths to the `Cargo.toml` files and a HashSet of all the nanoservices found in the `Cargo.toml` files.
pub fn install_nanoservices_once(
    wipe_cache: bool, 
    existing_tomls: HashSet<PathBuf>,
    existing_nanoservices: HashSet<String>,
    include_cache: bool
) -> Result<(Vec<PathBuf>, HashSet<String>, CargoDependencies), NanoServiceError> {
    if wipe_cache == true {
        wipe_and_create_cache();
    }

    let (cargo_paths_ref, all_nanoservices, cargo_dependencies) = get_nanoservices_once(
        existing_tomls,
        include_cache
    )?;

    let mut nanoservices_ref = HashSet::new();

    // download all the nanoservices from docker
    for (_name, nanoservice) in all_nanoservices {
        // add the nanoservice to the reference
        nanoservices_ref.insert(nanoservice.dev_image.clone());
        // bypass downloading the image if local is set to true
        let local = match nanoservice.local {
            Some(v) => v,
            _ => false,
        };
        if !local && !existing_nanoservices.contains(&nanoservice.dev_image) {
            let _path = download_nanoservice(&nanoservice.dev_image)?;
        }
    }
    Ok((cargo_paths_ref, nanoservices_ref, cargo_dependencies))
}
