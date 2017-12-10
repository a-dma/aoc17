use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

fn problem1<R: BufRead>(reader: &mut R) -> (u32, u32) {
    let mut score = 0;
    let mut crap = 0;

    let mut is_garbage = false;
    let mut skip_next = false;
    let mut depth = 0;

    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    for char in input.chars() {
        if skip_next {
            skip_next = false;
            continue;
        }

        match char {
            '{' => {
                if !is_garbage {
                    depth += 1;
                } else {
                    crap += 1;
                }
            }
            '}' => {
                if !is_garbage {
                    score += depth;
                    depth -= 1;
                } else {
                    crap += 1;
                }
            }
            '<' => {
                if !is_garbage {
                    is_garbage = true;
                } else {
                    crap += 1;
                }
            }
            '>' => is_garbage = false,
            '!' => skip_next = true,
            _ => {
                if is_garbage {
                    crap += 1;
                }
            }
        }
    }

    return (score, crap);
}


const INPUT_FILE: &'static str = "input.txt";

fn main() {
    let file = File::open(INPUT_FILE).expect(&format!("Input file {} not found", INPUT_FILE));
    let mut br = BufReader::new(file);
    let (solution1, solution2) = problem1(&mut br);
    println!("solution 1 {}", solution1);
    println!("solution 2 {}", solution2);
}
