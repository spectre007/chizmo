extern crate nalgebra as na;

use chizmo::constants::get_element;
use chizmo::matter::get_fragments;
use chizmo::matter::Atom;
use chizmo::parse_xyz;
use std::fs;

fn main() {
    const FNAME: &str = "test_case.xyz";
    let content = fs::read_to_string(&FNAME).expect("Failed to read file!");

    let atoms: Vec<Atom> = parse_xyz(&content).unwrap();

    println!("Read {} atoms", atoms.len());

    let hydrogen = get_element("H").unwrap();

    println!("H name {}", hydrogen.name);

    let indices = get_fragments(&atoms);
    for frag in indices {
        println!("-------");
        for atom in frag {
            println!("{:<2} {: >7.3} {: >7.3} {: >7.3}", atom.element, atom.x, atom.y, atom.z);
        }
    }
}

