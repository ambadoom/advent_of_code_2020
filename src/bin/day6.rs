use advent2020::input;

use std::collections::HashSet;

fn main() {
    let input = input::read_all(6);

    let mut groups = Vec::new();
    let mut group = Vec::new();

    for line in input.lines() {
        if line.len() == 0 {
            let mut g = Vec::new();
            std::mem::swap(&mut group, &mut g);
            groups.push(g);
        } else {
            let mut person = HashSet::new();
            for c in line.chars() {
                if !('a'..='z').contains(&c) {
                    println!("Unexepected character: {}", c);
                } else {
                    person.insert(c);
                }
            }
            group.push(person);
        }
    }

    let mut g = Vec::new();
    std::mem::swap(&mut group, &mut g);
    groups.push(g);

    let mut c_any = 0;
    let mut c_all = 0;
    for group in groups.iter() {
        for c in 'a'..='z' {
            let mut any = false;
            let mut all = true;
            for person in group.iter() {
                if person.contains(&c) {
                    any = true;
                } else {
                    all = false;
                }
            }
            if any { c_any += 1; }
            if all { c_all += 1; }
        }
    }
    println!("Part 1");
    println!("{}", c_any);
    println!("Part 2");
    println!("{}", c_all);
}
