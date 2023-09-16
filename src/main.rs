use clap::Parser;
use std::fs;
use chizmo::parse_xyz;
use chizmo::matter::Atom;
use chizmo::matter::get_fragments;

#[derive(Parser)]
struct Cli {
    // path: std::path::PathBuf,
    path: String,
}

fn main() {
    let args = Cli::parse();
    let content = fs::read_to_string(args.path).expect("Failed to read file!");
    let atoms: Vec<Atom> = parse_xyz(&content).unwrap();
    let fragments: Vec<Vec<Atom>> = get_fragments(&atoms);

    for frag in fragments {
        println!("---");
        for atom in frag {
            println!("{:<2} {: >7.3} {: >7.3} {: >7.3}", atom.element, atom.x, atom.y, atom.z);
        }
    }
}

