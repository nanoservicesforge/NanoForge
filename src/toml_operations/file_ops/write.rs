//! Defines the functionality around the reading and writing of TOML files.
use std::fs;
use nanoservices_utils::{
    safe_eject,
    errors::{
        NanoServiceError,
        NanoServiceErrorStatus
    }
};
use crate::toml_operations::kernel::RawCargoToml;


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