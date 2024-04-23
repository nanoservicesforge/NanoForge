mod toml_operations;
mod docker_files;

use nanoservices_utils::errors::{
    NanoServiceError,
    NanoServiceErrorStatus
};
use toml_operations::nanoservices::processes::{
    prep::recursive_prep_nanoservices,
    graph::graph_nanos
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
    if command == "prep" {
        println!("prepping nanos");
        recursive_prep_nanoservices()?;
    }
    else if command == "graph" {
        println!("graphing nanos");
        graph_nanos()?;
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
    else {
        println!("Command not found");
    }
    Ok(())
}
