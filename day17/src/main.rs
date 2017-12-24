use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};

const ROUND_TWO_TIMES: usize = 50_000_000;

fn problem1<R: BufRead>(reader: &mut R) -> u32 {
    let mut input = String::new();
    reader.read_to_string(&mut input).unwrap();
    input.pop();
    let value = input.parse::<u32>().unwrap();

    let mut sequence = Vec::with_capacity(2017);
    sequence.push(0);
    let mut current_position = 1;

    for i in 1..2018 {
        let position = (current_position + value) as usize % sequence.len();
        sequence.insert(position + 1, i);
        current_position = (position + 1) as u32;
        //println!("{:?}, cp {}", sequence, current_position);
    }

    let p = sequence.iter().position(|&x| x == 2017 as usize).unwrap();
    sequence[p + 1] as u32
}

fn problem2<R: BufRead>(reader: &mut R) -> u32 {
    let mut input = String::new();
    reader.read_to_string(&mut input).unwrap();
    input.pop();
    let value = input.parse::<u32>().unwrap();

    let mut sequence = Vec::with_capacity(2017);
    sequence.push(0);

    let mut marker = 0;
    let mut position = 0;
    for i in 1..ROUND_TWO_TIMES {
        position = ((position + value) as usize  % i + 1) as u32;
        if position == 1 {
            marker = i;
        }
    }

    marker as u32
}

const INPUT_FILE: &str = "input.txt";

fn main() {
    let file = File::open(INPUT_FILE).expect(&format!("Input file {} not found", INPUT_FILE));
    let mut br = BufReader::new(file);
    let solution1 = problem1(&mut br);
    println!("solution 1 {}", solution1);
    br.seek(SeekFrom::Start(0)).unwrap();
    let solution2 = problem2(&mut br);
    println!("solution 2 {}", solution2);
}
