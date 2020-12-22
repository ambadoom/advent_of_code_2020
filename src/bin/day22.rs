use advent2020::input;

use std::collections::VecDeque;
use std::collections::HashSet;

type Deck = VecDeque<usize>;

fn play1(p1: &mut Deck, p2: &mut Deck) -> bool {
    while !p1.is_empty() && !p2.is_empty() {
        let p1v = p1.pop_front().unwrap();
        let p2v = p2.pop_front().unwrap();
        if p1v > p2v {
            p1.push_back(p1v);
            p1.push_back(p2v);
        } else {
            p2.push_back(p2v);
            p2.push_back(p1v);
        }
    }
    !p1.is_empty()
}

fn play2(p1: &mut Deck, p2: &mut Deck) -> bool {
    let mut visited = HashSet::new();
    while !p1.is_empty() && !p2.is_empty() {
        if !visited.insert((p1.clone(), p2.clone())) {
            return true;
        }

        let p1v = p1.pop_front().unwrap();
        let p2v = p2.pop_front().unwrap();

        let p1win = if p1.len() >= p1v && p2.len() >= p2v {
            let mut p1_ = p1.clone();
            let mut p2_ = p2.clone();
            p1_.truncate(p1v);
            p2_.truncate(p2v);
            play2(&mut p1_, &mut p2_)
        } else {
            p1v > p2v
        };

        if p1win {
            p1.push_back(p1v);
            p1.push_back(p2v);
        } else {
            p2.push_back(p2v);
            p2.push_back(p1v);
        }
    }
    !p1.is_empty()
}

fn score(deck: &Deck) -> usize {
    let mut out = 0;
    for (i, c) in deck.iter().enumerate() {
        out += c * (deck.len() - i);
    }
    out
}

fn main() {
    let input = input::read_all(22);

    let mut lines = input.lines();

    let mut player1: Deck = Deck::new();
    lines.next().unwrap();
    for line in &mut lines {
        if line.is_empty() { break; }
        player1.push_back(line.parse().unwrap());
    }

    let mut player2: Deck = Deck::new();
    lines.next().unwrap();
    for line in &mut lines {
        if line.is_empty() { break; }
        player2.push_back(line.parse().unwrap());
    }

    println!("Part 1");
    let mut p1 = player1.clone();
    let mut p2 = player2.clone();
    let p1winner = play1(&mut p1, &mut p2);
    if p1winner {
        println!("Player 1 wins with score {}", score(&p1));
    } else {
        println!("Player 2 wins with score {}", score(&p2));
    }

    println!("Part 2");
    let mut p1 = player1.clone();
    let mut p2 = player2.clone();
    let p1winner = play2(&mut p1, &mut p2);
    if p1winner {
        println!("Player 1 wins with score {}", score(&p1));
    } else {
        println!("Player 2 wins with score {}", score(&p2));
    }
    
}
