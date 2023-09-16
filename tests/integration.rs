use chizmo::matter::get_fragments;
use chizmo::matter::Atom;
use chizmo::parse_xyz;
use pretty_assertions::assert_eq;
use std::fs;

fn get_test_case(filename: &str) -> Vec<Atom> {
    let content = fs::read_to_string(filename).expect("Failed to read file!");
    let atoms: Vec<Atom> = parse_xyz(&content).unwrap();
    atoms
}

macro_rules! test_case {
    ($fname:expr) => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/", $fname)
    };
}

#[test]
fn it_gives_fragments_case_1() {
    let test_case = test_case!("test_case_1.xyz");
    let atoms = get_test_case(test_case);
    let fragments: Vec<Vec<Atom>> = get_fragments(&atoms);
    assert_eq!(5, fragments.len());
    assert_eq!(2, fragments[0].len());
    assert_eq!("Cl", fragments[0][0].element);
    assert_eq!("H", fragments[0][1].element);
}
