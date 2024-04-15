use toml::Value;
use crate::toml_operations::kernel::RawCargoToml;


/// Wipes the nanoservices from the Cargo.toml dependencies.
///
/// # Arguments
/// * `cargo_toml_file` - A mutable reference to the CargoToml struct.
///
/// # Returns
/// None
pub fn wipe_nanoservices(cargo_toml_file: &mut RawCargoToml) {
    println!("Wiping nanoservices");
    for (name, value) in cargo_toml_file.dependencies.clone() {
        match value {
            Value::Table(table) => {
                match table.get("path") {
                    Some(path) => {
                        let path = path.as_str().unwrap();
                        if path.contains(".nanoservices_cache") {
                            println!("Removing nanoservice: {}", name);
                            cargo_toml_file.dependencies.remove(&name);
                        }
                    },
                    None => ()
                }
            },
            _ => ()
        }
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    use crate::toml_operations::kernel::Package;
    use std::collections::HashMap;
    use toml::Table;

    #[test]
    fn test_wipe_nanoservices() {

        let mut dependencies = HashMap::new();
        dependencies.insert("test".to_string(), Value::Table(Table::new()));
        dependencies.insert("test2".to_string(), Value::Table(Table::new()));

        let mut table = Table::new();
        table.insert("path".to_string(), Value::String("test".to_string()));
        dependencies.insert("test3".to_string(), Value::Table(table));

        let mut nanoservice_table = Table::new();
        nanoservice_table.insert("path".to_string(), Value::String(
            ".nanoservices_cache/one/two".to_string())
        );
        dependencies.insert("test4".to_string(), Value::Table(nanoservice_table));

        let mut cargo_toml = RawCargoToml {
            package: Package {
                name: "test".to_string(),
                version: "0.1.0".to_string(),
                edition: "2018".to_string()
            },
            dependencies,
            nanoservices: None
        };
        wipe_nanoservices(&mut cargo_toml);

        assert_eq!(cargo_toml.dependencies.len(), 3);
        assert_eq!(cargo_toml.dependencies.contains_key("test"), true);
        assert_eq!(cargo_toml.dependencies.contains_key("test2"), true);
        assert_eq!(cargo_toml.dependencies.contains_key("test3"), true);
    }

}
