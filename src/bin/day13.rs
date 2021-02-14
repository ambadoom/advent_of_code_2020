use advent2020::input;

fn main() {
    let input = input::read_all(13);

    let mut lines = input.lines();

    let available: usize = lines.next().unwrap().parse().unwrap();
    println!("Available at {}", available);
    let buses: Vec<usize> = lines.next().unwrap().split(',').filter(|a| *a != "x").map(|b| b.parse().unwrap()).collect();

    println!("Part 1");
    let bus = buses.iter().min_by_key(|id| (*id - (available % *id)) % *id).unwrap();
    println!("{}: {}", bus, bus * ((bus - (available % bus)) % bus));

    println!("Part 2");
    let mut lines = input.lines();
    let _ = lines.next();
    let mut req: Vec<(usize, usize)> = lines
        .next()
        .unwrap()
        .split(",")
        .enumerate()
        .filter(|(_i, a)| *a != "x")
        .map(|(i, b)| { let n = b.parse().unwrap(); ((n - (i%n)) % n, n) })
        .collect();
    req.sort_by_key(|(_i, id)| -(*id as isize));

    println!("{:?}", req);
    let mut solution = 0;
    let mut increment = 1;
    let mut count = 0;
    while count < req.len() {
        for (a, n) in req[count..].iter() {
            if solution % n != *a {
                break;
            }
            count += 1;
            increment *= n;
            println!("{} {}", count, solution);
        }
        if count == req.len() { break; }

        solution += increment;
    }
    println!("{}", solution);
    for (a, n) in req.iter() {
        if solution % n != *a {
            panic!("Solution not valid");
        }
    }

}
