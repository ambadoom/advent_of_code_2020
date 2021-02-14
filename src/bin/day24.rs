use advent2020::input;

use std::collections::HashSet;
use std::collections::HashMap;

#[allow(dead_code)]
const DEMO_INPUT: &str =
r"sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

enum Dir {
    SE,
    SW,
    NE,
    NW,
    E,
    W,
}

impl Dir {
    fn mov(&self, pos: &mut (i32, i32)) {
        match self {
            Dir::SE => {
                pos.0 += 1;
                pos.1 += 1;
            },
            Dir::SW => {
                pos.1 += 1;
            },
            Dir::NE => {
                pos.1 -= 1;
            },
            Dir::NW => {
                pos.0 -= 1;
                pos.1 -= 1;
            },
            Dir::E => {
                pos.0 += 1;
            },
            Dir::W => {
                pos.0 -= 1;
            },
        }
    }
}

fn step(set: &HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    let mut neigh: HashMap<(i32, i32), i32> = HashMap::new();
    for tile in set.iter() {
        for dir in &[Dir::SE, Dir::SW, Dir::NE, Dir::NW, Dir::E, Dir::W] {
            let mut pos = *tile;
            dir.mov(&mut pos);
            *neigh.entry(pos).or_insert(0) += 1;
        }
    }

    let mut out: HashSet<(i32, i32)> = HashSet::new();
    for (tile, count) in neigh.iter() {
        match count {
            0 => {
                // Unset
            },
            1 => {
                // Keep
                if set.contains(tile) {
                    out.insert(*tile);
                }
            },
            2 => {
                // Set
                out.insert(*tile);
            },
            _ => {
                // Unset
            },
        }
    }
    out
}

fn main() {
    //let input = DEMO_INPUT; 
    let input = input::read_all(24);

    let mut set = HashSet::new();

    let mut added = 0;
    let mut removed = 0;

    for line in input.lines() {
        let mut l = line;
        let mut pos = (0,0);
        while l.len() > 0 {
            let dir = if let Some(l2) = l.strip_prefix("se") {
                l = l2;
                Dir::SE
            } else if let Some(l2) = l.strip_prefix("sw") {
                l = l2;
                Dir::SW
            } else if let Some(l2) = l.strip_prefix("ne") {
                l = l2;
                Dir::NE
            } else if let Some(l2) = l.strip_prefix("nw") {
                l = l2;
                Dir::NW
            } else if let Some(l2) = l.strip_prefix("e") {
                l = l2;
                Dir::E
            } else if let Some(l2) = l.strip_prefix("w") {
                l = l2;
                Dir::W
            } else {
                panic!("Parse fail");
            };

            dir.mov(&mut pos);
        }

        if set.contains(&pos) {
            removed += 1;
            set.remove(&pos);
        } else {
            added += 1;
            set.insert(pos);
        }
    }

    println!("Part 1");
    println!("{}", set.iter().count());
    println!("added: {}, removed: {}", added, removed);

    println!("Part 2");
    for _i in 1..=100 {
        set = step(&set);
        //println!("Day {}: {}", _i, set.iter().count());
    }
    println!("{}", set.iter().count());
}
