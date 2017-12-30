use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn next(&self) -> Self {
        match *self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn oppoaite(&self) -> Self {
        match *self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

fn get_next_direction(current: &Direction, last: &Direction) -> Direction {
    let new = current.next();
    if new == last.oppoaite() {
        new.next()
    } else {
        new
    }
}

fn problem<R: BufRead>(reader: &mut R) -> (String, u32) {
    let mut diagram = Vec::new();
    for line in reader.lines() {
        let chars = line.unwrap().chars().collect::<Vec<_>>();
        diagram.push(chars);
    }

    let initial_position = diagram[0].iter().position(|&x| x == '|').unwrap();
    let mut current_direction = Direction::South;
    let mut last_direction = Direction::South;
    let mut pos = (0, initial_position);
    let mut letters = Vec::new();
    let max_x = diagram[0].len() - 1;
    let max_y = diagram.len() - 1;
    let mut switches = 0;
    let mut steps = 1;

    while switches != 4 {
        match current_direction {
            Direction::North => {
                if pos.0 > 1 && diagram[pos.0 - 1][pos.1] != ' ' {
                    pos.0 -= 1;
                    if diagram[pos.0][pos.1].is_alphabetic() {
                        letters.push(diagram[pos.0][pos.1])
                    }
                    switches = 0;
                    last_direction = Direction::North;
                    steps += 1;
                } else {
                    current_direction = get_next_direction(&current_direction, &last_direction);
                    switches += 1;
                }
            }
            Direction::East => {
                if pos.1 < max_x && diagram[pos.0][pos.1 + 1] != ' ' {
                    pos.1 += 1;
                    if diagram[pos.0][pos.1].is_alphabetic() {
                        letters.push(diagram[pos.0][pos.1])
                    }
                    switches = 0;
                    last_direction = Direction::East;
                    steps += 1;
                } else {
                    current_direction = get_next_direction(&current_direction, &last_direction);
                    switches += 1;
                }
            }
            Direction::South => {
                if pos.0 < max_y && diagram[pos.0 + 1][pos.1] != ' ' {
                    pos.0 += 1;
                    if diagram[pos.0][pos.1].is_alphabetic() {
                        letters.push(diagram[pos.0][pos.1])
                    }
                    switches = 0;
                    last_direction = Direction::South;
                    steps += 1;
                } else {
                    current_direction = get_next_direction(&current_direction, &last_direction);
                    switches += 1;
                }
            }
            Direction::West => {
                if pos.1 > 1 && diagram[pos.0][pos.1 - 1] != ' ' {
                    pos.1 -= 1;
                    if diagram[pos.0][pos.1].is_alphabetic() {
                        letters.push(diagram[pos.0][pos.1])
                    }
                    switches = 0;
                    last_direction = Direction::West;
                    steps += 1;
                } else {
                    current_direction = get_next_direction(&current_direction, &last_direction);
                    switches += 1;
                }
            }
        }
    }

    (letters.into_iter().collect(), steps)
}

const INPUT_FILE: &str = "input.txt";

fn main() {
    let file = File::open(INPUT_FILE).expect(&format!("Input file {} not found", INPUT_FILE));
    let mut br = BufReader::new(file);
    let (solution1, solution2) = problem(&mut br);
    println!("solution 1 {}", solution1);
    println!("solution 2 {}", solution2);
}
