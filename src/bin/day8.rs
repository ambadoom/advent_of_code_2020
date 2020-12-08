use advent2020::input;

use std::collections::HashSet;

#[derive(Debug, Clone)]
enum Instruction {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

fn run(prog: &Vec<Instruction>) -> (bool, isize) {
    let mut visited = HashSet::new();
    let mut pc = 0;
    let mut acc = 0;
    loop {
        if pc >= prog.len() {
            return (true, acc);
        }
        if visited.contains(&pc) {
            return (false, acc);
        }

        visited.insert(pc);

        match prog[pc] {
            Instruction::Acc(n) => { pc += 1; acc += n; },
            Instruction::Jmp(n) => { pc = (pc as isize + n) as usize; },
            Instruction::Nop(_) => { pc += 1; },
        }
    }
}

fn main() {
    let input = input::read_all(8);

    let mut program = Vec::new();
    for line in input.lines() {
        let split: Vec<&str> = line.split(" ").collect();
        let count: isize = split[1].parse().unwrap();
        program.push(match split[0] {
            "acc" => Instruction::Acc(count),
            "jmp" => Instruction::Jmp(count),
            "nop" => Instruction::Nop(count),
            _ => panic!("parse fail"),
        });
    }

    println!("Part 1");
    let (halted, acc) = run(&program);
    assert!(!halted);
    println!("acc = {}", acc);

    println!("Part 2");
    for i in 0..program.len() {
        let mut prog = program.clone();
        match program[i] {
            Instruction::Acc(_) => { continue; },
            Instruction::Jmp(n) => { prog[i] = Instruction::Nop(n); },
            Instruction::Nop(n) => { prog[i] = Instruction::Jmp(n); },
        }
        let (halted, acc) = run(&prog);
        if halted {
            println!("acc = {}", acc);
        }
    }
}
