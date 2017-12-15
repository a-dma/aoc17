use std::fs::File;
use std::io::{BufReader, BufRead, Seek, SeekFrom};
use std::collections::HashMap;

fn triangle(x: i32, amp: i32) -> u32 {
    if amp == 0 {
        0
    } else {
        (amp - i32::abs(x % (2 * amp) - amp)) as u32
    }
}

fn problem1<R: BufRead>(reader: &mut R) -> u32 {
    let mut firewall = HashMap::new();
    let mut max_depth = 0;
    for line in reader.lines() {
        let val: Vec<i32> = line.unwrap()
            .split(": ")
            .take(2)
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        firewall.insert(val[0], val[1]);

        if val[0] > max_depth {
            max_depth = val[0];
        }
    }

    let mut severity = 0;
    for i in 0..max_depth + 1 {
        if let Some(range) = firewall.get(&i) {
            if triangle(i as i32, *range - 1) == 0 {
                severity += i * *range;
            }
        }
    }

    severity as u32
}

fn problem2<R: BufRead>(reader: &mut R) -> u32 {
    let mut firewall = HashMap::new();
    let mut max_depth = 0;
    for line in reader.lines() {
        let val: Vec<i32> = line.unwrap()
            .split(": ")
            .take(2)
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        firewall.insert(val[0], val[1]);

        if val[0] > max_depth {
            max_depth = val[0];
        }
    }

    let mut delay = 0;
    let mut caught: bool;
    loop {
        caught = false;
        for i in delay..max_depth + 1 + delay {
            if let Some(range) = firewall.get(&(i - delay)) {
                if triangle(i as i32, *range - 1) == 0 {
                    caught = true;
                    delay += 1;
                    break;
                }
            }
        }

        if !caught {
            break;
        }
    }

    delay as u32
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
