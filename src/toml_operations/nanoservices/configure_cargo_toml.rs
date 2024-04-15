use crate::toml_operations::nanoservices::kernel::Nanoservice;
use toml::{Table, Value};
use crate::toml_operations::kernel::RawCargoToml;
use nanoservices_utils::{
    safe_eject,
    errors::{
        NanoServiceError,
        NanoServiceErrorStatus
    }
};
use crate::toml_operations::file_ops::calculate_paths::calculate_relative_path;
use std::path::PathBuf;

// pub type CargoDependencies = HashMap<std::path::PathBuf, Vec<(String, Nanoservice)>>;
// pub type AllNanoservices = HashSet<(String, Nanoservice)>;


/// Configures a Cargo.toml file with the nanoservices relative path and adds this relative path
/// to the dependencies section so the nanoservice can be built into the project.
///
/// # Arguments
/// * `path` - The path to the Cargo.toml file to configure.
/// * `nanos` - A vector of tuples containing the name of the nanoservice and the Nanoservice struct.
///
/// # Returns
/// None
pub fn config_cargo(
        mut cargo_toml: RawCargoToml, 
        nanos:  Vec<(String, Nanoservice)>, 
        nanoservices_path: PathBuf,
        cargo_toml_path: PathBuf
    ) -> Result<RawCargoToml, NanoServiceError> {
    // loop through nanos and add them to the dependencies section as tables
    for (name, nanoservice) in nanos {
        let mut nanoservice_table = Table::new();
        let relative_path = safe_eject!(
            calculate_relative_path(
                &cargo_toml_path,
                nanoservice.dev_image,
                nanoservice.entrypoint,
                &nanoservices_path
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
    Ok(cargo_toml)
}
