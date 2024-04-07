//! Defines the functionality around the reading and writing of TOML files.
use serde::{Deserialize, Serialize};
use toml::Value;
use std::collections::HashMap;
use std::fs;
use nanoservices_utils::{
    safe_eject,
    errors::{
        NanoServiceError,
        NanoServiceErrorStatus
    }
};


/// Represents the structure of a Cargo.toml file for file loading.
/// This is needed when reading Cargo.toml files such as workspaces that
/// do no have packages or dependencies.
///
/// # Fields
/// * `package` - The package section of the Cargo.toml file.
/// * `dependencies` - The dependencies section of the Cargo.toml file.
/// * `nanoservices` - The nanoservices section of the Cargo.toml file.
#[derive(Debug, Deserialize, Serialize)]
pub struct CargoToml {
    pub package: Option<Package>,
    pub dependencies: Option<HashMap<String, Value>>,
    pub nanoservices: Option<HashMap<String, Nanoservice>>,
}

impl CargoToml {

    /// If all the fields are Some, returns the RawCargoToml struct.
    ///
    /// # Returns
    /// * `Some(RawCargoToml)` - If all fields are Some.
    pub fn into_raw(self) -> Option<RawCargoToml> {
        let package = self.package?;
        let dependencies = self.dependencies?;
        let nanoservices = self.nanoservices;
        Some(RawCargoToml { package, dependencies, nanoservices })
    }

}


/// Represents the structure of a Cargo.toml file for file saving and manipulation
/// of the cargo file in relation to configuring of nanoservices and dependencies.
#[derive(Debug, Deserialize, Serialize)]
pub struct RawCargoToml {
    pub package: Package,
    pub dependencies: HashMap<String, Value>,
    pub nanoservices: Option<HashMap<String, Nanoservice>>,
}


/// Represents the structure of a package in a Cargo.toml file.
///
/// # Fields
/// * `name` - The name of the package.
/// * `version` - The version of the package.
/// * `edition` - The edition of the package.
#[derive(Debug, Deserialize, Serialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub edition: String,
}


/// Represents the structure of a nanoservice in a Cargo.toml file.
///
/// # Fields
/// * `dev_image` - The development Docker image of the nanoservice (to pull in dev environments).
/// * `prod_image` - The production Docker image of the nanoservice (to pull in prod environments).
/// * `entrypoint` - The entrypoint of the nanoservice (where the terminal has to point inside for the build).
/// * `features` - The enabled features of the nanoservice (optional).
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct Nanoservice {
    pub dev_image: String,
    pub prod_image: String,
    pub entrypoint: String,
    pub features: Option<Vec<String>>,
}


/// Reads a Cargo.toml file and returns the parsed CargoToml struct.
///
/// # Arguments
/// * `cargo_toml_path` - The path to the Cargo.toml file.
///
/// # Returns
/// A CargoToml struct representing the parsed Cargo.toml file.
pub fn read_toml(cargo_toml_path: &str) -> Result<CargoToml, NanoServiceError> {
    let cargo_toml_contents = safe_eject!(
        fs::read_to_string(cargo_toml_path),
        NanoServiceErrorStatus::Unknown,
        format!("Failed to read Cargo.toml: {}", cargo_toml_path)
    )?;
    let cargo_toml: CargoToml = safe_eject!(
        toml::from_str(&cargo_toml_contents),
        NanoServiceErrorStatus::Unknown,
        format!("Failed to parse Cargo.toml: {}", cargo_toml_path)
    )?;
    Ok(cargo_toml)
}


/// Writes a CargoToml struct to a Cargo.toml file.
///
/// # Arguments
/// * `cargo_toml_path` - The path to the Cargo.toml file.
/// * `cargo_toml` - The CargoToml struct to write to the Cargo.toml file.
///
/// # Returns
/// None
pub fn write_toml(cargo_toml_path: &str, cargo_toml: RawCargoToml) -> Result<(), NanoServiceError> {
    let modified_toml = safe_eject!(
        toml::to_string(&cargo_toml),
        NanoServiceErrorStatus::Unknown,
        format!("Failed to serialize Cargo.toml for writing: {}", cargo_toml_path)
    )?;
    safe_eject!(
        fs::write(cargo_toml_path, modified_toml),
        NanoServiceErrorStatus::Unknown,
        format!("Failed to write Cargo.toml: {}", cargo_toml_path)
    )?;
    Ok(())
}
