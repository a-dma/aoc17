use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::fs::File;
use std::io::{BufReader, BufRead, Seek, SeekFrom};

fn get_hash<T: Hash>(item: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    item.hash(&mut hasher);
    hasher.finish()
}

fn problem<R: BufRead>(reader: &mut R, go_again: bool) -> u32 {
    let mut go_again = go_again;
    let mut content = String::new();
    reader.read_line(&mut content);

    let mut memory_banks = content.split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    println!("{:?}", memory_banks);

    loop {
        let mut iterations = 0;
        let mut hashes = Vec::new();
        hashes.push(get_hash(&memory_banks));
        loop {
            let id = memory_banks.iter()
                .enumerate()
                .rev()
                .max_by_key(|&(_, item)| item)
                .unwrap()
                .0;
            let max = memory_banks[id];
            memory_banks[id] = 0;

            let size = memory_banks.len();
            for i in (id + 1)..(max as usize + id + 1) {
                memory_banks[i % size] += 1;
            }

            iterations += 1;

            let hash = get_hash(&memory_banks);
            if hashes.iter().find(|&&x| x == hash).is_some() {
                if !go_again {
                    return iterations;
                } else {
                    break;
                }
            }

            hashes.push(hash);
        }
        go_again = false;
    }
}

const INPUT_FILE: &'static str = "input.txt";

fn main() {
    let file = File::open(INPUT_FILE).expect(&format!("Input file {} not found", INPUT_FILE));
    let mut br = BufReader::new(file);
    let solution1 = problem(&mut br, false);
    println!("solution 1 {}", solution1);
    br.seek(SeekFrom::Start(0)).unwrap();
    let solution2 = problem(&mut br, true);
    println!("solution 2 {}", solution2);
}
