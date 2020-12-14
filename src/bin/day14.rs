use advent2020::input;

use regex::Regex;
use std::collections::HashMap;

fn write_all(memory: &mut HashMap<u64, u64>, mask_bits: u64, mask_set: u64, addr: u64, val: u64, i: u64) -> u32 {
    if i == 36 {
        memory.insert(addr, val);
        1
    } else if (mask_bits & (1 << i)) > 0 {
        let bit = if (mask_set & (1 << i)) == 0 { 0 } else { 1 };
        write_all(memory, mask_bits, mask_set, addr | (bit << i), val, i+1)
    } else {
        let mut out = 0;
        for bit in 0..=1 {
            out += write_all(memory, mask_bits, mask_set, addr ^ (bit << i), val, i+1);
        }
        out
    }
}

fn main() {
    let input = input::read_all(14);

    let r_mask = Regex::new(r"mask = ([01X]+)").unwrap();
    let r_mem = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();

    let mut memory = HashMap::new();
    let mut mask_bits: u64 = 0;
    let mut mask_set: u64 = 0;

    println!("Part 1");
    for line in input.lines() {
        if let Some(caps) = r_mask.captures(line) {
            let mask = caps.get(1).unwrap().as_str();
            mask_bits = u64::from_str_radix(
                &mask.chars().map(|c| if c == 'X' { '0' } else { '1' }).collect::<String>(),
                2
            ).unwrap();
            mask_set = u64::from_str_radix(
                &mask.chars().map(|c| if c == '1' { '1' } else { '0' }).collect::<String>(),
                2
            ).unwrap();

        }
        if let Some(caps) = r_mem.captures(line) {
            let addr: u64 = caps.get(1).unwrap().as_str().parse().unwrap();
            let val: u64 = caps.get(2).unwrap().as_str().parse().unwrap();

            memory.insert(addr, (val & !mask_bits) | mask_set);
        }
    }

    let sum: u64 = memory.values().sum();
    println!("{}", sum);

    let mut memory = HashMap::new();
    let mut mask_bits: u64 = 0;
    let mut mask_set: u64 = 0;

    println!("Part 2");
    for line in input.lines() {
        if let Some(caps) = r_mask.captures(line) {
            let mask = caps.get(1).unwrap().as_str();
            mask_bits = u64::from_str_radix(
                &mask.chars().map(|c| if c == 'X' { '0' } else { '1' }).collect::<String>(),
                2
            ).unwrap();
            mask_set = u64::from_str_radix(
                &mask.chars().map(|c| if c == '1' { '1' } else { '0' }).collect::<String>(),
                2
            ).unwrap();

        }
        if let Some(caps) = r_mem.captures(line) {
            let addr: u64 = caps.get(1).unwrap().as_str().parse().unwrap();
            let val: u64 = caps.get(2).unwrap().as_str().parse().unwrap();

            let _wrote = write_all(&mut memory, mask_bits, mask_set, addr, val, 0);
            //println!("Wrote {} addresses", wrote);
        }
    }

    let sum: u64 = memory.values().sum();
    println!("{}", sum);
}
