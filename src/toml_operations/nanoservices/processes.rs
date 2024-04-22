//! Defines the processes around preparing the nanoservices for a build.
use std::path::PathBuf;
use std::collections::HashSet;

use crate::docker_files::{
    download_nanoservice,
    cache::{
        wipe_and_create_cache,
        CACHE_NANOSERVICES_DIR
    }
};
use crate::toml_operations::nanoservices::get_all::get_all_nanoservices;
use crate::toml_operations::nanoservices::configure_cargo_toml::config_cargo;
use crate::toml_operations::file_ops::{
    read::read_toml,
    write::write_toml,
    find_all_cargos::find_all_cargos_interface
};

use nanoservices_utils::errors::NanoServiceError;


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
    if wipe_cache == true {
        wipe_and_create_cache();
    }

    // extract this out into an interface
    let mut all_cargo_paths = find_all_cargos_interface(include_cache)?;

    // wipe the existing paths from the new ones
    all_cargo_paths.retain(|item| !existing_tomls.contains(item));

    println!("Cargo paths found: {:?}", all_cargo_paths);
    let cargo_paths_ref = all_cargo_paths.clone();
    let (cargo_dependencies, all_nanoservices) = get_all_nanoservices(all_cargo_paths)?;
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

    for (path, nanoservices) in cargo_dependencies {
        // we can unwrap the `into_raw()` function because the cargo.toml will not be here if it did not have
        // nanoservices in the contents
        let raw_dog_cargo = read_toml(path.to_str().unwrap())?.into_raw().unwrap();
        let raw_dog_cargo = config_cargo(
            raw_dog_cargo, 
            nanoservices, 
            CACHE_NANOSERVICES_DIR.clone(),
            path.clone()
        )?;
        write_toml(path.to_str().unwrap(), raw_dog_cargo)?
    }
    Ok((cargo_paths_ref, nanoservices_ref))
}