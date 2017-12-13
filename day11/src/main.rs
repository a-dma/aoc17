use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug)]
struct Point {
    // NOTE(adma): x = 0, y = 1, z = 2
    coord: [i32; 3],
    farthest: u32,
}

impl Point {
    fn change_position(&mut self, direction: &str) {
        match direction {
            "n" => {
                self.coord[1] += 1;
                self.coord[2] += -1;
            }
            "ne" => {
                self.coord[0] += 1;
                self.coord[2] += -1;
            }
            "se" => {
                self.coord[0] += 1;
                self.coord[1] += -1;
            }
            "s" => {
                self.coord[1] += -1;
                self.coord[2] += 1;
            }
            "sw" => {
                self.coord[0] += -1;
                self.coord[2] += 1;
            }
            "nw" => {
                self.coord[0] += -1;
                self.coord[1] += 1;
            }
            _ => unreachable!(),
        }

        let dist = self.get_distance_from_origin();
        if dist > self.farthest {
            self.farthest = dist;
        }
    }

    fn get_distance_from_origin(&self) -> u32 {
        self.coord.iter().map(|&x| x.abs()).max().unwrap() as u32
    }
}

fn problem<R: BufRead>(reader: &mut R) -> (u32, u32) {
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    input.pop();
    let directions: Vec<&str> = input.split(",")
        .collect();

    let mut point = Point {
        coord: [0, 0, 0],
        farthest: 0,
    };
    for direction in directions {
        point.change_position(direction);
    }

    (point.get_distance_from_origin(), point.farthest)
}

const INPUT_FILE: &'static str = "input.txt";

fn main() {
    let file = File::open(INPUT_FILE).expect(&format!("Input file {} not found", INPUT_FILE));
    let mut br = BufReader::new(file);
    let solution = problem(&mut br);
    println!("solution 1 {}", solution.0);
    println!("solution 2 {}", solution.1);
}
