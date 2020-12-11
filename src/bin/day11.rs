use advent2020::input;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
    Seat,
    Person,
    Floor,
}

type World = Vec<Vec<Cell>>;

impl Cell {
    fn from_char(c: char) -> Cell {
        match c {
            '.' => Cell::Floor,
            'L' => Cell::Seat,
            '#' => Cell::Person,
            _ => panic!("invalid char for cell"),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Cell::Floor => '.',
            Cell::Person => '#',
            Cell::Seat => 'L',
        }
    }
}

fn map_cell<F: FnMut((usize, usize), Cell) -> Cell>(world: &World, mut f: F) -> World {
    world
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter().enumerate().map(|(j, cell)| f((i,j), *cell)).collect()
        }).collect()
}

fn show(world: &World) -> String {
    let mut out = String::new();
    for row in world.iter() {
        for cell in row.iter() {
            out.push(cell.to_char());
        }
        out.push('\n');
    }

    out
}

fn neigh_part1(world: &World, pos: (isize, isize)) -> usize {
    let mut out = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 { continue; }
            let y = pos.0 + i;
            let x = pos.1 + j;
            if x < 0 || y < 0 || y as usize >= world.len() || x as usize >= world[y as usize].len() { continue; }
            if world[y as usize][x as usize] == Cell::Person {
                out += 1;
            }
        }
    }
    out
}

fn step_part1(world: &World) -> World {
    map_cell(world , |(i,j), cell| {
        if cell == Cell::Floor {
            Cell::Floor
        } else {
            match neigh_part1(world, (i as isize, j as isize)) {
                0 => Cell::Person,
                4..=8 => Cell::Seat,
                _ => cell,
            }
        }
    })
}

fn neigh_part2(world: &World, pos: (isize, isize)) -> usize {
    let mut out = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 { continue; }
            for n in 1.. {
                let y = pos.0 + (n*i);
                let x = pos.1 + (n*j);
                if x < 0 || y < 0 || y as usize >= world.len() || x as usize >= world[y as usize].len() { break; }
                match world[y as usize][x as usize] {
                    Cell::Person => {
                        out += 1;
                        break;
                    },
                    Cell::Seat => {
                        break;
                    },
                    Cell::Floor => {
                        // do nothing
                    },
                }
            }
        }
    }
    out
}

fn step_part2(world: &World) -> World {
    map_cell(world , |(i,j), cell| {
        if cell == Cell::Floor {
            Cell::Floor
        } else {
            match neigh_part2(world, (i as isize, j as isize)) {
                0 => Cell::Person,
                5..=8 => Cell::Seat,
                _ => cell,
            }
        }
    })
}

fn find_stable<F: Fn(&World) -> World>(world_in: &World, step: F) -> usize {
    let mut world = world_in.clone();
    let mut prev_world = world;
    world = step(&prev_world);

    let mut steps = 1;

    while world != prev_world {
        prev_world = world;
        world = step(&prev_world);
        steps += 1;
    }

    println!("Found fixed point after {} steps", steps);
    //println!("{}", show(&world));
    
    world.iter().map(|r| r.iter()).flatten().filter(|cell| **cell == Cell::Person).count()
}

fn main() {
    let input = input::read_all(11);

    let mut world: Vec<Vec<Cell>> = input
        .lines()
        .map(|line| line.chars().map(Cell::from_char).collect())
        .collect();

    //println!("{}", show(&world));

    println!("Part 1"); 
    println!("{}", find_stable(&world, step_part1)); 

    println!("Part 2"); 
    println!("{}", find_stable(&world, step_part2)); 

}


