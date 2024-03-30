mod toml_operations;
mod docker_files;


fn main() {
    println!("Hello, nanoservices!! .... That's so dingo!");
    // get the first argument passed to the program
    let args: Vec<String> = std::env::args().collect();
    let command = &args[1];

    if command == "prep" {
        docker_files::cache::wipe_and_create_cache();
        let (cargo_dependencies, all_nanoservices) = toml_operations::get_all_nanoservices();

        // download all the nanoservices from docker
        for (_name, nanoservice) in all_nanoservices {
            let _path = docker_files::download_nanoservice(&nanoservice.dev_image).unwrap();
        }

        for (path, nanoservices) in cargo_dependencies {
            toml_operations::config_cargo(path, nanoservices);
        }
    } else if command == "pull" {
        let image = &args[2];
        let _path = docker_files::download_nanoservice(image).unwrap();
    }
    else {
        println!("Command not found");
    }
}
