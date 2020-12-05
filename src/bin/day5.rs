use advent2020::input;

use std::collections::HashSet;

fn decode(seat: &str) -> (isize, isize, isize) {
    let mut row = 0;
    let mut col = 0;
    let chars: Vec<char> = seat.chars().collect();
    for c in chars[0..7].iter() {
        row = row << 1;
        if *c == 'B' {
            row += 1;
        }
    }

    for c in chars[7..].iter() {
        col <<= 1;
        if *c == 'R' {
            col += 1;
        }
    }

    (row, col, row*8 + col)
}

fn main() {
    println!("{:?}", decode("BFFFBBFRRR"));
    let input = input::read_all(5);

    let mut max = 0;
    let mut seats = HashSet::new();
    for line in input.lines() {
        let dec = decode(line);
        seats.insert(dec.2);

        if dec.2 > max {
            max = dec.2;
        }
    }
    println!("Part 1");
    println!("{}", max);

    println!("Part 2");
    for seat in seats.iter() {
        if !seats.contains(&(seat + 1)) &&
            seats.contains(&(seat + 2)) {
                println!("{}", seat+1);
        }
    }
}
