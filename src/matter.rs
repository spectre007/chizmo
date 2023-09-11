use na::base::{DMatrix, Vector3};

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

