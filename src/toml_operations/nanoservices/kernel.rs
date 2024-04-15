use serde::{Deserialize, Serialize};


/// Represents the structure of a nanoservice in a Cargo.toml file.
///
/// # Fields
/// * `dev_image` - The development Docker image of the nanoservice (to pull in dev environments).
/// * `prod_image` - The production Docker image of the nanoservice (to pull in prod environments).
/// * `entrypoint` - The entrypoint of the nanoservice (where the terminal has to point inside for the build).
/// * `features` - The enabled features of the nanoservice (optional).
/// * `local` - A flag to indicate if the nanoservice image is local and should not be pulled (optional).
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct Nanoservice {
    pub dev_image: String,
    pub prod_image: String,
    pub entrypoint: String,
    pub features: Option<Vec<String>>,
    pub local: Option<bool>,
}
