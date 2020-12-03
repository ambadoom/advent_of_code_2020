use advent2020::input;
use regex::Regex;

fn main() {
    let input = input::read_all(2);
    let lines: Vec<String> = input.lines().map(|s| s.to_owned()).collect();

    let parse = Regex::new(r"^(\d+)-(\d+) (.): (.*)$").unwrap();

    let mut p1valid = 0;
    let mut p1invalid = 0;

    let mut p2valid = 0;
    let mut p2invalid = 0;
    for line in lines {
        let caps = parse.captures(&line).unwrap();
        let min: usize = caps.get(1).unwrap().as_str().parse().unwrap();
        let max: usize = caps.get(2).unwrap().as_str().parse().unwrap();
        let chr: char = caps.get(3).unwrap().as_str().parse().unwrap();
        let pass: String = caps.get(4).unwrap().as_str().to_owned();

        // Part 1
        let count = pass.chars().filter(|c| c == &chr).count();
        if (min..=max).contains(&count) {
            p1valid += 1;
        } else {
            p1invalid += 1;
        }

        // Part 2
        let c1 = pass.chars().nth(min-1).map(|c| c == chr).unwrap_or(false);
        let c2 = pass.chars().nth(max-1).map(|c| c == chr).unwrap_or(false);
        if c1 ^ c2 {
            p2valid += 1;
        } else {
            p2invalid += 1;
        }
    }
    println!("Part 1");
    println!("valid = {} invalid = {}", p1valid, p1invalid);

    println!("Part 2");
    println!("valid = {} invalid = {}", p2valid, p2invalid);

}
