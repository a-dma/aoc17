use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::str::FromStr;

#[derive(Debug)]
struct Line {
    register_name: String,
    operation: Operation,
    amount: i32,
    condition_register: String,
    condition: Condition,
    condition_amount: i32,
}

#[derive(Debug)]
enum Operation {
    Increment,
    Decrement,
}

impl std::str::FromStr for Operation {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "inc" {
            Ok(Operation::Increment)
        } else if s == "dec" {
            Ok(Operation::Decrement)
        } else {
            Err("Not an operation!!!".to_string())
        }
    }
}

impl Operation {
    fn eval(&self, l: i32, r: i32) -> i32 {
        match *self {
            Operation::Increment => l + r,
            Operation::Decrement => l - r,
        }
    }
}

#[derive(Debug)]
enum Condition {
    Gt,
    Lt,
    Ge,
    Le,
    Eq,
    Ne,
}

impl std::str::FromStr for Condition {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ">" => Ok(Condition::Gt),
            "<" => Ok(Condition::Lt),
            ">=" => Ok(Condition::Ge),
            "<=" => Ok(Condition::Le),
            "==" => Ok(Condition::Eq),
            "!=" => Ok(Condition::Ne),
            _ => Err("Not a condition!!!".to_string()),
        }
    }
}

impl Condition {
    fn eval(&self, l: i32, r: i32) -> bool {
        match *self {
            Condition::Gt => l > r,
            Condition::Lt => l < r,
            Condition::Ge => l >= r,
            Condition::Le => l <= r,
            Condition::Eq => l == r,
            Condition::Ne => l != r,
        }
    }
}

impl Line {
    fn from_input(input: String) -> Self {
        let mut words = input.split_whitespace();
        let register_name = words.next().unwrap();
        let operation = Operation::from_str(words.next().unwrap()).unwrap();
        let amount = i32::from_str(words.next().unwrap()).unwrap();
        words.next();
        let condition_register = words.next().unwrap();
        let condition = Condition::from_str(words.next().unwrap()).unwrap();
        let condition_amount = i32::from_str(words.next().unwrap()).unwrap();

        Line {
            register_name: register_name.to_owned(),
            operation: operation,
            amount: amount,
            condition_register: condition_register.to_owned(),
            condition: condition,
            condition_amount: condition_amount,
        }
    }
}

fn problem<R: BufRead>(reader: &mut R) -> (i32, i32) {
    let mut registers = HashMap::<String, i32>::new();
    let mut max_value = 0;
    for line in reader.lines() {
        let current_line = Line::from_input(line.unwrap());

        registers.entry(current_line.condition_register.to_owned()).or_insert(0);

        if !current_line.condition
            .eval(registers[&current_line.condition_register],
                  current_line.condition_amount) {
            continue;
        }

        let register_name = current_line.register_name;
        registers.entry(register_name.to_owned()).or_insert(0);

        let current_value = registers[&register_name];
        let new_value = current_line.operation
            .eval(current_value, current_line.amount);
        registers.insert(register_name, new_value);

        if new_value > max_value {
            max_value = new_value;
        }
    }

    return (*(registers.values().max().unwrap()), max_value);
}


const INPUT_FILE: &'static str = "input.txt";

fn main() {
    let file = File::open(INPUT_FILE).expect(&format!("Input file {} not found", INPUT_FILE));
    let mut br = BufReader::new(file);
    let (solution1, solution2) = problem(&mut br);
    println!("solution 1 {}", solution1);
    println!("solution 2 {}", solution2);
}
