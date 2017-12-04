use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, BufRead, Seek, SeekFrom};

fn problem1<R: BufRead>(reader: &mut R) -> i32 {
    let mut valid_passphrases = 0;

    for line in reader.lines() {
        let mut set = HashSet::new();
        let mut count = 0;
        let line = line.unwrap();
        for word in line.split_whitespace() {
            set.insert(word.to_owned());
            count += 1;
        }

        if set.len() == count {
            valid_passphrases += 1;
        }
    }

    valid_passphrases
}

fn problem2<R: BufRead>(reader: &mut R) -> i32 {
    let mut valid_passphrases = 0;

    for line in reader.lines() {
        let mut set = HashSet::new();
        let mut count = 0;
        let line = line.unwrap();
        for word in line.split_whitespace() {
            let mut sorted_word = word.chars().collect::<Vec<char>>();
            sorted_word.sort();
            set.insert(sorted_word);
            count += 1;
        }

        if set.len() == count {
            valid_passphrases += 1;
        }
    }

    valid_passphrases
}

const INPUT_FILE: &'static str = "input.txt";

fn main() {
    let file = File::open(INPUT_FILE).expect(&format!("Input file {} not found", INPUT_FILE));
    let mut br = BufReader::new(file);
    let solution1 = problem1(&mut br);
    br.seek(SeekFrom::Start(0)).unwrap();
    let solution2 = problem2(&mut br);
    println!("{} {}", solution1, solution2);

}
