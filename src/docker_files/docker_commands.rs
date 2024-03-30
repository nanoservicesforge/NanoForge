use std::process::Command;
use tar::Archive;
use std::fs::File;


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


pub fn save_docker_image(image_name: &str, tar_path: &str) -> std::io::Result<String> {
    pull_docker_image(image_name)?; // Ensure the image is pulled before saving it

    let tar_path = std::path::Path::new(tar_path);
    let tar_file = image_name;
    let tar_file = tar_file.replace("/", "_").replace(":", "_");
    let unpack_tar_path = tar_path.join(format!("{}.tar", tar_file));
    let package_path = tar_path.join(tar_file);

    // create tar path if it doesn't exist
    // if !tar_path.exists() {
    //     std::fs::create_dir_all(&tar_path)?;
    // }
    println!("Tar path: {:?}", tar_path);

    let _ = Command::new("docker")
        .args(["save", "-o", unpack_tar_path.to_str().unwrap(), image_name])
        .status()?;
    let tar_file = File::open(unpack_tar_path)?;
    let mut archive = Archive::new(tar_file);
    archive.unpack(&package_path).unwrap();
    Ok(package_path.to_str().unwrap().to_string())
}