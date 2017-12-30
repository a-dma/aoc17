use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::collections::VecDeque;

struct Program {
    registers: [i64; 26],
    memory: Vec<Instruction>,
    pc: usize,
    queue: VecDeque<i64>,
    sent: usize,
    last_sent: i64,
    solution_1: i64,
}

impl Program {
    fn new(memory: Vec<Instruction>, id: i64) -> Program {
        let mut cpu = Program {
            registers: [0; 26],
            memory,
            pc: 0,
            queue: VecDeque::new(),
            sent: 0,
            last_sent: 0,
            solution_1: 0,
        };
        cpu.registers[(b'p' - b'a') as usize] = id;
        cpu
    }

    fn run_dual(cpu1: &mut Program, cpu2: &mut Program) {
        cpu1.run(cpu2, true);
    }

    fn run(&mut self, other: &mut Program, first: bool) {
        while self.pc < self.memory.len() {
            let advance = self.memory[self.pc].clone().execute(self, other);
            if !advance {
                if !first {
                    break;
                }
                other.run(self, false);
                if !self.memory[self.pc].clone().execute(self, other) {
                    break;
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Value {
    Reg(usize),
    Const(i64),
}

impl Value {
    fn eval(&self, cpu: &Program) -> i64 {
        match *self {
            Value::Reg(r) => cpu.registers[r],
            Value::Const(c) => c,
        }
    }

    fn reg(s: &str) -> usize {
        (s.as_bytes()[0] - b'a') as usize
    }
}

impl FromStr for Value {
    type Err = String;

    fn from_str(s: &str) -> Result<Value, String> {
        if s.len() == 1 && s.as_bytes()[0] >= b'a' {
            Ok(Value::Reg(Value::reg(s)))
        } else {
            s.parse()
                .map(Value::Const)
                .map_err(|e| format!("cannot parse {}: {}", s, e))
        }
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Snd(Value),
    Set(usize, Value),
    Add(usize, Value),
    Mul(usize, Value),
    Mod(usize, Value),
    Rcv(usize),
    Jgz(Value, Value),
}

impl Instruction {
    fn execute(&self, cpu: &mut Program, other: &mut Program) -> bool {
        cpu.pc += 1;
        match *self {
            Instruction::Snd(ref v) => {
                let v = v.eval(cpu);
                cpu.last_sent = v;
                other.queue.push_back(v);
                cpu.sent += 1;
            }
            Instruction::Set(ref r, ref v) => {
                cpu.registers[*r] = v.eval(cpu);
            }
            Instruction::Add(ref r, ref v) => {
                cpu.registers[*r] += v.eval(cpu);
            }
            Instruction::Mul(ref r, ref v) => {
                cpu.registers[*r] *= v.eval(cpu);
            }
            Instruction::Mod(ref r, ref v) => {
                cpu.registers[*r] %= v.eval(cpu);
                assert!(cpu.registers[*r] >= 0);
            }
            Instruction::Rcv(ref r) => {
                if cpu.solution_1 == 0 && cpu.registers[*r] != 0 {
                    cpu.solution_1 = cpu.last_sent;
                }
                if let Some(v) = cpu.queue.pop_front() {
                    cpu.registers[*r] = v;
                } else {
                    cpu.pc -= 1;
                    return false;
                }
            }
            Instruction::Jgz(ref t, ref o) => {
                if t.eval(cpu) > 0 {
                    cpu.pc = (cpu.pc as i64 + o.eval(cpu) - 1) as usize;
                }
            }
        }
        true
    }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Instruction, String> {
        let words = s.split_whitespace().collect::<Vec<_>>();
        Ok(match words[0] {
            "snd" => Instruction::Snd(words[1].parse().unwrap()),
            "set" => Instruction::Set(Value::reg(words[1]), words[2].parse().unwrap()),
            "add" => Instruction::Add(Value::reg(words[1]), words[2].parse().unwrap()),
            "mul" => Instruction::Mul(Value::reg(words[1]), words[2].parse().unwrap()),
            "mod" => Instruction::Mod(Value::reg(words[1]), words[2].parse().unwrap()),
            "rcv" => Instruction::Rcv(Value::reg(words[1])),
            "jgz" => Instruction::Jgz(words[1].parse().unwrap(), words[2].parse().unwrap()),
            _ => {
                return Err(format!("cannot parse instruction {}", words[0]));
            }
        })
    }
}

fn problem<R: BufRead>(reader: &mut R) -> (i64, usize) {
    let instructions: Vec<Instruction> = reader
        .lines()
        .map(|x| Instruction::from_str(&x.unwrap()).unwrap())
        .collect();

    let mut program_1 = Program::new(instructions.clone(), 0);
    let mut program_2 = Program::new(instructions, 1);
    Program::run_dual(&mut program_1, &mut program_2);

    (program_1.solution_1, program_2.sent)
}

const INPUT_FILE: &str = "input.txt";

fn main() {
    let file = File::open(INPUT_FILE).expect(&format!("Input file {} not found", INPUT_FILE));
    let mut br = BufReader::new(file);
    let (solution1, solution2) = problem(&mut br);
    println!("solution 1 {}", solution1);
    println!("solution 2 {}", solution2);
}
