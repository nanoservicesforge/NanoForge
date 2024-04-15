//! Defines the actions around downloading and unpacking docker images to access the files.
use std::process::Command;
use tar::Archive;
use std::fs::File;
use nanoservices_utils::{
    safe_eject,
    errors::{
        NanoServiceError,
        NanoServiceErrorStatus
    }
};
use super::cache::process_image_name;


/// Pulls a docker image from the docker registry.
///
/// # Arguments
/// * `image_name` - A string slice that holds the name of the docker image to pull.
///
/// # Returns
/// None
pub fn pull_docker_image(image_name: &str) -> Result<(), NanoServiceError> {
    let status = safe_eject!(Command::new("docker")
        .args(["pull", image_name])
        .status(),
        NanoServiceErrorStatus::Unknown,
        "Failed to run pull Docker image command in NanoForge"
    )?;

    if status.success() {
        Ok(())
    } else {
        Err(NanoServiceError::new(
            "Failed to pull Docker image in NanoForge".to_string(),
            NanoServiceErrorStatus::Unknown
            )
        )
    }
}


/// Extracts the Tar file from the Docker image, and saves it to the specified path.
///
/// # Notes
/// The pulling of the Docker image is also handled in this function.
///
/// # Arguments
/// * `image_name` - The name of the Docker image to pull and unpack.
/// * `tar_path` - The path to save the unpacked Docker image.
///
/// # Returns
/// The path to where the compressed Docker image files are stored
pub fn save_docker_image(image_name: &str, tar_path: &str) -> Result<String, NanoServiceError> {
    pull_docker_image(image_name)?; // Ensure the image is pulled before saving it

    let tar_path = std::path::Path::new(tar_path);
    let tar_file = image_name;
    let tar_file = process_image_name(&tar_file.to_string());

    let binding = tar_path.join(format!("{}.tar", tar_file));
    let unpack_tar_path = match binding.to_str() {
        Some(v) => v,
        None => {
            return Err(NanoServiceError::new(
                "Failed to convert path to string in NanoForge".to_string(),
                NanoServiceErrorStatus::Unknown
                )
            )
        }
    };
    let package_path = tar_path.join(tar_file);

    println!("Tar path: {:?}", tar_path);

    let _ = safe_eject!(Command::new("docker")
        .args(["save", "-o", unpack_tar_path, image_name])
        .status(),
        NanoServiceErrorStatus::Unknown,
        "Failed to run save Docker image command in NanoForge"
    )?;
    let tar_file = safe_eject!(
        File::open(unpack_tar_path),
        NanoServiceErrorStatus::Unknown,
        "Failed to open Docker image in NanoForge"
    )?;
    let mut archive = Archive::new(tar_file);

    safe_eject!(
        archive.unpack(&package_path),
        NanoServiceErrorStatus::Unknown,
        "Failed to unpack Docker image in NanoForge"
    )?;

    // return statement
    Ok(match package_path.to_str() {
        Some(v) => v.to_string(),
        None => {
            return Err(NanoServiceError::new(
                "Failed to convert path to string in NanoForge".to_string(),
                NanoServiceErrorStatus::Unknown
                )
            )
        }
    })
}
