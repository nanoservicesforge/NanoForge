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
                &nanoservice.dev_image,
                &nanoservice.entrypoint,
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
        // add optional features
        nanoservice.add_features(&mut nanoservice_table);
        nanoservice.add_package(&mut nanoservice_table);
        
        // insert the contructed nanoervice table into the dependencies section of the Cargo.toml
        cargo_toml.dependencies.insert(name, toml::Value::Table(nanoservice_table));

        // insert the kernel of the nanoservice into the dependencies section of the Cargo.toml if exists
        let nanoservice_kernel = match nanoservice.construct_kernel(&cargo_toml_path, &nanoservices_path)? {
            Some(kernel) => kernel,
            None => continue
        };
        // can directly unwrap here as the kernel is guaranteed to exist otherwise the `nanoservice_kernel` would not have
        // been constructed
        let name = nanoservice.kernel.unwrap().name.clone();
        cargo_toml.dependencies.insert(name, toml::Value::Table(nanoservice_kernel));

    }
    Ok(cargo_toml)
}
