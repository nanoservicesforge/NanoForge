use crate::docker_files::{
    download_nanoservice,
    cache::{
        wipe_and_create_cache,
        CACHE_NANOSERVICES_DIR
    }
};
use crate::toml_operations::nanoservices::get_all::get_all_nanoservices;
use crate::toml_operations::nanoservices::configure_cargo_toml::config_cargo;
use crate::toml_operations::file_ops::{
    read::read_toml,
    write::write_toml,
    find_all_cargos::find_all_cargos
};

use nanoservices_utils::errors::{NanoServiceError, NanoServiceErrorStatus};
use nanoservices_utils::safe_eject;


/// Wipes the cache, creates a new cache, and downloads all the images, unpacking them
/// and configuring the `Cargo.toml` files that are linking them.
pub fn prep_nanoservices_once() -> Result<(), NanoServiceError> {
    wipe_and_create_cache();

    let all_cargo_paths = safe_eject!(
        find_all_cargos(std::env::current_dir().unwrap()),
        NanoServiceErrorStatus::Unknown,
        "getting all cargo paths in prep_nanoservices_once"
    )?;
    println!("Cargo paths found: {:?}", all_cargo_paths);
    let (cargo_dependencies, all_nanoservices) = get_all_nanoservices(all_cargo_paths)?;

    // download all the nanoservices from docker
    for (_name, nanoservice) in all_nanoservices {
        // bypass downloading the image if local is set to true
        let local = match nanoservice.local {
            Some(v) => v,
            _ => false,
        };
        if !local {
            let _path = download_nanoservice(&nanoservice.dev_image)?;
        }
    }

    for (path, nanoservices) in cargo_dependencies {
        // we can unwrap the `into_raw()` function because the cargo.toml will not be here if it did not have
        // nanoservices in the contents
        let raw_dog_cargo = read_toml(path.to_str().unwrap())?.into_raw().unwrap();
        let raw_dog_cargo = config_cargo(
            raw_dog_cargo, 
            nanoservices, 
            CACHE_NANOSERVICES_DIR.clone(),
            path.clone()
        )?;
        write_toml(path.to_str().unwrap(), raw_dog_cargo)?
    }
    Ok(())
}