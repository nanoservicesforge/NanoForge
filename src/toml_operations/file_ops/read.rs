//! Defines the functionality around reading TOML files.
use std::fs;
use nanoservices_utils::{
    safe_eject,
    errors::{
        NanoServiceError,
        NanoServiceErrorStatus
    }
};
use crate::toml_operations::kernel::CargoToml;


/// Reads a Cargo.toml file and returns the parsed CargoToml struct.
///
/// # Arguments
/// * `cargo_toml_path` - The path to the Cargo.toml file.
///
/// # Returns
/// A CargoToml struct representing the parsed Cargo.toml file.
pub fn read_toml(cargo_toml_path: &str) -> Result<CargoToml, NanoServiceError> {
    let cargo_toml_contents = safe_eject!(
        fs::read_to_string(cargo_toml_path),
        NanoServiceErrorStatus::Unknown,
        format!("Failed to read Cargo.toml: {}", cargo_toml_path)
    )?;
    let cargo_toml: CargoToml = safe_eject!(
        toml::from_str(&cargo_toml_contents),
        NanoServiceErrorStatus::Unknown,
        format!("Failed to parse Cargo.toml: {}", cargo_toml_path)
    )?;
    Ok(cargo_toml)
}


#[cfg(test)]
mod tests {

    use super::*;
    use crate::toml_operations::kernel::Package;

    #[test]
    fn test_read_success_package() {

        let expected_package = Package {
            version: "0.1.0".to_string(),
            name: "testing_two".to_string(),
            edition: "2021".to_string()
        };

        let cargo_data = read_toml("./tests/assets/Cargo.toml").unwrap();

        let read_package = cargo_data.package.unwrap();

        assert_eq!(
            &expected_package,
            &read_package
        );

    }

    #[test]
    fn test_read_success_dependencies() {
        let cargo_data = read_toml("./tests/assets/Cargo.toml").unwrap();
        let read_deps = cargo_data.dependencies.unwrap();

        assert_eq!(
            "1.0.0".to_string(),
            read_deps.get("tokio").unwrap().as_str().unwrap()
        );
    }

    #[test]
    fn test_read_success_nanoservices() {
        let cargo_data = read_toml("./tests/assets/Cargo.toml").unwrap();
        let read_nanoservices = cargo_data.nanoservices.unwrap();

        let nan_one = read_nanoservices.get("nan-one").unwrap();
        let nan_two = read_nanoservices.get("nan-two").unwrap();
        println!("{:?}", nan_one);

        // testing nan-one
        assert_eq!(
            "maxwellflitton/nan-one".to_string(),
            nan_one.dev_image
        );
        assert_eq!(
            "maxwellflitton/nan-one".to_string(),
            nan_one.prod_image
        );
        assert_eq!(
            ".".to_string(),
            nan_one.entrypoint
        );
        assert_eq!(
            None,
            nan_one.features
        );
        assert_eq!(
            None,
            nan_one.local
        );

        // testing nan-two
        assert_eq!(
            "maxwellflitton/nan-two".to_string(),
            nan_two.dev_image
        );
        assert_eq!(
            "maxwellflitton/nan-two".to_string(),
            nan_two.prod_image
        );
        assert_eq!(
            ".".to_string(),
            nan_two.entrypoint
        );
        assert_eq!(
            vec!["one".to_string(), "two".to_string()],
            nan_two.features.clone().unwrap()
        );
        assert_eq!(
            true,
            nan_two.local.unwrap()
        );
    }

    #[test]
    fn test_read_fail_corrupt() {

        match read_toml("./tests/assets/corrupt.toml") {
            Ok(_) => {
                // should fail
                assert_eq!(1, 2);
            },
            Err(error) => {
                let expected_message = "Failed to parse Cargo.toml: ./tests/assets/corrupt.toml: TOML parse error at line 1, column 21\n  |\n1 | sehiwofghirhngpwhfon\n  |                     ^\nexpected `.`, `=`\n";
                assert_eq!(
                    expected_message,
                    error.message
                );
                println!("{:?}", error.message);
            }
        };

    }

    #[test]
    fn test_read_nothing() {
        match read_toml("./tests/assets/nothing.toml") {
            Ok(data) => {
                assert_eq!(
                    data.dependencies,
                    None
                );
                assert_eq!(
                    data.nanoservices,
                    None
                );
                assert_eq!(
                    data.package,
                    None
                );
            },
            Err(_) => {
                // should fail
                assert_eq!(1, 2);
            }
        }
    }

    #[test]
    fn test_read_fail_missing() {
        match read_toml("./tests/assets/not_here.toml") {
            Ok(_) => {
                assert_eq!(1, 2);
            },
            Err(error) => {
                let expected_message = "Failed to read Cargo.toml: ./tests/assets/not_here.toml: No such file or directory (os error 2)";
                assert_eq!(
                    expected_message,
                    error.message
                )
            }
        }
    }

}
