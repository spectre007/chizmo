use na::base::{DMatrix, Vector3};

use crate::constants::get_element;

pub struct Atom {
    pub element: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

fn compute_distance_matrix(atoms: &Vec<Atom>) -> DMatrix<f64> {
    let n_atoms = atoms.len();
    let mut dist = DMatrix::zeros(n_atoms, n_atoms);

    for i in 0..n_atoms {
        let a = Vector3::new(atoms[i].x, atoms[i].y, atoms[i].z);
        for j in 0..i {
            if i == j { continue; }
            let b = Vector3::new(atoms[j].x, atoms[j].y, atoms[j].z);
            let norm = (b-a).norm();
            dist[(i, j)] = norm;
            dist[(j, i)] = norm;
        }
    }

    dist
}

fn vdw_distance(
    symbol_a: &str,
    symbol_b: &str,
    scaling_factor: Option<f64>
) -> Option<f64> {
    let a = get_element(&symbol_a).unwrap();
    let b = get_element(&symbol_b).unwrap();

    let mut r: f64 = 0.0;
    if let Some(v) = a.van_del_waals_radius {
        r += v as f64;
    } else { return None; }

    if let Some(v) = b.van_del_waals_radius {
        r += v as f64;
    } else { return None; }

    r *= scaling_factor.unwrap_or(1.0) * 1e-2; // Scale and convert to Angstrom
    Some(r)
}

