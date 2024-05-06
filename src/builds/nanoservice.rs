//! Builds a new nanoservice template in the current directory.
use nanoservices_utils::{
    safe_eject,
    errors::{
        NanoServiceError,
        NanoServiceErrorStatus
    }
};
use std::{
    env,
    fs,
    process::Command
};


/// Creates a new nanoservice in the current directory by cloning the nanoservice template into a named directory.
/// 
/// # Arguments
/// * `name` - The name of the nanoservice to create.
/// 
/// # Returns
/// None
pub fn create_new_nanoservice(name: String) -> Result<(), NanoServiceError> {
    println!("Creating new nanoservice: {}", name);
    // get the current directory
    let current_dir = match env::current_dir() {
        Ok(v) => v,
        Err(e) => {
            return Err(
                NanoServiceError::new(
                    format!("Failed to get the current directory: {}", e),
                    NanoServiceErrorStatus::Unknown
                )
            )
        }
    };
    // check to see if the nanoservice already exists
    let nanoservice_path = current_dir.join(&name);
    if nanoservice_path.exists() {
        return Err(
            NanoServiceError::new(
                format!("The nanoservice already exists: {}", name),
                NanoServiceErrorStatus::Unknown
            )
        )
    }
    let status = safe_eject!(Command::new("git")
        .args([
            "clone".to_string(), 
            "https://github.com/nanoservicesforge/nanoservice-template.git".to_string(),
            nanoservice_path.to_str().unwrap().to_string()
        ])
        .status(),
        NanoServiceErrorStatus::Unknown,
        "Failed to run pull Docker image command in NanoForge"
    )?;
    if status.success() {
        println!("Successfully cloned the nanoservice: {}", &name);
    } else {
        return Err(
            NanoServiceError::new(
                format!("Failed to create the nanoservice: {}", name),
                NanoServiceErrorStatus::Unknown
            )
        )
    }
    let git_directory = nanoservice_path.join(".git");
    let _ = fs::remove_dir_all(git_directory).unwrap();
    Ok(())
}
