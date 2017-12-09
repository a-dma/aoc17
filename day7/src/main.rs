use std::fs::File;
use std::io::{BufReader, BufRead, Seek, SeekFrom};
use std::collections::HashMap;

#[derive(Debug)]
struct Node {
    weight: u32,
    children: HashMap<String, Node>,
}

#[derive(Debug)]
struct Line {
    name: String,
    weight: u32,
    children: Option<Vec<String>>,
}

impl Line {
    fn from_input(input: String) -> Self {
        let head_tail = input.split(" -> ").collect::<Vec<&str>>();
        let name_weight = head_tail[0].split_whitespace().collect::<Vec<&str>>();

        let name = name_weight[0];
        let weight = name_weight[1][1..name_weight[1].len() - 1].parse::<u32>();

        let children = if head_tail.len() == 1 {
            None
        } else {
            Some(head_tail[1].split(", ").map(|x| x.to_owned()).collect::<Vec<String>>())
        };

        Line {
            name: name.to_string(),
            weight: weight.unwrap(),
            children: children,
        }
    }
}

fn problem1<R: BufRead>(reader: &mut R) -> String {
    let mut forest = HashMap::<String, u32>::new();
    let mut children = Vec::<Vec<String>>::new();

    for line in reader.lines() {
        let current_line = Line::from_input(line.unwrap());

        forest.insert(current_line.name, current_line.weight);
        if current_line.children.is_some() {
            children.push(current_line.children.unwrap());
        }
    }

    for children_list in children {
        for child in children_list {
            forest.remove(&child);
        }
    }

    forest.keys().next().unwrap().to_owned()
}

fn get_children_weight(weight: &HashMap<String, u32>,
                       children: &HashMap<String, Vec<String>>,
                       node: &str)
                       -> u32 {
    match children.get(node) {
        Some(vec) => {
            let mut weights = 0;
            for child in vec {
                weights += get_children_weight(weight, children, child);
            }

            weights + weight[node]
        }
        None => weight[node],
    }
}

fn problem2<R: BufRead>(reader: &mut R) -> u32 {
    let mut forest = HashMap::<String, u32>::new();
    let mut children = HashMap::<String, Vec<String>>::new();

    for line in reader.lines() {
        let current_line = Line::from_input(line.unwrap());

        forest.insert(current_line.name.to_owned(), current_line.weight);
        if current_line.children.is_some() {
            children.insert(current_line.name, current_line.children.unwrap());
        }
    }

    let mut solution = u32::max_value();
    for node in forest.keys() {
        let node_children = children.get(node);
        if node_children.is_some() {
            let mut children_weights = Vec::new();
            for child in node_children.unwrap() {
                children_weights.push(get_children_weight(&forest, &children, child));
            }

            let pred = children_weights[0];
            let (l, r): (Vec<u32>, Vec<u32>) = children_weights.iter().partition(|&&x| x == pred);
            if l.len() == 1 {
                let pos = children_weights.iter().position(|&x| x == l[0]).unwrap();
                let candidate = (forest[&node_children.unwrap()[pos]] as i32 +
                                 (r[0] as i32 -
                                  l[0] as i32)) as u32;
                if candidate < solution {
                    solution = candidate;
                }
            }
            if r.len() == 1 {
                let pos = children_weights.iter().position(|&x| x == r[0]).unwrap();
                let candidate = (forest[&node_children.unwrap()[pos]] as i32 +
                                 (r[0] as i32 -
                                  l[0] as i32)) as u32;
                if candidate < solution {
                    solution = candidate;
                }
            }
        }
    }
    solution
}

const INPUT_FILE: &'static str = "input.txt";

fn main() {
    let file = File::open(INPUT_FILE).expect(&format!("Input file {} not found", INPUT_FILE));
    let mut br = BufReader::new(file);
    let solution1 = problem1(&mut br);
    println!("solution 1 {}", solution1);
    br.seek(SeekFrom::Start(0)).unwrap();
    let solution2 = problem2(&mut br);
    println!("solution 2 {}", solution2);
}
