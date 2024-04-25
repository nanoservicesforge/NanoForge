//! Defines the processes around getting all the nanoservices.
use std::path::PathBuf;
use std::collections::HashSet;
use crate::toml_operations::nanoservices::get_all::{
    get_all_nanoservices,
    CargoDependencies
};
use crate::toml_operations::file_ops::find_all_cargos::find_all_cargos_interface;

use nanoservices_utils::errors::NanoServiceError;
use crate::toml_operations::nanoservices::kernel::Nanoservice;


/// Gets all the nanoservices from the TOML files in the current directory and subdirectories of the current directory.
/// 
/// # Arguments
/// * `existing_tomls`: A HashSet of all the existing `Cargo.toml` from a previous run of this function to prevent duplication.
/// * `include_cache`: If `true` the cache will be included in the search for `Cargo.toml` files.
/// 
/// # Returns
/// A tuple of: 
///     all the paths to the `Cargo.toml` files, 
///     a HashSet of all the nanoservices found in the `Cargo.toml` files, 
///     and a HashMap of all the dependencies found in the `Cargo.toml` files.
pub fn get_nanoservices_once(
    existing_tomls: HashSet<PathBuf>,
    include_cache: bool
) -> Result<(Vec<PathBuf>, HashSet<(std::string::String, Nanoservice)>, CargoDependencies), NanoServiceError> {
    // extract this out into an interface
    let mut all_cargo_paths = find_all_cargos_interface(include_cache)?;

    // wipe the existing paths from the new ones
    all_cargo_paths.retain(|item| !existing_tomls.contains(item));

    println!("Cargo paths found: {:?}", all_cargo_paths);
    let cargo_paths_ref = all_cargo_paths.clone();
    let (cargo_dependencies, all_nanoservices) = get_all_nanoservices(all_cargo_paths)?;
    Ok((cargo_paths_ref, all_nanoservices, cargo_dependencies))
}
