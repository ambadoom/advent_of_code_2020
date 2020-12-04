use advent2020::input;

use std::collections::HashMap;

use regex::Regex;

fn numcheck<F: Fn(isize)->bool>(s: &str, f: F) -> bool {
    s.parse::<isize>().map(f).unwrap_or(false)
}

fn main() {
    let input = input::read_all(4);

    let hclre = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    let pidre = Regex::new(r"^[0-9]{9}$").unwrap();

    let required: Vec<(&'static str, Box<dyn Fn(&str)->bool>)> = vec![
        ("byr", Box::new(|byr| {
            numcheck(byr, |n| n <= 2002 && n >= 1920)
        })), 
        ("iyr", Box::new(|iyr| { 
            numcheck(iyr, |n| n <= 2020 && n >= 2010)
        })), 
        ("eyr", Box::new(|eyr| { 
            numcheck(eyr, |n| n <= 2030 && n >= 2020)
        })), 
        ("hgt", Box::new(|hgt| { 
            if hgt.ends_with("cm") {
                numcheck(hgt.strip_suffix("cm").unwrap(), |n| n <= 193 && n >= 150)
            } else if hgt.ends_with("in") {
                numcheck(hgt.strip_suffix("in").unwrap(), |n| n <= 76 && n >= 59)
            } else {
                false
            }
        })), 
        ("hcl", Box::new(|hcl| { 
            hclre.is_match(hcl)
        })), 
        ("ecl", Box::new(|ecl| { 
            match ecl {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                _ => false,
            }
        })), 
        ("pid", Box::new(|pid| { 
            pidre.is_match(pid) 
        }))
    ];
    let mut passports = Vec::new();
    let mut passport = HashMap::new();

    for line in input.lines() {
        if line.len()==0 {
            let mut pass = HashMap::new();
            std::mem::swap(&mut pass, &mut passport);
            passports.push(pass);
        } else {
            for kv in line.split(" ").map(|kv| kv.split(":").collect::<Vec<&str>>()) {
                passport.insert(kv[0], kv[1]);
            }
        }
    }
    let mut pass = HashMap::new();
    std::mem::swap(&mut pass, &mut passport);
    passports.push(pass);

    let mut ok = 0;
    let mut valid = 0;
    for passport in passports.iter() {
        let mut pass_ok = true;
        let mut pass_valid = true;
        for element in required.iter() {
            if !passport.contains_key(element.0) {
                pass_ok = false;
                pass_valid = false;
                break;
            }

            if !element.1(passport[element.0]) {
                pass_valid = false;
                break;
            }
        }
        if pass_ok { ok += 1; }
        if pass_valid { valid += 1; }
    }
    println!("Part 1");
    println!("ok = {} / {}", ok, passports.len());
    println!("Part 2");
    println!("valid = {} / {}", valid, passports.len());

}
