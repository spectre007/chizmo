use itertools::Itertools;
use na::base::{DMatrix, Vector3};
use std::collections::{HashMap, HashSet};

use crate::constants::get_element;

#[derive(Clone)]
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
            if i == j {
                continue;
            }
            let b = Vector3::new(atoms[j].x, atoms[j].y, atoms[j].z);
            let norm = (b - a).norm();
            dist[(i, j)] = norm;
            dist[(j, i)] = norm;
        }
    }

    dist
}

fn vdw_distance(symbol_a: &str, symbol_b: &str, scaling_factor: Option<f64>) -> Option<f64> {
    let a = get_element(&symbol_a).unwrap();
    let b = get_element(&symbol_b).unwrap();

    let mut r: f64 = 0.0;
    if let Some(v) = a.van_del_waals_radius {
        r += v as f64;
    } else {
        return None;
    }

    if let Some(v) = b.van_del_waals_radius {
        r += v as f64;
    } else {
        return None;
    }

    r *= scaling_factor.unwrap_or(1.0) * 1e-2; // Scale and convert to Angstrom
    Some(r)
}

fn make_vdw_bond_table(atoms: &Vec<Atom>, scaling_factor: Option<f64>) -> HashMap<String, f64> {
    let mut table: HashMap<String, f64> = HashMap::new();
    let unique_elements: HashSet<String> = atoms.iter().map(|a| a.element.clone()).collect();

    let combinations: HashSet<_> = unique_elements
        .clone()
        .into_iter()
        .combinations_with_replacement(2)
        .collect();
    let permutations: HashSet<_> = unique_elements.into_iter().permutations(2).collect();

    let union = combinations.union(&permutations);
    for pair in union {
        let vdw = vdw_distance(&pair[0], &pair[1], scaling_factor).unwrap_or(-1.0_f64);
        table.insert(format!("{}-{}", pair[0].clone(), pair[1].clone()), vdw);
    }
    table
}

fn get_adjacency_matrix(atoms: &Vec<Atom>) -> DMatrix<bool> {
    let n_atoms = atoms.len();
    let mut interact = DMatrix::from_element(n_atoms, n_atoms, false);
    let distance = compute_distance_matrix(&atoms);
    let vdw_table = make_vdw_bond_table(&atoms, Some(0.5));

    for i in 0..n_atoms {
        let a = atoms[i].element.clone();
        for j in 0..i {
            if i == j {
                continue;
            }
            let b = atoms[j].element.clone();
            let vdw_distance = vdw_table
                .get(&format!("{}-{}", &a, &b))
                .cloned()
                .unwrap_or(-1.0_f64);
            interact[(i, j)] = distance[(i, j)] <= vdw_distance;
            interact[(j, i)] = distance[(i, j)] <= vdw_distance;
        }
    }

    interact
}

pub fn get_fragment_indices(atoms: &Vec<Atom>) -> Vec<Vec<usize>> {
    let mut fragments = Vec::new();
    let adj = get_adjacency_matrix(&atoms);
    let mut visited = vec![false; adj.nrows()];

    for i in 0..adj.nrows() {
        if !visited[i] {
            let mut fragment = Vec::new();
            dfs(i, &adj, &mut visited, &mut fragment);
            fragments.push(fragment);
        }
    }

    fragments
}

fn dfs(
    atom_index: usize,
    adjacency_matrix: &DMatrix<bool>,
    visited: &mut Vec<bool>,
    fragment: &mut Vec<usize>,
) {
    visited[atom_index] = true;
    fragment.push(atom_index);

    for j in 0..adjacency_matrix.ncols() {
        if adjacency_matrix[(atom_index, j)] && !visited[j] {
            dfs(j, adjacency_matrix, visited, fragment);
        }
    }
}

pub fn get_fragments(atoms: &Vec<Atom>) -> Vec<Vec<Atom>> {
    let frag_index_groups = get_fragment_indices(&atoms);
    let mut fragments = Vec::new();

    for frag_group in frag_index_groups {
        let mut fragment = Vec::new();
        for ifrag in frag_group {
            fragment.push(atoms[ifrag].clone());
        }
        fragments.push(fragment);
    }

    fragments
}
