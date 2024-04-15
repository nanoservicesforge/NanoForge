use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use toml::Value;
use crate::toml_operations::nanoservices::kernel::Nanoservice;


/// Represents the structure of a package in a Cargo.toml file.
///
/// # Fields
/// * `name` - The name of the package.
/// * `version` - The version of the package.
/// * `edition` - The edition of the package.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub edition: String,
}


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
