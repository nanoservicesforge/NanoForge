//! Defines the actions around reading, writing, and configuring TOML files.
pub mod read;
pub mod finder;
use std::collections::{
    HashMap,
    HashSet
};
use std::path::PathBuf;
use read::Nanoservice;
use toml::{Table, Value};
use crate::toml_operations::read::RawCargoToml;
use nanoservices_utils::{
    safe_eject,
    errors::{
        NanoServiceError,
        NanoServiceErrorStatus
    }
};

pub type CargoDependencies = HashMap<std::path::PathBuf, Vec<(String, Nanoservice)>>;
pub type AllNanoservices = HashSet<(String, Nanoservice)>;


/// Gets all the nanoservices from the TOML files in the current directory.
///
/// # Returns
/// All the paths to the TOML files and the nanoservices in them,
/// A HashSet of all the nanoservices found in all the TOML files.
pub fn get_all_nanoservices() -> Result<(CargoDependencies, AllNanoservices), NanoServiceError> {
    // get all the paths to the cargo files
    let cargo_paths = safe_eject!(
        finder::find_all_cargos(),
        NanoServiceErrorStatus::NotFound,
        "Getting all cargo paths when getting all nanoservices"
    )?;

    // define the hashmap to hold the cargo dependencies
    let mut cargo_dependencies = HashMap::new();
    let mut all_nanoservices = HashSet::new();

    for cargo_path in cargo_paths {
        println!("cargo_path: {:?}", cargo_path);
        let mut cargo_toml = match read::read_toml(cargo_path.to_str().unwrap())?.into_raw() {
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
                read::write_toml(cargo_path.to_str().unwrap(), cargo_toml)?;
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


/// Wipes the nanoservices from the Cargo.toml dependencies.
///
/// # Arguments
/// * `cargo_toml_file` - A mutable reference to the CargoToml struct.
///
/// # Returns
/// None
fn wipe_nanoservices(cargo_toml_file: &mut RawCargoToml) {
    println!("Wiping nanoservices");
    for (name, value) in cargo_toml_file.dependencies.clone() {
        match value {
            Value::Table(table) => {
                match table.get("path") {
                    Some(path) => {
                        let path = path.as_str().unwrap();
                        if path.contains(".nanoservices_cache") {
                            println!("Removing nanoservice: {}", name);
                            cargo_toml_file.dependencies.remove(&name);
                        }
                    },
                    None => ()
                }
            },
            _ => ()
        }
    }
}


/// Configures a Cargo.toml file with the nanoservices relative path and adds this relative path
/// to the dependencies section so the nanoservice can be built into the project.
///
/// # Arguments
/// * `path` - The path to the Cargo.toml file to configure.
/// * `nanos` - A vector of tuples containing the name of the nanoservice and the Nanoservice struct.
///
/// # Returns
/// None
pub fn config_cargo(path: PathBuf, nanos:  Vec<(String, Nanoservice)>) -> Result<(), NanoServiceError> {
    // exit the function early if the cargo.toml is not valid
    let mut cargo_toml = match read::read_toml(path.to_str().unwrap())?.into_raw() {
        Some(raw_dog) => raw_dog,
        None => return Err(
            NanoServiceError::new(
                "Failed to read the Cargo.toml file when configuring the Cargo.toml file".to_string(),
                NanoServiceErrorStatus::Unknown
            )
        )
    };
    // loop through nanos and add them to the dependencies section as tables
    for (name, nanoservice) in nanos {
        let mut nanoservice_table = Table::new();
        let relative_path = safe_eject!(
            finder::calculate_relative_path(
                &path,
                nanoservice.dev_image,
                nanoservice.entrypoint
            ),
            NanoServiceErrorStatus::Unknown,
            "Failed to calculate the relative path when configuring the Cargo.toml file"
        )?;
        let relative_path_str = match relative_path.to_str() {
            Some(v) => v,
            None => {
                return Err(
                    NanoServiceError::new(
                        "Failed to convert the relative path to a string when configuring the Cargo.toml file".to_string(),
                        NanoServiceErrorStatus::Unknown
                    )
                )
            }
        }.to_string();
        nanoservice_table.insert(
            "path".to_string(),
            Value::String(relative_path_str)
        );
        // add the features to the table if they exist
        match nanoservice.features {
            Some(features) => {
                nanoservice_table.insert(
                    "features".to_string(),
                    Value::Array(
                        features.iter().map(|f|
                                Value::String(f.to_string())
                        ).collect()
                    )
                );
            },
            None => ()
        }
        cargo_toml.dependencies.insert(name, toml::Value::Table(nanoservice_table));
    }
    read::write_toml(path.to_str().unwrap(), cargo_toml)
}


#[cfg(test)]
mod tests {

    use super::*;
    use crate::toml_operations::read::Package;

    #[test]
    fn test_wipe_nanoservices() {

        let mut dependencies = HashMap::new();
        dependencies.insert("test".to_string(), Value::Table(Table::new()));
        dependencies.insert("test2".to_string(), Value::Table(Table::new()));

        let mut table = Table::new();
        table.insert("path".to_string(), Value::String("test".to_string()));
        dependencies.insert("test3".to_string(), Value::Table(table));

        let mut nanoservice_table = Table::new();
        nanoservice_table.insert("path".to_string(), Value::String(
            ".nanoservices_cache/one/two".to_string())
        );
        dependencies.insert("test4".to_string(), Value::Table(nanoservice_table));

        let mut cargo_toml = RawCargoToml {
            package: Package {
                name: "test".to_string(),
                version: "0.1.0".to_string(),
                edition: "2018".to_string()
            },
            dependencies,
            nanoservices: None
        };
        wipe_nanoservices(&mut cargo_toml);

        assert_eq!(cargo_toml.dependencies.len(), 3);
        assert_eq!(cargo_toml.dependencies.contains_key("test"), true);
        assert_eq!(cargo_toml.dependencies.contains_key("test2"), true);
        assert_eq!(cargo_toml.dependencies.contains_key("test3"), true);
    }

}