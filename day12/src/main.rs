extern crate petgraph;

use petgraph::algo::connected_components;
use petgraph::graphmap::UnGraphMap;
use petgraph::visit::Dfs;
use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug)]
struct Line {
    name: u32,
    children: Vec<u32>,
}

impl Line {
    fn from_input(input: &str) -> Self {
        let head_tail = input.split(" <-> ").collect::<Vec<&str>>();

        let name = head_tail[0].parse::<u32>().unwrap();
        let children = head_tail[1]
            .split(", ")
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        Line {
            name: name,
            children: children,
        }
    }
}

fn problem<R: BufRead>(reader: &mut R) -> (u32, u32) {
    let mut graph = UnGraphMap::new();

    for line in reader.lines() {
        let current_line = Line::from_input(&line.unwrap());
        for child in current_line.children {
            graph.add_edge(current_line.name, child, ());
        }
    }

    let mut dfs = Dfs::new(&graph, 0);
    let mut nodes = 0;
    while let Some(_) = dfs.next(&graph) {
        nodes += 1;
    }

    (nodes, connected_components(&graph) as u32)
}

const INPUT_FILE: &str = "input.txt";

fn main() {
    let file = File::open(INPUT_FILE).expect(&format!("Input file {} not found", INPUT_FILE));
    let mut br = BufReader::new(file);
    let solution = problem(&mut br);
    println!("solution 1 {}", solution.0);
    println!("solution 2 {}", solution.1);
}
