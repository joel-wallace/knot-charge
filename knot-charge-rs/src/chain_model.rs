extern crate nalgebra as na;
use na::{Vector3, Point3};


use crate::protein_model::CaAtom;
use crate::plane::does_vector_pass_through_triangle_section;

#[derive(Debug, Clone)]
pub struct ChainPoint {
    pub serial: i32,       // Atom serial number, corresponds to the CA atom
    pub coord: Point3<f64>,          // xyz
}

impl ChainPoint {
    pub fn new(
        serial: i32,
        coord: Point3<f64>,
    ) -> Self {
        ChainPoint {
            serial,
            coord,
        }
    }
}

pub fn ca_to_chainpoint(ca_atom: &CaAtom) -> ChainPoint {
    let coord = Point3::new(ca_atom.x, ca_atom.y, ca_atom.z);
    ChainPoint::new(
        ca_atom.serial,
        coord
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

pub fn calc_vector(point_a: &ChainPoint, point_b: &ChainPoint) -> na::Vector3<f64> {
    point_a.coord - point_b.coord 
}

pub fn reduce_chain(mut chain: Vec<ChainPoint>) -> Vec<ChainPoint> {
    
    let mut i = 2;
    // Algorithm to reduce chain complexity without losing any knot present
    while i < chain.len() {
        
        // The three points of the triangle
        let a = chain[i-2].coord;
        let b = chain[i-1].coord;
        let c = chain[i].coord;
        let mut j = 1;
        let mut remove_b = true;
        while j < chain.len() {
            let ray_start = chain[j].coord;
            let ray_direction_fwd = calc_vector(&chain[j], &chain[j-1]);
            let ray_direction_rev = calc_vector(&chain[j-1], &chain[j]);
            match does_vector_pass_through_triangle_section(&a, &b, &c, &ray_start, &ray_direction_fwd, false) {
                Some(point) => { remove_b = false },
                None => {},
            }
            match does_vector_pass_through_triangle_section(&a, &b, &c, &ray_start, &ray_direction_rev, false) {
                Some(point) => { remove_b = false },
                None => {},
            }
            if remove_b == false {
                println!("Intersection found");
                break;
            }
            j = j + 1;
        }
        if remove_b == true {
            chain.remove(i-1);
            println!("Intersection not found");
        } else {
            i = i + 1;
            println!("Intersection found");
        }
    }
    println!("{}", chain.len());
    chain
}