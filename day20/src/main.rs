extern crate regex;

use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::collections::HashMap;
use regex::Regex;

fn apply(to: &mut (i64, i64, i64), der: &(i64, i64, i64)) {
    to.0 += der.0;
    to.1 += der.1;
    to.2 += der.2;
}

fn dist(p: &(i64, i64, i64)) -> u64 {
    (p.0.abs() + p.1.abs() + p.2.abs()) as u64
}

fn problem1<R: BufRead>(reader: &mut R) -> usize {
    let re = Regex::new(r"[^\d-]+").unwrap();
    let mut particles = reader
        .lines()
        .map(|x| {
            let ns = re.split(&x.unwrap()[3..])
                .map(|w| w.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            (
                (ns[0], ns[1], ns[2]),
                (ns[3], ns[4], ns[5]),
                (ns[6], ns[7], ns[8]),
            )
        })
        .collect::<Vec<_>>();

    let max_acceleration = particles.iter().map(|&p| dist(&p.2)).max().unwrap();
    for _ in 0..max_acceleration * max_acceleration {
        for p in &mut particles {
            apply(&mut p.1, &p.2);
            apply(&mut p.0, &p.1);
        }
    }

    particles
        .iter()
        .enumerate()
        .min_by_key(|&(_, p)| dist(&p.0))
        .unwrap()
        .0
}

fn problem2<R: BufRead>(reader: &mut R) -> usize {
    let re = Regex::new(r"[^\d-]+").unwrap();
    let mut particles = reader
        .lines()
        .map(|x| {
            let ns = re.split(&x.unwrap()[3..])
                .map(|w| w.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            (
                (ns[0], ns[1], ns[2]),
                (ns[3], ns[4], ns[5]),
                (ns[6], ns[7], ns[8]),
            )
        })
        .collect::<Vec<_>>();

    let max_acceleration = particles.iter().map(|&p| dist(&p.2)).max().unwrap();
    for _ in 0..max_acceleration * max_acceleration {
        let mut pos = HashMap::new();
        for p in &mut particles {
            apply(&mut p.1, &p.2);
            apply(&mut p.0, &p.1);
            *pos.entry(p.0).or_insert(0) += 1;
        }
        particles.retain(|p| pos[&p.0] < 2);
    }

    particles.len()
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
