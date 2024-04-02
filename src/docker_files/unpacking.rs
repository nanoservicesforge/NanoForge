//! Defines the actions around unpacking compressed Docker files from the manifest.
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::Path;
use serde_json::Value;
use tar::Archive;
use flate2::read::GzDecoder;



/// Checks if the given file is gzipped by reading its magic number.
///
/// # Arguments
///
/// * `file` - A mutable reference to the file to check.
///
/// # Returns
///
/// * `Result<bool, io::Error>` - `Ok(true)` if the file is gzipped, `Ok(false)` otherwise, or an `Err` on failure.
///
fn check_if_gzipped(file: &mut File) -> io::Result<bool> {
    let mut magic_number = [0; 2];

    // Seek to the start of the file and read the first two bytes
    file.seek(SeekFrom::Start(0))?;
    file.read_exact(&mut magic_number)?;

    // Seek back to the start for subsequent operations on the file
    file.seek(SeekFrom::Start(0))?;

    // Check if the magic number matches gzip's 1F 8B
    Ok(magic_number == [0x1f, 0x8b])
}


/// Reads a JSON file.
///
/// # Notes
/// This function is mainly used for reading the manifest.json file in the unpacked Docker image
/// directory so we can extract the layers.
///
/// # Arguments
/// * `path` - Path to the JSON file being read.
fn read_json_file<P: AsRef<Path>>(path: P) -> serde_json::Result<Value> {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    serde_json::from_str(&contents)
}


/// Decompresses the layers from the Docker image and extracts them to a directory.
///
/// # Arguments
/// * `main_path` - The path to the compressed extracted layers from the Docker image.
/// * `unpack_path` - The path to where the layers will be extracted.
///
/// # Returns
/// The path to where the layers are extracted.
pub fn extract_layers(main_path: &str, unpack_path: &str) -> std::io::Result<String> {

    let manifest_path = std::path::Path::new(main_path).join("manifest.json");
    let blobs_dir = std::path::Path::new(main_path);
    let unpack_path = std::path::Path::new(unpack_path);

    if !unpack_path.exists() {
        std::fs::create_dir_all(&unpack_path)?;
    }

    let manifest = read_json_file(manifest_path).expect("Failed to read manifest.json");

    if let Some(layers) = manifest[0]["Layers"].as_array() {
        println!("Found {} layers in manifest", layers.len());
        for layer in layers {
            println!("Extracting layer: {}", layer);
            let base_path = blobs_dir;
            let layer_path = base_path.join(layer.as_str().unwrap());

            // Extract the layer's tarball to a directory
            let mut tar_file = File::open(layer_path)?;

            match check_if_gzipped(&mut tar_file)? {
                true => {
                    println!("Layer is gzipped");
                    let decompressed = GzDecoder::new(tar_file);
                    let mut archive = Archive::new(decompressed);
                    archive.unpack(unpack_path).unwrap();
                },
                false => {
                    println!("Layer is not gzipped");
                    let mut archive = Archive::new(tar_file);
                    archive.unpack(unpack_path).unwrap();
                }
            }

            // let archive = if is_gzipped {
            //     // If the file is gzipped, create a GzDecoder
            //     let decompressed = GzDecoder::new(tar_file);
            //     Archive::new(decompressed)
            // } else {
            //     // If the file is not gzipped, use it directly
            //     Archive::new(tar_file)
            // };
            //
            // // Create a GzDecoder to handle the gzip decompression
            // let decompressed = GzDecoder::new(tar_file);
            // let mut archive = Archive::new(decompressed);
            // archive.unpack(unpack_path).unwrap();
        }
    }
    else {
        println!("No layers found in manifest");
    }

    Ok(unpack_path.to_str().unwrap().to_string())
}