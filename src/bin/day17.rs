use advent2020::input;

use std::collections::HashSet;
use std::collections::HashMap;

use std::cmp::Eq;
use std::hash::Hash;

type Coord3 = (i16, i16, i16);
type Coord4 = (i16, i16, i16, i16);

fn step_common<T: Eq + Hash + Clone>(world: &HashSet<T>, neigh: &HashMap<T,usize>) -> HashSet<T> {
    let mut out = HashSet::new();
    for (pos, count) in neigh.iter() {
        let active = world.contains(pos);
        let n_active = match count {
            2 => active, // keep
            3 => true, // create
            _ => false, // destroy
        };
        if n_active {
            out.insert(pos.clone());
        }
    }

    out
}

fn step3(world: &HashSet<Coord3>) -> HashSet<Coord3> {
    let mut neigh = HashMap::new();
    for pos in world.iter() {
        for i in -1..=1 {
            for j in -1..=1 {
                for k in -1..=1 {
                    let entry = neigh.entry((pos.0+i,pos.1+j,pos.2+k)).or_insert(0);
                    if (i,j,k) != (0,0,0) {
                        *entry += 1;
                    }
                }
            }
        }
    }

    step_common(world, &neigh)
}

fn step4(world: &HashSet<Coord4>) -> HashSet<Coord4> {
    let mut neigh = HashMap::new();
    for pos in world.iter() {
        for i in -1..=1 {
            for j in -1..=1 {
                for k in -1..=1 {
                    for l in -1..=1 {
                        let entry = neigh
                            .entry((pos.0+i,pos.1+j,pos.2+k,pos.3+l))
                            .or_insert(0);
                        if (i,j,k,l) != (0,0,0,0) {
                            *entry += 1;
                        }
                    }
                }
            }
        }
    }

    step_common(world, &neigh)
}

fn main() {
    let input = input::read_all(17);

    let mut start = HashSet::new();

    for (i, line) in input.lines().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            if ch == '#' {
                start.insert((i as i16, j as i16));
            }
        }
    }

    println!("Part 1");
    let mut world3d: HashSet<Coord3> = start.iter().map(|(i,j)| (*i,*j,0)).collect();
    for _ in 0..6 {
        world3d = step3(&world3d);
    }
    println!("{}", world3d.len());

    println!("Part 2");
    let mut world4d: HashSet<Coord4> = start.iter().map(|(i,j)| (*i,*j,0,0)).collect();
    for _ in 0..6 {
        world4d = step4(&world4d);
    }
    println!("{}", world4d.len());
}
