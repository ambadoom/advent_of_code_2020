use advent2020::input;

fn trees(map: &Vec<Vec<bool>>, dx: usize, dy: usize) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut hit_count = 0;

    while y < map.len() {
        if map[y][x % map[y].len()] {
            hit_count += 1;
        }
        y += dy;
        x += dx;
    }

    hit_count
}

fn main() {
    let input = input::read_all(3);

    let map: Vec<Vec<bool>> =
        input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();

    println!("Part 1");
    let solution = trees(&map, 3, 1);
    println!("Hit {} trees", solution);

    println!("Part 2");

//    Right 1, down 1.
//    Right 3, down 1. (This is the slope you already checked.)
//    Right 5, down 1.
//    Right 7, down 1.
//    Right 1, down 2.

    let solution =
        trees(&map, 1, 1) *
        trees(&map, 3, 1) *
        trees(&map, 5, 1) *
        trees(&map, 7, 1) *
        trees(&map, 1, 2);

    println!("Solution = {}", solution);
}
