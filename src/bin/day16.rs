use advent2020::input;

use regex::Regex;

use std::collections::HashMap;

struct Field<'a> {
    name: &'a str,
    l1: usize,
    h1: usize,
    l2: usize,
    h2: usize,
}

impl <'a> Field<'a> {
    fn check(&self, n: usize) -> bool {
        //println!("{}-{} or {}-{} : {}", self.l1, self.h1, self.l2, self.h2, n);
        (self.l1..=self.h1).contains(&n) || (self.l2..=self.h2).contains(&n)
    }
}

fn solve(can_go: &Vec<Vec<usize>>) -> Option<Vec<usize>> {
    if can_go.len() == 0 { return Some(Vec::new()); }

    let position = can_go.iter().map(|v| v.len()).enumerate().min_by_key(|(_,l)| *l).unwrap().0;

    for candidate in can_go[position].iter() {
        let mut new_can_go = can_go.clone();
        new_can_go.remove(position);
        for v in new_can_go.iter_mut() {
            v.retain(|i| i != candidate);
        }

        match solve(&new_can_go) {
            None => {},
            Some(mut out) => {
                out.insert(position, *candidate);
                return Some(out);
            }
        }
    }
    None
}

fn main() {
    let input = input::read_all(16);

    let mut lines = input.lines();
    
    let re_field = Regex::new(r"([a-zA-Z ]+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();

    let mut fields = Vec::new();
    for line in &mut lines {
        if let Some(caps) = re_field.captures(line) {
            fields.push(Field {
                name: caps.get(1).unwrap().as_str(),
                l1: caps.get(2).unwrap().as_str().parse().unwrap(),
                h1: caps.get(3).unwrap().as_str().parse().unwrap(),
                l2: caps.get(4).unwrap().as_str().parse().unwrap(),
                h2: caps.get(5).unwrap().as_str().parse().unwrap(),
            });
        } else {
            break;
        }
    }

    assert_eq!(lines.next(), Some("your ticket:"));

    let our_ticket: Vec<usize> = lines.next().unwrap().split(',').map(|n| n.parse().unwrap()).collect();
    assert_eq!(our_ticket.len(), fields.len());
    assert_eq!(lines.next(), Some(""));
    assert_eq!(lines.next(), Some("nearby tickets:"));

    let mut tickets = Vec::new();
    for line in &mut lines {
        let ticket: Vec<usize> = line.split(',').map(|n| n.parse().unwrap()).collect();
        assert_eq!(ticket.len(), fields.len());
        tickets.push(ticket);
    }

    println!("Part 1");
    let mut rate = 0;
    for ticket in tickets.iter() {
        for item in ticket.iter() {
            if !fields.iter().any(|f| f.check(*item)) {
                rate += item;
            }
        }
    }
    println!("{}", rate);

    println!("Part 2");
    tickets.retain(|ticket| ticket.iter().all(|item| fields.iter().any(|f| f.check(*item))));
    tickets.push(our_ticket.clone());

    let mut can_go: Vec<Vec<usize>> = Vec::new();
    for j in 0..fields.len() {
        let mut pos = Vec::new();
        for (i, field) in fields.iter().enumerate() {
            if tickets.iter().all(|ticket| field.check(ticket[j])) {
                pos.push(i);
            }
        }

        can_go.push(pos);
    }

    let field_pos = solve(&can_go).unwrap();
    //println!("{:?}", field_pos);

    let solution : usize = field_pos
        .iter()
        .enumerate()
        .map(|(i, fi)| (fields[*fi].name, our_ticket[i]))
        .filter(|(name, _)| name.starts_with("departure"))
        .map(|(name, val)| val)
        .product();
    println!("{}", solution);
    
}
