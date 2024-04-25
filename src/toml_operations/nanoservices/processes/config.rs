//! Defines the processes around configuring the `Cargo.toml` files for nanoservices.
use std::collections::HashSet;

use crate::docker_files::cache::CACHE_NANOSERVICES_DIR;
use crate::toml_operations::nanoservices::configure_cargo_toml::config_cargo;
use crate::toml_operations::nanoservices::processes::get::get_nanoservices_once;
use crate::toml_operations::nanoservices::get_all::CargoDependencies;
use crate::toml_operations::file_ops::{
    read::read_toml,
    write::write_toml,
};

use nanoservices_utils::errors::NanoServiceError;


/// Configures the `Cargo.toml` files for the nanoservices.
/// 
/// # Note
/// The function will continue to loop until all the nanoservices have been configured.
pub fn recursive_config_nanoservices() -> Result<(), NanoServiceError> {
    let (toml_paths, _, cargo_dependencies) = get_nanoservices_once(
        HashSet::new(),
        false
    )?;
    config_nanoservices_once(cargo_dependencies)?;
    let mut existing_tomls = toml_paths;

    loop {
        let (cargo_paths, _, cargo_dependencies) = get_nanoservices_once(
            existing_tomls.clone().into_iter().collect(),
            true
        )?;

        if &cargo_paths.len() == &0 {
            break;
        }
        for cargo_path in cargo_paths {
            existing_tomls.push(cargo_path);
        }
        config_nanoservices_once(cargo_dependencies)?;
    }
    Ok(())

}


/// Configures the `Cargo.toml` files for the nanoservices.
/// 
/// # Arguments
/// * `cargo_dependencies`: A HashMap of all the dependencies found in the `Cargo.toml` files with the path to the `Cargo.toml` file
///                         as the key.
pub fn config_nanoservices_once(cargo_dependencies: CargoDependencies) -> Result<(), NanoServiceError> {
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
    Ok(())
}
