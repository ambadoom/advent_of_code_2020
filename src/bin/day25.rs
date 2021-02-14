use advent2020::input;

const P: u64 = 20201227;
const G: u64 = 7;

fn brute_force_log(t: u64) -> u64 {
    let mut n = 1;
    let mut i = 0;
    while n != t {
        i += 1;
        n = (n * G) % P;
    }
    i
}

fn main() {
    let input = input::read_all(25);

    let mut lines = input.lines();
    let pa = lines.next().unwrap().parse::<u64>().unwrap();
    let pb = lines.next().unwrap().parse::<u64>().unwrap();

    let a = brute_force_log(pa);
    println!("a = {}", brute_force_log(pa));
    println!("Part 1");
    let mut solution = 1;
    for _ in 0..a {
        solution = (solution * pb) % P;
    }
    println!("{}", solution);
}
