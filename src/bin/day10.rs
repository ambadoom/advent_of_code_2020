use advent2020::input;

use std::collections::HashSet;

fn main() {
    let input = input::read_all(10);
    
    let mut adapters: Vec<isize> = input.lines().map(|l| l.parse().unwrap()).collect();
    adapters.sort();

    println!("Part 1");

    let mut c1 = 0;
    let mut c3 = 0;
    match adapters[0] {
        1 => { c1 += 1; },
        2 => {},
        3 => { c3 += 1; },
        _ => { panic!("can't adapt"); },
    }
    for i in 0..adapters.len()-1 {
        match adapters[i+1]-adapters[i] {
            1 => { c1 += 1; },
            2 => {},
            3 => { c3 += 1; },
            _ => { panic!("can't adapt"); },
        }
    }

    c3 += 1; // Our device

    println!("{}", c1*c3);


    println!("Part 2");
    let top = adapters.iter().max().unwrap() + 3;
    let mut counts: Vec<isize> = vec![0;(top + 1) as usize];
    let mut set: HashSet<isize> = adapters.into_iter().collect();
    set.insert(top);
    counts[0] = 1;
    for i in 0..counts.len() {
        for j in 1..=3 {
            if set.contains(&((i+j) as isize)) {
                counts[i+j] += counts[i];
            }
        }
    }
    //println!("{:?}", counts);
    println!("{}", counts[counts.len()-1]);
}
