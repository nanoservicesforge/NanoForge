//! Defines the functionality around the reading and writing of TOML files.
use serde::{Deserialize, Serialize};
use toml::Value;
use std::collections::HashMap;
use std::fs;


/// Represents the structure of a Cargo.toml file.
///
/// # Fields
/// * `package` - The package section of the Cargo.toml file.
/// * `dependencies` - The dependencies section of the Cargo.toml file.
/// * `nanoservices` - The nanoservices section of the Cargo.toml file.
#[derive(Debug, Deserialize, Serialize)]
pub struct CargoToml {
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
pub fn read_toml(cargo_toml_path: &str) -> CargoToml {
    let cargo_toml_contents = fs::read_to_string(cargo_toml_path)
        .expect("Failed to read Cargo.toml");
    let cargo_toml: CargoToml = toml::from_str(&cargo_toml_contents)
        .expect("Failed to parse Cargo.toml");
    return cargo_toml
}


/// Writes a CargoToml struct to a Cargo.toml file.
///
/// # Arguments
/// * `cargo_toml_path` - The path to the Cargo.toml file.
/// * `cargo_toml` - The CargoToml struct to write to the Cargo.toml file.
///
/// # Returns
/// None
pub fn write_toml(cargo_toml_path: &str, cargo_toml: CargoToml) {
    let modified_toml = toml::to_string(&cargo_toml).expect("Failed to serialize Cargo.toml");
    fs::write(cargo_toml_path, modified_toml).expect("Failed to write Cargo.toml");
}
