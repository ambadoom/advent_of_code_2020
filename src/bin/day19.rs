use advent2020::input;

use std::collections::HashMap;

use regex::Regex;

enum Rule<'a> {
    Leaf(&'a str),
    Node(Vec<Vec<usize>>),
}

type Rules<'a> = HashMap<usize, Rule<'a>>;

fn partial_matches<'a>(rules: &'a Rules, rule: &'a Rule, s: &'a str) -> Box<dyn Iterator<Item=&'a str> + 'a> {
    match rule {
        Rule::Leaf(s_) => { 
            if let Some(out) = s.strip_prefix(s_) {
                Box::new(std::iter::once(out))
            } else {
                Box::new(std::iter::empty())
            }
        },
        Rule::Node(alts) => { 
            Box::new(alts.iter().map(move |seq| {
                let mut out: Box<dyn Iterator<Item=&'a str> + 'a> = Box::new(std::iter::once(s));
                for item in seq {
                    out = Box::new(
                        out.map(move |s_| {
                            partial_matches(rules, &rules[item], s_)
                        }).flatten()
                    );
                }
                out
            }).flatten())
        },
    }
}

fn matches(rules: &Rules, rule: &Rule, s: &str) -> bool {
    partial_matches(rules, rule, s)
        .filter(|s| s.is_empty())
        .next()
        .is_some()
}

fn parse_rule<'a, 'b>(rules: &'a mut Rules<'b>, rule: &'b str) where 'b: 'a {
    let re_leaf = Regex::new(r#""(.)""#).unwrap();

    let mut sp = rule.split(':');
    let id: usize = sp.next().unwrap().parse().unwrap();
    let rule_s = sp.next().unwrap();

    if let Some(caps) = re_leaf.captures(rule_s) {
        rules.insert(id, Rule::Leaf(caps.get(1).unwrap().as_str()));
    } else {
        let rule: Vec<Vec<usize>> = rule_s
            .split('|')
            .map(|r| r.trim().split(' ').map(|n| n.parse().unwrap()).collect())
            .collect(); 
        rules.insert(id, Rule::Node(rule));
    }
}

fn main() {
    let input = input::read_all(19);
    let mut lines = input.lines();

    let mut rules = HashMap::new();

    for line in &mut lines {
        if line.is_empty() { break; }
        parse_rule(&mut rules, line);
    }

    let mut strings = Vec::new();

    for line in &mut lines {
        strings.push(line);
    }

    println!("Part 1");
    let count = strings.iter().filter(|s| matches(&rules, &rules[&0], s)).count();
    println!("{}", count);

    println!("Part 2");
    parse_rule(&mut rules, "8: 42 | 42 8");
    parse_rule(&mut rules, "11: 42 31 | 42 11 31");
    let count = strings.iter().filter(|s| matches(&rules, &rules[&0], s)).count();
    println!("{}", count);

}
