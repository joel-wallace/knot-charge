use crate::protein_model::CaAtom;

#[derive(Debug, Clone)]
pub struct ChainPoint {
    pub serial: i32,       // Atom serial number, corresponds to the CA atom
    pub x: f64,              // X coordinate
    pub y: f64,              // Y coordinate
    pub z: f64,              // Z coordinate
}

impl ChainPoint {
    pub fn new(
        serial: i32,
        x: f64,
        y: f64,
        z: f64,
    ) -> Self {
        ChainPoint {
            serial,
            x,
            y,
            z,
        }
    }
}

pub fn ca_to_chainpoint(ca_atom: &CaAtom) -> ChainPoint {
    ChainPoint::new(
        ca_atom.serial,
        ca_atom.x,
        ca_atom.y,
        ca_atom.z,
    )
}

pub fn backbone_to_chain(ca_atoms: &Vec<CaAtom>) -> Vec<ChainPoint> {
    let mut chainpoints = Vec::<ChainPoint>::new();
    let num_points = ca_atoms.len();
    let mut i = 0;
    while i < num_points {
        chainpoints.push(ca_to_chainpoint(&ca_atoms[i]));
        i = i + 1;
    }

    chainpoints
}

pub fn calc_vector(point_a: &ChainPoint, point_b: &ChainPoint) -> [f64; 3] {
    [point_a.x - point_b.x, point_a.y - point_b.y, point_a.z - point_b.z]
}

pub fn calc_plane(point_a: &ChainPoint, point_b: &ChainPoint, point_c: &ChainPoint) -> [f64; 4] {
    let ab = calc_vector(point_a, point_b);
    let bc = calc_vector(point_b, point_c);
    let plane_x: f64 = ab[1]*bc[2] - ab[2]*bc[1];
    let plane_y: f64 = ab[2]*bc[0] - ab[0]*bc[2];
    let plane_z: f64 = ab[0]*bc[1] - ab[1]*bc[0];
    let plane_k: f64 = -(plane_x*point_a.x - plane_y*point_a.y - plane_z*point_a.z);
    [plane_x, plane_y, plane_z, plane_k]
}

pub fn reduce_chain(chain: &Vec<ChainPoint>) -> Vec<ChainPoint> {
    
}