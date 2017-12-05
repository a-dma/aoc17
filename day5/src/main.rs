use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, BufRead, Seek, SeekFrom};

fn problem<R: BufRead>(reader: &mut R, crazy_treshold: i32) -> i32 {
    let mut the_maze = Vec::new();

    for line in reader.lines() {
        let value = line.unwrap().parse::<i32>().unwrap();
        the_maze.push(value);
    }

    let mut pos = 0i32;
    let mut jumps = 0;
    let size = the_maze.len();
    loop {
        let new_pos = the_maze[pos as usize] + pos;
        if the_maze[pos as usize] >= crazy_treshold {
            the_maze[pos as usize] -= 1;
        }
        else {
            the_maze[pos as usize] += 1;
        }

        jumps += 1;

        if new_pos as usize >= size || new_pos < 0 {
            break jumps;
        }
        else {
            pos = new_pos;
        }
    }
}

const INPUT_FILE: &'static str = "input.txt";

fn main() {
    let file = File::open(INPUT_FILE).expect(&format!("Input file {} not found", INPUT_FILE));
    let mut br = BufReader::new(file);
    let solution1 = problem(&mut br, i32::max_value());
    br.seek(SeekFrom::Start(0)).unwrap();
    let solution2 = problem(&mut br, 3);
    println!("{} {}", solution1 , solution2);
}
