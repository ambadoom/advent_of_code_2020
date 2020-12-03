use std::fs;

pub fn read_all(n: usize) -> String {
    let path = format!("input/{}/input", n);
    fs::read_to_string(path).expect("Unable to read input file")
}
