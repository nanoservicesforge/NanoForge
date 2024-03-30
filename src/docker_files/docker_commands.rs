//! Defines the actions around downloading and unpacking docker images to access the files.
use std::process::Command;
use tar::Archive;
use std::fs::File;


/// Pulls a docker image from the docker registry.
///
/// # Arguments
/// * `image_name` - A string slice that holds the name of the docker image to pull.
///
/// # Returns
/// None
pub fn pull_docker_image(image_name: &str) -> std::io::Result<()> {
    let status = Command::new("docker")
        .args(["pull", image_name])
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to pull Docker image"))
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
pub fn save_docker_image(image_name: &str, tar_path: &str) -> std::io::Result<String> {
    pull_docker_image(image_name)?; // Ensure the image is pulled before saving it

    let tar_path = std::path::Path::new(tar_path);
    let tar_file = image_name;
    let tar_file = tar_file.replace("/", "_").replace(":", "_");
    let unpack_tar_path = tar_path.join(format!("{}.tar", tar_file));
    let package_path = tar_path.join(tar_file);

    println!("Tar path: {:?}", tar_path);

    let _ = Command::new("docker")
        .args(["save", "-o", unpack_tar_path.to_str().unwrap(), image_name])
        .status()?;
    let tar_file = File::open(unpack_tar_path)?;
    let mut archive = Archive::new(tar_file);
    archive.unpack(&package_path).unwrap();
    Ok(package_path.to_str().unwrap().to_string())
}
