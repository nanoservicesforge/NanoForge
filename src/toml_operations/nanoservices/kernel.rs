//! The data structs for nanoservices.
use serde::{Deserialize, Serialize};
use toml::{Value, map::Map, Table};
use nanoservices_utils::{
    safe_eject,
    errors::{
        NanoServiceError,
        NanoServiceErrorStatus
    }
};
use crate::toml_operations::file_ops::calculate_paths::calculate_relative_path;
use std::path::PathBuf;


/// Represents the structure of a nanoservice in a Cargo.toml file.
///
/// # Fields
/// * `dev_image` - The development Docker image of the nanoservice (to pull in dev environments).
/// * `prod_image` - The production Docker image of the nanoservice (to pull in prod environments).
/// * `entrypoint` - The entrypoint of the nanoservice (where the terminal has to point inside for the build).
/// * `features` - The enabled features of the nanoservice (optional).
/// * `local` - A flag to indicate if the nanoservice image is local and should not be pulled (optional).
/// * `package` - The package name of the nanoservice (optional). If set then the name of the nanoservice will be
///               the alias of the package name.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct Nanoservice {
    pub dev_image: String,
    pub prod_image: String,
    pub entrypoint: String,
    pub features: Option<Vec<String>>,
    pub local: Option<bool>,
    pub package: Option<String>,
    pub kernel: Option<NanoserviceKernel>,
}


// TODO => look into abstracting out the add_features and add_package functions into a trait
impl Nanoservice {

    /// Adds features to the nanoservice table if the features exist.
    /// 
    /// # Arguments
    /// * `nanoservice_table` - The table to add the features to.
    pub fn add_features(&self, nanoservice_table: &mut Map<String, Value>) {
        match &self.features {
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
    }

    /// Adds the package to the nanoservice table if the package exists.
    /// 
    /// # Arguments
    /// * `nanoservice_table` - The table to add the package to.
    pub fn add_package(&self, nanoservice_table: &mut Map<String, Value>) {
        match &self.package {
            Some(package) => {
                nanoservice_table.insert(
                    "package".to_string(),
                    Value::String(package.to_string())
                );
            },
            None => ()
        }
    }

    pub fn construct_kernel(
            &self,
            cargo_toml_path: &PathBuf,
            nanoservices_path: &PathBuf 
        ) -> Result<Option<Map<String, Value>>, NanoServiceError> {
        match &self.kernel {
            Some(kernel) => {
                // TODO => look into putting the path code below as a function into a utils file for the nanoservices
                let mut kernel_table = Table::new();
                let relative_path = safe_eject!(
                    calculate_relative_path(
                        &cargo_toml_path,
                        &self.dev_image,
                        &kernel.entrypoint,
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
                kernel_table.insert(
                    "path".to_string(),
                    Value::String(relative_path_str)
                );
                // add optional features
                kernel.add_features(&mut kernel_table);
                kernel.add_package(&mut kernel_table);
                return Ok(Some(kernel_table))
            },
            None => ()
        }
        return Ok(None)
    }

}


/// Represents the structure of a nanoservice kernel for a nanoservice in a Cargo.toml file.
/// 
/// # Fields
/// * `entrypoint` - The entrypoint of the nanoservice kernel (where the terminal has to point inside for the build).
/// * `features` - The enabled features of the nanoservice kernel (optional).
/// * `package` - The package name of the nanoservice kernel (optional). If set then the name of the nanoservice kernel will be
///               the alias of the package name.
/// * `name` - The name of the nanoservice kernel, if the package is not set then this will be the name of the nanoservice kernel.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct NanoserviceKernel {
    pub entrypoint: String,
    pub features: Option<Vec<String>>,
    pub package: Option<String>,
    pub name: String,
}


impl NanoserviceKernel {

    /// Adds features to the nanoservice table if the features exist.
    /// 
    /// # Arguments
    /// * `nanoservice_table` - The table to add the features to.
    pub fn add_features(&self, kernel_table: &mut Map<String, Value>) {
        match &self.features {
            Some(features) => {
                kernel_table.insert(
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
    }

    /// Adds the package to the nanoservice table if the package exists.
    /// 
    /// # Arguments
    /// * `nanoservice_table` - The table to add the package to.
    pub fn add_package(&self, kernel_table: &mut Map<String, Value>) {
        match &self.package {
            Some(package) => {
                kernel_table.insert(
                    "package".to_string(),
                    Value::String(package.to_string())
                );
            },
            None => ()
        }
    }

}