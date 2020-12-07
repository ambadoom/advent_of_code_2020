use advent2020::input;

use regex::Regex;
use petgraph::graphmap::{
    DiGraphMap,
};
use petgraph::algo;
use petgraph::prelude::*;

fn main() {
    let our_bag = "shiny gold";
    let input = input::read_all(7);

    let re_start = Regex::new(r"^([a-z ]*) bags contain").unwrap();
    let re_none = Regex::new(r" no other bags.").unwrap();
    let re_bag = Regex::new(r" (\d) ([a-z ]*) bags?[.,]").unwrap();

    let mut graph: DiGraphMap<&str, u32> = DiGraphMap::new();

    for line in input.lines() {
        let src = re_start.captures(line).unwrap().get(1).unwrap().as_str();
        print!("{:?} ->", src);
        graph.add_node(src);

        if re_none.is_match(line) {
            continue;
        }

        let dsts = re_bag.captures_iter(line);

        for dst in dsts {
            let count = dst.get(1).unwrap().as_str().parse().unwrap();
            let name = dst.get(2).unwrap().as_str();
            print!(" {}x{:?}", count, name);

            graph.add_node(src);
            graph.add_edge(src, name, count);
        }
        println!();
    }

    assert!(graph.contains_node(our_bag));

    let mut count = 0;
    for node in graph.nodes() {
        if node == our_bag { continue; }

        if algo::has_path_connecting(&graph, node, our_bag, None) {
            count += 1;
        }
    }

    println!("Part 1");
    println!("{}", count);

    fn explore(graph: &DiGraphMap<&str, u32>, id: &str) -> usize {
        let mut out = 0;
        for edge in graph.edges(id) {
            //println!("Edge {} {:?}", edge.weight(), edge.target());
            out += *edge.weight() as usize;
            out += (*edge.weight() as usize) * explore(graph, edge.target());
        }
        out
    }

    println!("Part 2");
    println!("{}", explore(&graph, our_bag));

}
