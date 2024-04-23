//! Graphs the nanoservices in the current workspace
use std::collections::HashMap;
use crate::toml_operations::nanoservices::get_all::get_all_nanoservices;
use crate::toml_operations::file_ops::find_all_cargos::find_all_cargos_interface;
use nanoservices_utils::errors::NanoServiceError;
use petgraph::{Graph, Directed};
use petgraph::dot::{Dot, Config};
use graphviz_rust::{
    cmd::Format,
    exec, parse,
    printer::PrinterContext,
};


/// Goes through all the nanoservices in the current workspace and graphs them to see their
/// dependencies.
/// 
/// # Notes
/// Writes the outcome to a file called `nanoserve_dep_graph.png`
pub fn graph_nanos() -> Result<(), NanoServiceError>{
    let all_cargo_paths = find_all_cargos_interface(true)?;
    let (cargo_dependencies, _) = get_all_nanoservices(all_cargo_paths)?;

    let mut graph_deps: HashMap<String, Vec<String>> = HashMap::new();

    for (cargo_path, nanos) in cargo_dependencies {
        let mut nanoservices = Vec::new();
        for (name, _) in nanos {
            nanoservices.push(name.replace(".nanoservices_cache/domain_services/nanoservices/", "nanoservice:"));
        }
        graph_deps.insert(
            cargo_path.to_str().unwrap().to_string().replace(".nanoservices_cache/domain_services/nanoservices/", "nanoservice:"), 
            nanoservices
        );
    }

    // Create a directed graph
    let mut graph = Graph::<&str, (), Directed>::new();

    // This HashMap will help in quickly finding node indices
    let mut node_indices = HashMap::new();

    // Insert all nodes into the graph and save their indices
    for (key, deps) in &graph_deps {
        // .replace(".nanoservices_cache/domain")
        let index = graph.add_node(key);
        node_indices.insert(key, index);

        for dep in deps {
            if !node_indices.contains_key(dep) {
                let index = graph.add_node(dep);
                node_indices.insert(dep, index);
            }
        }
    }

    // Now, add edges according to dependencies
    for (key, deps) in &graph_deps {
        let to_index = node_indices[&key];
        for dep in deps {
            let from_index = node_indices[&dep];
            graph.add_edge(from_index, to_index, ());
        }
    }

    // Convert to DOT format
    let dot_output = format!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

    let g = parse(&dot_output).unwrap();

    let graph_svg = exec(
        g,
        &mut PrinterContext::default(),
        vec![Format::Png.into()],
    )
    .unwrap();

    // write the graph to a file
    std::fs::write("./nanoserve_dep_graph.png", graph_svg).expect("Failed to write graph to file");
    println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    Ok(())
}
