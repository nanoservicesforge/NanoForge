use std::collections::{
    HashMap,
    HashSet
};
use std::path::PathBuf;
use crate::toml_operations::nanoservices::kernel::Nanoservice;
use crate::toml_operations::nanoservices::wipe::wipe_nanoservices;
use crate::toml_operations::read::{
    read_toml, 
    write_toml
};
use nanoservices_utils::errors::NanoServiceError;

pub type CargoDependencies = HashMap<std::path::PathBuf, Vec<(String, Nanoservice)>>;
pub type AllNanoservices = HashSet<(String, Nanoservice)>;


/// Gets all the nanoservices from the TOML files in the current directory.
/// 
/// # Notes
/// There are read and writes in this function for the loop as we need this function to be atomic for
/// `Cargo.toml` state. This is because the nanoservices might have been removed, therefore, that
/// `Cargo.toml` will not continue in the rest of the process but the wiped state needs to be
/// written removing the previously stated nanoservices.
///
/// # Returns
/// All the paths to the TOML files and the nanoservices in them,
/// A HashSet of all the nanoservices found in all the TOML files.
pub fn get_all_nanoservices(cargo_paths: Vec<PathBuf>) -> Result<(CargoDependencies, AllNanoservices), NanoServiceError> {

    // define the hashmap to hold the cargo dependencies
    let mut cargo_dependencies = HashMap::new();
    let mut all_nanoservices = HashSet::new();

    for cargo_path in cargo_paths {
        println!("cargo_path: {:?}", cargo_path);
        let mut cargo_toml = match read_toml(cargo_path.to_str().unwrap())?.into_raw() {
            Some(raw_dog) => raw_dog,
            None => continue
        };
        wipe_nanoservices(&mut cargo_toml);

        let mut buffer = Vec::new();

        let nanos = match cargo_toml.nanoservices {
            Some(nanos) => nanos,
            None => {
                // break the loop if there are no nanoservices in the cargo file
                // write the cargo file back to the disk as the nanoservices might
                // have been wiped
                write_toml(cargo_path.to_str().unwrap(), cargo_toml)?;
                continue;
            }
        };

        // loop through the nanoservices and add them to the buffer and all nanoservices hashset
        for nanoservice in nanos {
            buffer.push(nanoservice.clone());
            all_nanoservices.insert(nanoservice);
        }
        if !buffer.is_empty() {
            cargo_dependencies.insert(cargo_path, buffer);
        }
    }
    return Ok((cargo_dependencies, all_nanoservices))
}
