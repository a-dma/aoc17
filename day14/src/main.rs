#![feature(slice_rotate)]

extern crate petgraph;

use petgraph::algo::connected_components;
use petgraph::graphmap::UnGraphMap;

use std::fs::File;
use std::io::{BufReader, BufRead, Seek, SeekFrom};

const STANDARD_SUFFIX: &[u8] = &[17, 31, 73, 47, 23];

fn sparse_hash(list: &mut [u8], lengths: &mut Vec<u8>, rounds: usize) {
    let mut current_position: usize = 0;
    let mut skip = 0;
    let mut total_skip: usize = 0;
    let list_len = list.len();

    if rounds != 1 {
        lengths.extend(STANDARD_SUFFIX);
    };

    for _ in 0..rounds {
        for length in lengths.iter() {
            list.rotate(current_position);
            list[..*length as usize].reverse();
            total_skip += current_position;
            current_position = (*length as usize + skip) % list_len;
            skip = (skip + 1) % list_len;
        }
        total_skip %= list_len;
    }
    list.rotate(list_len - (total_skip % list_len));
}

fn knot_hash(input: &str) -> Vec<u8> {
    let mut list: Vec<u8> = (0..256).map(|x| x as u8).collect();
    let mut lengths: Vec<u8> = input.bytes().collect();

    sparse_hash(&mut list, &mut lengths, 64);

    list.chunks(16)
        .map(|x| x.iter().fold(0, |acc, &y| acc ^ y))
        .collect::<Vec<u8>>()
}

fn problem1<R: BufRead>(reader: &mut R) -> u32 {
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    input.pop();

    let mut used = 0;
    for i in 0..128 {
        let in_string = format!("{}-{}", input, i);
        used += knot_hash(&in_string).iter().fold(
            0,
            |acc, x| acc + x.count_ones(),
        );
    }

    used
}

fn problem2<R: BufRead>(reader: &mut R) -> u32 {
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    input.pop();

    let mut matrix = vec![-5i32; 128 * 128];
    for i in 0..128 {
        let in_string = format!("{}-{}", input, i);
        for v in knot_hash(&in_string).iter().enumerate() {
            for b in 0..8 {
                if (1 << (7 - b)) & *v.1 != 0 {
                    matrix[(i * 128) + (v.0 * 8) + b] = 1;
                } else {
                    matrix[(i * 128) + (v.0 * 8) + b] = 0;
                }
            }
        }
    }

    // Prints the matrix
    // for i in 0..(128 * 128) {
    //     if matrix[i] == 1 {
    //         print!("#");
    //     } else {
    //         print!(".");
    //     }
    //     if (i + 1) % 128 == 0 {
    //         println!("");
    //     }
    // }

    // Logic from Day 12
    let mut graph = UnGraphMap::new();
    for y in 0..128 {
        for x in 0..128 {
            if matrix[(y * 128) + x] == 1 {
                graph.add_edge((y * 128) + x, (y * 128) + x, ());
                if y > 0 && matrix[((y - 1) * 128) + x] == 1 {
                    graph.add_edge((y * 128) + x, ((y - 1) * 128) + x, ());
                }
                if y < 127 && matrix[((y + 1) * 128) + x] == 1 {
                    graph.add_edge((y * 128) + x, ((y + 1) * 128) + x, ());
                }
                if x > 0 && matrix[(y * 128) + (x - 1)] == 1 {
                    graph.add_edge((y * 128) + x, (y * 128) + (x - 1), ());
                }
                if x < 127 && matrix[(y * 128) + (x + 1)] == 1 {
                    graph.add_edge((y * 128) + x, (y * 128) + (x + 1), ());
                }
            }
        }
    }

    connected_components(&graph) as u32
}

const INPUT_FILE: &str = "input.txt";

fn main() {
    let file = File::open(INPUT_FILE).expect(&format!("Input file {} not found", INPUT_FILE));
    let mut br = BufReader::new(file);
    let solution1 = problem1(&mut br);
    println!("solution 1 {}", solution1);
    br.seek(SeekFrom::Start(0)).unwrap();
    let solution2 = problem2(&mut br);
    print!("solution 2 {}", solution2);
}
