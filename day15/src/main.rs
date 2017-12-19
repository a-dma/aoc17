use std::fs::File;
use std::io::{BufReader, BufRead, Seek, SeekFrom};

const A_FACTOR: u64 = 16_807;
const B_FACTOR: u64 = 48_271;
const MODULO: u64 = 2_147_483_647;

const PROBLEM1_ROUNDS: usize = 40_000_000;
const PROBLEM2_ROUNDS: usize = 5_000_000;

fn problem1<R: BufRead>(reader: &mut R) -> u32 {
    let mut input = reader.lines().take(2);
    let a_start: u32 = input
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .last()
        .map(|x| x.parse())
        .unwrap()
        .unwrap();
    let b_start: u32 = input
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .last()
        .map(|x| x.parse())
        .unwrap()
        .unwrap();

    let mut a_previous: u64 = u64::from(a_start);
    let mut b_previous: u64 = u64::from(b_start);
    let mut matches = 0;

    for _ in 0..PROBLEM1_ROUNDS {
        let a_value = (a_previous * A_FACTOR) % MODULO;
        let b_value = (b_previous * B_FACTOR) % MODULO;

        if a_value as u16 == b_value as u16 {
            matches += 1;
        }

        a_previous = a_value;
        b_previous = b_value;
    }

    matches
}

fn problem2<R: BufRead>(reader: &mut R) -> u32 {
    let mut input = reader.lines().take(2);
    let a_start: u32 = input
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .last()
        .map(|x| x.parse())
        .unwrap()
        .unwrap();
    let b_start: u32 = input
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .last()
        .map(|x| x.parse())
        .unwrap()
        .unwrap();

    let mut a_previous: u64 = u64::from(a_start);
    let mut b_previous: u64 = u64::from(b_start);

    let mut a_values = Vec::new();
    let mut b_values = Vec::new();

    while a_values.len() != PROBLEM2_ROUNDS || b_values.len() != PROBLEM2_ROUNDS {
        let a_value = (a_previous * A_FACTOR) % MODULO;
        let b_value = (b_previous * B_FACTOR) % MODULO;

        if a_values.len() < PROBLEM2_ROUNDS && a_value % 4 == 0 {
            a_values.push(a_value as u16);
        }

        if b_values.len() < PROBLEM2_ROUNDS && b_value % 8 == 0 {
            b_values.push(b_value as u16);
        }

        a_previous = a_value;
        b_previous = b_value;
    }

    a_values
        .iter()
        .zip(b_values.iter())
        .filter(|&(x, y)| *x == *y)
        .count() as u32
}

const INPUT_FILE: &str = "input.txt";

fn main() {
    let file = File::open(INPUT_FILE).expect(&format!("Input file {} not found", INPUT_FILE));
    let mut br = BufReader::new(file);
    let solution = problem1(&mut br);
    println!("solution 1 {}", solution);
    br.seek(SeekFrom::Start(0)).unwrap();
    let solution2 = problem2(&mut br);
    println!("solution 2 {}", solution2);
}
