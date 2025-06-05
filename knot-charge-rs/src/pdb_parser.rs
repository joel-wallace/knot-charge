// src/pdb_parser.rs

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use crate::protein_model::CaAtom; // Import the CaAtom struct
use crate::chain_model::{ChainPoint, ca_to_chainpoint, backbone_to_chain}; // Import what you need

/// Parses a PDB file and extracts C-alpha (CA) atoms.
///
/// # Arguments
/// * `file_path` - The path to the PDB file.
///
/// # Returns
/// * `Result<Vec<CaAtom>, String>` - A vector of CaAtom structs if successful,
///   or an error message string if parsing fails or the file cannot be read.
pub fn parse_pdb_ca_atoms(file_path: &str) -> Result<Vec<CaAtom>, String> {
    let file = File::open(file_path)
        .map_err(|e| format!("Failed to open file '{}': {}", file_path, e))?;
    let reader = BufReader::new(file);

    let mut ca_atoms = Vec::new();

    for (line_num, line_result) in reader.lines().enumerate() {
        let line = line_result
            .map_err(|e| format!("Failed to read line {} from '{}': {}", line_num + 1, file_path, e))?;

        // PDB ATOM records start with "ATOM  " or "HETATM"
        // We are typically interested in ATOM records for the main protein chain.
        if line.starts_with("ATOM  ") {
            // https://www.wwpdb.org/documentation/file-format-content/format33/sect9.html#ATOM

            // Atom name (cols 13-16)
            let atom_name = line.get(12..16).unwrap_or("").trim();

            // We are only interested in C-alpha atoms.
            if atom_name == "CA" {
                let serial_str = line.get(6..11).unwrap_or("").trim();
                let res_name_str = line.get(17..20).unwrap_or("").trim();
                let chain_id_char = line.get(21..22).unwrap_or(" ").chars().next().unwrap_or(' ');
                let res_seq_str = line.get(22..26).unwrap_or("").trim();
                let x_str = line.get(30..38).unwrap_or("").trim();
                let y_str = line.get(38..46).unwrap_or("").trim();
                let z_str = line.get(46..54).unwrap_or("").trim();

                // Basic error checking for parsing numbers
                let serial = serial_str.parse::<i32>()
                    .map_err(|e| format!("Line {}: Failed to parse atom serial '{}': {}", line_num + 1, serial_str, e))?;
                let res_seq = res_seq_str.parse::<i32>()
                    .map_err(|e| format!("Line {}: Failed to parse residue sequence '{}': {}", line_num + 1, res_seq_str, e))?;
                let x = x_str.parse::<f64>()
                    .map_err(|e| format!("Line {}: Failed to parse X coordinate '{}': {}", line_num + 1, x_str, e))?;
                let y = y_str.parse::<f64>()
                    .map_err(|e| format!("Line {}: Failed to parse Y coordinate '{}': {}", line_num + 1, y_str, e))?;
                let z = z_str.parse::<f64>()
                    .map_err(|e| format!("Line {}: Failed to parse Z coordinate '{}': {}", line_num + 1, z_str, e))?;

                ca_atoms.push(CaAtom {
                    serial,
                    atom_name: atom_name.to_string(),
                    res_name: res_name_str.to_string(),
                    chain_id: chain_id_char,
                    res_seq,
                    x,
                    y,
                    z,
                });
            }
        }
    }

    if ca_atoms.is_empty() {
        return Err(format!("No C-alpha atoms found in '{}'", file_path));
    }

    let chain :Vec<ChainPoint> = backbone_to_chain(&ca_atoms);


    Ok(ca_atoms)
}

