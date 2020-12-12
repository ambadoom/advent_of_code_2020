use advent2020::input;

const NORTH: (isize, isize) = (0, 1);
const EAST: (isize, isize) = (1, 0);
const SOUTH: (isize, isize) = (0, -1);
const WEST: (isize, isize) = (-1, 0);

fn rotate(vec: &mut (isize, isize), mut deg: isize) {
    while deg < 0 { deg += 360; }
    while deg >= 360 { deg -= 360; }

    match deg {
        0 => {},
        90 => { *vec = (vec.1, -vec.0);},
        180 => { *vec = (-vec.0, -vec.1); },
        270 => { *vec = (-vec.1, vec.0); },
        d => panic!("Unsupported rotation: {}", d),
    }
}

fn mov(pos: &mut (isize, isize), vec: (isize, isize), quantity: isize) {
    pos.0 += vec.0 * quantity;
    pos.1 += vec.1 * quantity;
}

fn main() {
    let input = input::read_all(12);

    println!("Part 1");
    let mut pos = (0,0);
    let mut facing = EAST;
    for line in input.lines() {
        let instruction = line.get(0..1).unwrap();
        let quantity: isize = line.get(1..).unwrap().parse().unwrap();

        match instruction {
            "N" => { mov(&mut pos, NORTH, quantity); },
            "E" => { mov(&mut pos, EAST, quantity); },
            "S" => { mov(&mut pos, SOUTH, quantity); },
            "W" => { mov(&mut pos, WEST, quantity); },
            "F" => { mov(&mut pos, facing, quantity); },
            "L" => { rotate(&mut facing, -quantity); },
            "R" => { rotate(&mut facing, quantity); },
            _ => { panic!("Unrecognised instruction"); },
        }
    }

    println!("ended up at {:?}", pos);
    println!("distance {}", pos.0.abs() + pos.1.abs());

    println!();

    println!("Part 2");
    let mut pos = (0,0);
    let mut waypoint = (10,1);
    for line in input.lines() {
        let instruction = line.get(0..1).unwrap();
        let quantity: isize = line.get(1..).unwrap().parse().unwrap();

        match instruction {
            "N" => { mov(&mut waypoint, NORTH, quantity); },
            "E" => { mov(&mut waypoint, EAST, quantity); },
            "S" => { mov(&mut waypoint, SOUTH, quantity); },
            "W" => { mov(&mut waypoint, WEST, quantity); },
            "F" => { mov(&mut pos, waypoint, quantity); },
            "L" => { rotate(&mut waypoint, -quantity); },
            "R" => { rotate(&mut waypoint, quantity); },
            _ => { panic!("Unrecognised instruction"); },
        }
    }

    println!("ended up at {:?}", pos);
    println!("distance {}", pos.0.abs() + pos.1.abs());
}
