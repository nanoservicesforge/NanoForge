//! # NanoForge
//! This is the Rust interface for the NanoForge project. NanoForge is a tool for packaging
//! nanoservices into Docker images, and then configuring a Rust project to use these nanoservices
//! as dependencies. The end result is that multiple servers and even JavaScript apps can be
//! compiled into one Rust binary.
#[allow(dead_code)]
pub mod docker_files;
#[allow(dead_code)]
pub mod toml_operations;
