extern crate nalgebra as na;

use crate::matter::Atom;

pub mod constants;
pub mod matter;


pub fn parse_xyz(content: &str) -> Option<Vec<Atom>> {
    let lines = content.lines();

    let mut atoms = Vec::new();
    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 4 {
            continue;
        }
        let element = parts[0].to_string();
        let x = parts[1].parse::<f64>().ok()?;
        let y = parts[2].parse::<f64>().ok()?;
        let z = parts[3].parse::<f64>().ok()?;
        atoms.push(Atom { element, x, y, z });
    }
    Some(atoms)
}
