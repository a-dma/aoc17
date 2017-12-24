#![feature(slice_rotate)]

use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};

const ROUND_TWO_TIMES: usize = 1_000_000_000;

#[derive(Debug)]
enum Move {
    Spin(u32),
    Exchange(u32, u32),
    Partner(char, char),
}

impl Move {
    fn from_input(input: &str) -> Move {
        match input.chars().next().unwrap() {
            's' => {
                let x: u32 = input
                    .chars()
                    .skip(1)
                    .fold(0, |acc, i| (acc * 10) + i.to_digit(10).unwrap());
                Move::Spin(x)
            }

            'x' => {
                let v: Vec<&str> = input.split('/').collect();
                let a = v[0][1..].parse::<u32>().unwrap();
                let b = v[1].parse::<u32>().unwrap();
                Move::Exchange(a, b)
            }

            'p' => {
                let mut it = input.chars().skip(1);
                let a = it.next().unwrap();
                let b = it.nth(1).unwrap();
                Move::Partner(a, b)
            }

            _ => unreachable!(),
        }
    }

    fn do_spin(x: u32, seq: &mut [char]) {
        let len = seq.len();
        seq.rotate(len - x as usize);
    }

    fn do_exchange(a: u32, b: u32, seq: &mut [char]) {
        seq.swap(a as usize, b as usize);
    }

    fn do_partner(a: char, b: char, seq: &mut [char]) {
        let a_pos = seq.iter().position(|&x| x == a).unwrap();
        let b_pos = seq.iter().position(|&x| x == b).unwrap();
        seq.swap(a_pos, b_pos);
    }

    fn execute(&self, seq: &mut [char]) {
        match *self {
            Move::Spin(x) => Move::do_spin(x, seq),
            Move::Exchange(a, b) => Move::do_exchange(a, b, seq),
            Move::Partner(a, b) => Move::do_partner(a, b, seq),
        }
    }
}

fn problem1<R: BufRead>(reader: &mut R) -> String {
    let mut input = String::new();
    reader.read_to_string(&mut input).unwrap();
    let moves = input
        .split(',')
        .map(|x| Move::from_input(x))
        .collect::<Vec<_>>();

    let mut sequence = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p'
    ];

    for move_ in &moves {
        move_.execute(&mut sequence);
    }

    sequence.into_iter().collect()
}

fn problem2<R: BufRead>(reader: &mut R) -> String {
    let mut input = String::new();
    reader.read_to_string(&mut input).unwrap();
    let moves = input
        .split(',')
        .map(|x| Move::from_input(x))
        .collect::<Vec<_>>();

    let mut sequence = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p'
    ];

    let mut sequences: Vec<Vec<char>> = Vec::new();

    let mut found = false;
    while !found {
        for move_ in &moves {
            move_.execute(&mut sequence);
        }

        for s in &sequences {
            if s.iter().cmp(sequence.iter()) == std::cmp::Ordering::Equal {
                println!("cycle at {}", sequences.len());
                found = true;
                break;
            }
        }

        if found {
            break;
        } else {
            sequences.push(sequence.to_owned());
        }
    }

    let modulo = ROUND_TWO_TIMES % sequences.len();

    let mut sequence = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p'
    ];

    for _ in 0..modulo {
        for move_ in &moves {
            move_.execute(&mut sequence);
        }
    }

    sequence.into_iter().collect()
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
