use serde::{Deserialize, Serialize};
use toml::{Value, Table};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
pub struct CargoToml {
    pub package: Package,
    pub dependencies: HashMap<String, Value>,
    pub nanoservices: Option<HashMap<String, Nanoservice>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub edition: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct Nanoservice {
    pub dev_image: String,
    pub prod_image: String,
    pub entrypoint: String,
}


pub fn read_toml(cargo_toml_path: &str) -> CargoToml {
    let cargo_toml_contents = fs::read_to_string(cargo_toml_path)
        .expect("Failed to read Cargo.toml");
    let mut cargo_toml: CargoToml = toml::from_str(&cargo_toml_contents)
        .expect("Failed to parse Cargo.toml");
    return cargo_toml
}


pub fn write_toml(cargo_toml_path: &str, cargo_toml: CargoToml) {
    let modified_toml = toml::to_string(&cargo_toml).expect("Failed to serialize Cargo.toml");
    fs::write(cargo_toml_path, modified_toml).expect("Failed to write Cargo.toml");
}


