mod toml_operations;
mod docker_files;
mod builds;

use nanoservices_utils::errors::{
    NanoServiceError,
    NanoServiceErrorStatus
};
use toml_operations::nanoservices::processes::{
    prep::recursive_prep_nanoservices,
    graph::graph_nanos,
    config::recursive_config_nanoservices,
    install::recurrsive_install_nanoservices
};


fn main() -> Result<(), NanoServiceError> {
    println!("Hello, nanoservices!! .... That's so dingo!");
    // get the first argument passed to the program
    let args: Vec<String> = std::env::args().collect();
    let command = match args.get(1) {
        Some(v) => v,
        _ => {
            return Err(NanoServiceError::new(
                "No command specified".to_string(),
                NanoServiceErrorStatus::Unknown
            ))
        }
    };
    // dependency management
    if command == "prep" {
        println!("prepping nanos");
        recursive_prep_nanoservices()?;
    }
    else if command == "graph" {
        println!("graphing nanos");
        graph_nanos()?;
    }
    else if command == "config" {
        println!("configuring nanos");
        recursive_config_nanoservices()?;
    }
    else if command == "install" {
        println!("installing nanos");
        recurrsive_install_nanoservices()?;
    }
    else if command == "pull" {
        let image = match args.get(2) {
            Some(v) => v,
            _ => {
                return Err(NanoServiceError::new(
                    "No image specified".to_string(),
                    NanoServiceErrorStatus::Unknown
                ))
            
            },
        };

        let _path = docker_files::download_nanoservice(image)?;
    }
    else if command == "new" {
        let name = match args.get(2) {
            Some(v) => v,
            _ => {
                return Err(NanoServiceError::new(
                    "No name specified".to_string(),
                    NanoServiceErrorStatus::Unknown
                ))
            }
        };
        builds::nanoservice::create_new_nanoservice(name.to_string())?;
    }
    else {
        println!("Command not found");
    }
    Ok(())
}
