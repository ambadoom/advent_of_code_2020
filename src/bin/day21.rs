use advent2020::input;

use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let input = input::read_all(21);
    let mut foods: Vec<(HashSet<&str>, HashSet<&str>)> = Vec::new();
    for line in input.lines() {
        let mut sp = line.split('(');
        let ingredients = sp
            .next()
            .unwrap()
            .split(' ')
            .filter(|s| !s.is_empty())
            .collect();
        let allergens = sp
            .next()
            .unwrap()
            .strip_prefix("contains")
            .unwrap()
            .strip_suffix(")")
            .unwrap()
            .split(",")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();
        foods.push((ingredients, allergens));
    }

    let ingredients: HashSet<&str> = foods
        .iter()
        .map(|(i, _a)| i.iter())
        .flatten()
        .copied()
        .collect();

    let allergens: HashSet<&str> = foods
        .iter()
        .map(|(_i, a)| a.iter())
        .flatten()
        .copied()
        .collect();

    let mut all_candidates: HashMap<&str, HashSet<&str>> = HashMap::new();
    for allergen in allergens {
        let mut candidates: Option<HashSet<&str>> = None;
        for (is, _as) in foods.iter().filter(|f| f.1.contains(allergen)) {
            if let Some(cs) = candidates {
                candidates = Some(cs.intersection(is).copied().collect())
            } else {
                candidates = Some(is.clone());
            }
        }
        all_candidates.insert(allergen, candidates.unwrap());
    }

    // If any allergen has only one possibility we can use that to further narrow down the others
    let mut known: HashMap<&str, &str> = HashMap::new();
    while !all_candidates.is_empty() {
        let allergen = all_candidates
            .iter()
            .filter(|(_a,is)| is.len() == 1)
            .map(|(a,_is)| *a)
            .next()
            .unwrap();

        let ingredient: &str = *all_candidates
            .remove(allergen)
            .unwrap()
            .iter()
            .next()
            .unwrap();

        known.insert(allergen, ingredient);

        for (_a, is) in all_candidates.iter_mut() {
            is.remove(ingredient);
        }
    }

    let known_i2a: HashMap<&str, &str> = known.iter().map(|(a,i)| (*i,*a)).collect();

    assert!(all_candidates.is_empty());
    println!("{:?}", known);

    println!("Part 1");
    let count = foods
        .iter()
        .map(|(is, _)| is.iter())
        .flatten()
        .filter(|i| !known_i2a.contains_key(*i))
        .count(); 
    println!("{}", count);

    println!("Part 2");
    let mut dangerous: Vec<&str> = ingredients
        .iter()
        .filter(|i| known_i2a.contains_key(*i))
        .copied()
        .collect();
    
    dangerous.sort_by_key(|i| known_i2a[i]);
    println!("{}", dangerous.join(","));

}
