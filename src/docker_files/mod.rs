//! Defines the actions around downloading and unpacking docker images to access the files.
pub mod unpacking;
pub mod cache;
pub mod docker_commands;


/// Downloads a docker image and unpacks it to the nanoservices cache directory.
///
/// # Arguments
/// * `image` - A string slice that holds the name of the docker image to download.
///
/// # Returns
/// The paths to where the files have been unpacked to from the docker image
pub fn download_nanoservice(image: &str) -> std::io::Result<String> {
    let image_file = image.replace("/", "_").replace(":", "_");
    let main_path = docker_commands::save_docker_image(
        image,
        cache::CACHE_NANOSERVICES_TAR_DIR.to_str().unwrap(),
    )?;
    let unpack_path = cache::CACHE_NANOSERVICES_DIR.join(
        image_file.as_str()
    );
    let final_path = unpacking::extract_layers(
        main_path.as_str(),
        unpack_path.to_str().unwrap(),
    )?;
    Ok(final_path)
}
