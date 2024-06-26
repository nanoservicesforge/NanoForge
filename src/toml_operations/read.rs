//! Defines the functionality around the reading and writing of TOML files.
use std::fs;
use nanoservices_utils::{
    safe_eject,
    errors::{
        NanoServiceError,
        NanoServiceErrorStatus
    }
};
use crate::toml_operations::kernel::{
    CargoToml, 
    RawCargoToml
};


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
