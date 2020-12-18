use advent2020::input;

enum Problem {
    Num(i64),
    Add(Box<Problem>, Box<Problem>),
    Mul(Box<Problem>, Box<Problem>)
}

fn strip_parens(mut s: &str) -> &str {
    s = s.trim();
    let mut nest = 0;
    for (i,c) in s.char_indices() {
        if c == '(' { nest += 1; }
        if c == ')' { nest -= 1; }
        if nest == 0 && i+1 != s.len() {
            return s;
        }
    }
    if let Some(s) = s.strip_prefix('(') {
        if let Some(s) = s.strip_suffix(')') {
            return strip_parens(s);
        }
    }
    s
}

fn find_nest(s: &str, c: char) -> Option<usize> {
    assert!(c != ')');
    assert!(c != '(');
    let mut nest = 0;
    for (i,cs) in s.char_indices() {
        if cs == '(' { nest += 1; }
        if cs == ')' { nest -= 1; }
        if nest == 0 && cs == c {
            return Some(i);
        }
    }
    return None;
}

fn find_nest_rev(s: &str, c: char) -> Option<usize> {
    assert!(c != ')');
    assert!(c != '(');
    let mut nest = 0;
    for (i,cs) in s.char_indices().rev() {
        if cs == '(' { nest += 1; }
        if cs == ')' { nest -= 1; }
        if nest == 0 && cs == c {
            return Some(i);
        }
    }
    return None;
}

fn parse1(mut s: &str) -> Problem {
    s = strip_parens(s);

    if let Ok(n) = s.parse::<i64>() {
        return Problem::Num(n);
    }

    let addpos = find_nest_rev(s, '+');
    let mulpos = find_nest_rev(s, '*');
    if addpos.is_none() && mulpos.is_none() {
        panic!("Parse fail: input is not a number and does not contain valid operator");
    } else if addpos.unwrap_or(0) > mulpos.unwrap_or(0) {
        Problem::Add(
            Box::new(parse1(&s[0..addpos.unwrap()])),
            Box::new(parse1(&s[addpos.unwrap()+1..]))
        )
    } else {
        Problem::Mul(
            Box::new(parse1(&s[0..mulpos.unwrap()])),
            Box::new(parse1(&s[mulpos.unwrap()+1..]))
        )
    }
}

fn parse2(mut s: &str) -> Problem {
    s = strip_parens(s);

    if let Ok(n) = s.parse::<i64>() {
        return Problem::Num(n);
    }

    if let Some(mulpos) = find_nest(s,'*') {
        Problem::Mul(
            Box::new(parse2(&s[0..mulpos])),
            Box::new(parse2(&s[mulpos+1..]))
        )
    } else if let Some(addpos) = find_nest(s, '+') {
        Problem::Add(
            Box::new(parse2(&s[0..addpos])),
            Box::new(parse2(&s[addpos+1..]))
        )
    } else {
        panic!("Parse fail: input is not a number and does not contain valid operator");
    }
}

fn solve(p: &Problem) -> i64 {
    match p {
        Problem::Num(i) => *i,
        Problem::Add(a, b) => solve(a) + solve(b),
        Problem::Mul(a, b) => solve(a) * solve(b)
    }
}

fn main() {
    let input = input::read_all(18);

    println!("Part 1");
    let sum: i64 = input.lines().map(|line| solve(&parse1(line))).sum();
    println!("{}", sum);

    println!("Part 2");
    let sum: i64 = input.lines().map(|line| solve(&parse2(line))).sum();
    println!("{}", sum);
    
}
