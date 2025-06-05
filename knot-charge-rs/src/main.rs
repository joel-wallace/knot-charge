// src/main.rs

// Declare the modules
mod protein_model;
mod pdb_parser;
mod knot_detection;
mod chain_model;

// Import necessary items
use std::env; // For command-line arguments
use protein_model::CaAtom; // Our custom CaAtom struct
use pdb_parser::parse_pdb_ca_atoms; // The PDB parsing function
use knot_detection::find_knots; // The placeholder knot detection function

fn main() {
    println!("Protein Knot Detector");

    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if a PDB file path is provided
    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_pdb_file>", args[0]);
        eprintln!("Error: PDB file path not provided.");
        std::process::exit(1); // Exit with an error code
    }

    let pdb_file_path = &args[1];
    println!("Attempting to parse PDB file: {}", pdb_file_path);

    // Parse the PDB file to get C-alpha atoms
    match parse_pdb_ca_atoms(pdb_file_path) {
        Ok(ca_atoms) => {
            println!("Successfully parsed {} C-alpha atoms.", ca_atoms.len());

            if ca_atoms.is_empty() {
                println!("No C-alpha atoms were found or extracted from the PDB file.");
            } else {
                // Optionally, print some information about the first few atoms
                // for verification.
                // For a large protein, printing all atoms can be overwhelming.
                // println!("First 5 C-alpha atoms (if available):");
                // for (i, atom) in ca_atoms.iter().take(5).enumerate() {
                //     println!(
                //         "  {}: Serial {}, Res: {}{}, Chain: {}, Coords: ({:.3}, {:.3}, {:.3})",
                //         i + 1,
                //         atom.serial,
                //         atom.res_name,
                //         atom.res_seq,
                //         atom.chain_id,
                //         atom.x,
                //         atom.y,
                //         atom.z
                //     );
                // }
                
                // You can print all atom details if needed, for example:
                // for atom in &ca_atoms {
                //     println!("{:?}", atom);
                // }

                // Pass the chain to the (placeholder) knot detection function
                find_knots(&ca_atoms);
            }
        }
        Err(e) => {
            eprintln!("Error parsing PDB file: {}", e);
            std::process::exit(1); // Exit with an error code
        }
    }
}

