//! Defines all the paths to cache directories and also handles the wiping of cache.
use std::env;
use std::path::PathBuf;
use lazy_static::lazy_static;


/// Processes the image name into a directory name.
/// 
/// # Arguments
/// * `image_name` - the name of the image to be processed
/// 
/// # Returns
/// * the converted image name string that can be used in a directory path.
pub fn process_image_name(image_name: &String) -> String {
    image_name.replace(":latest", "").replace("/", "_").replace(":", "_")
}


lazy_static! {
    // The current working directory of the terminal calling the program.
    pub static ref CURRENT_WORKING_DIR: PathBuf = {
        env::current_dir().expect("Failed to determine current working directory")
    };
}


lazy_static! {
    // The patient to the cache directory (please put this in your .gitignore file).
    pub static ref CACHE_DIR: PathBuf = {
        let mut path = CURRENT_WORKING_DIR.clone();
        path.push(".nanoservices_cache");
        path
    };
}


lazy_static! {
    // The path to the domain services cache directory.
    pub static ref CACHE_DOMAIN_SERVICES_DIR: PathBuf = {
        let mut path = CACHE_DIR.clone();
        path.push("domain_services");
        path
    };
}


lazy_static! {
    // The path to the domain services tar cache directory.
    pub static ref CACHE_DOMAIN_SERVICES_TAR_DIR: PathBuf = {
        let mut path = CACHE_DIR.clone();
        path.push("domain_services_tar");
        path
    };
}


lazy_static! {
    // The path to the nanoservices cache directory.
    pub static ref CACHE_NANOSERVICES_DIR: PathBuf = {
        let mut path = CACHE_DOMAIN_SERVICES_DIR.clone();
        path.push("nanoservices");
        path
    };
}


lazy_static! {
    // The path to the nanoservices tar cache directory.
    pub static ref CACHE_NANOSERVICES_TAR_DIR: PathBuf = {
        let mut path = CACHE_DOMAIN_SERVICES_TAR_DIR.clone();
        path.push("nanoservices_tar");
        path
    };
}


/// Wipes the cache directory and creates a new cache directory.
///
/// # Returns
/// None
pub fn wipe_and_create_cache() {
    if CACHE_DIR.exists() {
        std::fs::remove_dir_all(&*CACHE_DIR).expect(
            "Failed to remove cache directory"
        );
    }
    std::fs::create_dir_all(&*CACHE_NANOSERVICES_DIR).expect(
        "Failed to create nanoservices cache directory"
    );
    std::fs::create_dir_all(&*CACHE_NANOSERVICES_TAR_DIR).expect(
        "Failed to create nanoservices tar cache directory"
    );
}
