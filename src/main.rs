use std::fs;

fn main() {
    const FNAME: &str = "test_case.xyz";
    let content = fs::read_to_string(&FNAME).expect("Failed to read file!");

    let atoms: Vec<Atom> = parse_xyz(&content).unwrap();

    println!("Read {} atoms", atoms.len());
}

struct Atom {
    element: String,
    x: f64,
    y: f64,
    z: f64,
}

fn parse_xyz(content: &str) -> Option<Vec<Atom>> {
    let lines = content.lines();

    let mut atoms = Vec::new();
    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 4 { continue; }
        let element = parts[0].to_string();
        let x = parts[1].parse::<f64>().ok()?;
        let y = parts[2].parse::<f64>().ok()?;
        let z = parts[3].parse::<f64>().ok()?;
        atoms.push(Atom {
            element,
            x,
            y,
            z,
        });
    }
    Some(atoms)
}

