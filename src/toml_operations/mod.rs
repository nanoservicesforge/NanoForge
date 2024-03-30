pub mod read;
pub mod finder;
use std::collections::{
    HashMap,
    HashSet
};
use std::path::PathBuf;
use read::Nanoservice;
use toml::{Table, Value};


pub fn get_all_nanoservices()
    -> (HashMap<std::path::PathBuf, Vec<(String, Nanoservice)>>, HashSet<(String, Nanoservice)>) {
    let cargo_paths = finder::find_all_cargos().unwrap();
    let mut cargo_dependencies = HashMap::new();
    let mut all_nanoservices = HashSet::new();

    for cargo_path in cargo_paths {
        println!("cargo_path: {:?}", cargo_path);
        let cargo_toml = read::read_toml(cargo_path.to_str().unwrap());

        let mut buffer = Vec::new();

        let nanos = match cargo_toml.nanoservices {
            Some(nanos) => nanos,
            None => continue
        };

        for nanoservice in nanos {
            buffer.push(nanoservice.clone());
            all_nanoservices.insert(nanoservice);
        }
        if !buffer.is_empty() {
            cargo_dependencies.insert(cargo_path, buffer);
        }
    }

    return (cargo_dependencies, all_nanoservices)
}


pub fn config_cargo(path: PathBuf, nanos:  Vec<(String, Nanoservice)>) {
    let mut cargo_toml = read::read_toml(path.to_str().unwrap());

    // loop through nanos and add them to the dependencies section as tables
    for (name, nanoservice) in nanos {
        let mut nanoservice_table = Table::new();
        nanoservice_table.insert(
            "path".to_string(),
            Value::String(
                finder::calculate_relative_path(
                    &path,
                    nanoservice.dev_image,
                    nanoservice.entrypoint
                ).to_str().unwrap().to_string()
            )
        );
        cargo_toml.dependencies.insert(name, toml::Value::Table(nanoservice_table));
    }

    read::write_toml(path.to_str().unwrap(), cargo_toml);
}
