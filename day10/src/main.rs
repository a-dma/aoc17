#![feature(slice_rotate)]

use std::fs::File;
use std::io::{BufReader, BufRead, Seek, SeekFrom};
use std::str::FromStr;

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

fn problem1<R: BufRead>(reader: &mut R) -> u32 {
    let mut list: Vec<u8> = (0..256).map(|x| x as u8).collect();
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    input.pop();
    let mut lengths: Vec<u8> = input.split(",")
        .map(|x| u8::from_str(x).unwrap())
        .collect();

    sparse_hash(&mut list, &mut lengths, 1);

    return list[0] as u32 * list[1] as u32;
}

fn problem2<R: BufRead>(reader: &mut R) -> Vec<u8> {
    let mut list: Vec<u8> = (0..256).map(|x| x as u8).collect();
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    input.pop();
    let mut lengths: Vec<u8> = input.bytes().collect();

    sparse_hash(&mut list, &mut lengths, 64);

    let hash = list.chunks(16)
        .map(|x| x.iter().fold(0, |acc, &y| acc ^ y))
        .collect::<Vec<u8>>();
    return hash;
}


const INPUT_FILE: &'static str = "input.txt";

fn main() {
    let file = File::open(INPUT_FILE).expect(&format!("Input file {} not found", INPUT_FILE));
    let mut br = BufReader::new(file);
    let solution1 = problem1(&mut br);
    println!("solution 1 {}", solution1);
    br.seek(SeekFrom::Start(0)).unwrap();
    let solution2 = problem2(&mut br);
    print!("solution 2 ");
    for i in solution2 {
        print!("{:02x}", i);
    }
    println!();
}
