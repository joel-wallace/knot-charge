#[derive(Debug, Clone)]
pub struct CaAtom {
    pub serial: i32,       // Atom serial number
    pub atom_name: String,   // Atom name (should be "CA")
    pub res_name: String,    // Residue name (e.g., "ALA", "LYS")
    pub chain_id: char,      // Chain identifier (e.g., 'A', 'B')
    pub res_seq: i32,        // Residue sequence number
    pub x: f64,              // X coordinate
    pub y: f64,              // Y coordinate
    pub z: f64,              // Z coordinate
}

// For knot detection on a C-alpha trace, a Vec<CaAtom> is a good starting point.

impl CaAtom {
    /// Creates a new CaAtom.
    /// This is a basic constructor. Parsing logic will be more complex.
    pub fn new(
        serial: i32,
        atom_name: String,
        res_name: String,
        chain_id: char,
        res_seq: i32,
        x: f64,
        y: f64,
        z: f64,
    ) -> Self {
        CaAtom {
            serial,
            atom_name,
            res_name,
            chain_id,
            res_seq,
            x,
            y,
            z,
        }
    }
}
