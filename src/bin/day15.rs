use std::collections::HashMap;

fn play(start: &Vec<usize>, count: usize) -> usize {
    let mut map = HashMap::new();
    let mut prev = 0;

    for i in 0..count {
        let dist = if i < start.len() {
            start[i]
        } else if let Some(index) = map.get(&prev) {
            i - index
        } else {
            0
        };
        if i > 0 {
            map.insert(prev, i);
        }
        prev = dist;
    }

    prev
}

fn main() {
    let input = "6,19,0,5,7,13,1";
    //let input = "3,1,2";

    let start: Vec<usize> = input.split(',').map(|n| n.parse().unwrap()).collect();
    
    println!("Part 1");
    println!("{}", play(&start, 2020));

    println!("Part 2");
    println!("{}", play(&start, 30_000_000));
}
