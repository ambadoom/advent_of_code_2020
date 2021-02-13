use std::fmt;

struct Node {
    next: i32,
}

struct Cups {
    data: Vec<Node>,
    current: i32,
}

impl fmt::Display for Cups {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[ ")?;
        let mut i: i32 = 0;
        let mut first = true;
        while i != 0 || first {
            first = false;
            write!(f, "{} ", i + 1)?;
            i = self.data[i as usize].next;
        }
        write!(f, "]")
    }
}

impl Cups {
    fn new(cups: &Vec<i32>) -> Cups {
        let size = cups.len() as i32;
        let mut data = Vec::with_capacity(size as usize);
        for _i in 0..size {
            data.push(Node {
                next: 0,
            });
        }
        for i in 0..size {
            data[cups[i as usize] as usize] = Node {
                next: cups[(i+1).rem_euclid(size) as usize],
            };
        }
        Cups { data, current: cups[0] }
    }

    fn get(&mut self, i: i32) -> &mut Node {
        &mut self.data[i as usize]
    }

    fn do_move(&mut self) {
        let pickup_start = self.get(self.current).next;
        let pickup_mid = self.get(pickup_start).next;
        let pickup_end = self.get(pickup_mid).next;

        // remove pickup cups
        let before = self.current;
        let after = self.get(pickup_end).next;
        self.get(before).next = after;

        // find destination cup
        let mut destination = (self.current - 1).rem_euclid(self.data.len() as i32);
        while destination == pickup_start || destination == pickup_mid || destination == pickup_end {
            destination = (destination - 1).rem_euclid(self.data.len() as i32);
        }

        // insert pickup cups
        let before = destination;
        let after = self.get(destination).next;
        self.get(before).next = pickup_start;
        self.get(pickup_end).next = after;

        // update current
        self.current = self.get(self.current).next;
    }

    fn get_p2_proof(&mut self) -> u64 {
        let c1 = self.get(0).next;
        let c2 = self.get(c1).next;
        println!("Cups are {} and {}", c1 + 1, c2 + 1);
        return (c1 as u64 + 1) * (c2 as u64 + 1);
    }
}


fn main() {
    let input = "193467258"; // our input
    //let input = "389125467"; // demo
    let mut cups: Vec<i32> = input.chars().map(|c| c.to_digit(10).unwrap() as i32 - 1).collect();

    println!(" == Part 1 == ");
    let mut cups_p1 = Cups::new(&cups);
    for _i in 1..=100 {
        cups_p1.do_move();
    }
    println!("cups: {}", cups_p1);


    println!(" == Part 2 == ");
    let mut i = cups.len();
    while cups.len() < 1_000_000 {
        cups.push(i as i32);
        i += 1;
    }
    let mut cups_p2 = Cups::new(&cups);
    for i in 1..=10_000_000 {
        if i % 1000_000 == 0 {
            println!("{}%", i / 100_000);
        }
        cups_p2.do_move();
    }
    println!("solution: {}", cups_p2.get_p2_proof());
}
