use advent2020::input;

use std::collections::HashSet;

const TARGET: isize = 2020;

fn find(nums: &HashSet<isize>, target: isize) -> Option<isize> {
    for num in nums.iter() {
        if nums.contains(&(target-num)) {
            return Some(num * (target-num));
        }
    }
    None
}

fn main() {
    let input = input::read_all(1);

    let mut nums = HashSet::new();

    for line in input.lines() {
        let num: isize = line.parse().unwrap();
        nums.insert(num);
    }

    println!("Part 1");
    if let Some(solution) = find(&nums, TARGET) {
        println!("Solved: {}", solution);
    } else {
        println!(":(");
    }

    println!("Part 2");
    for num in nums.iter() {
        if let Some(solution) = find(&nums, TARGET-num) {
            println!("Solved: {}", solution * num);
            return;
        }
    }
    println!(":(");
}
