use std::{env, fs, usize};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::{Display, Formatter, Write};
use std::hash::{Hash, Hasher};
use std::ptr::hash;

use crate::Instruction::Single;

mod test;


#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum VarNum {
    Variable(char),
    Number(i64),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Instruction {
    Single(InstructionType, char),
    Double(InstructionType, char, VarNum),
    Unknown,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum InstructionType {
    Input,
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

fn parse_instruction(line: &str) -> Instruction {
    let s: Vec<&str> = line.split(" ").collect();
    let inst = s[0];

    let c1 = s[1].chars().nth(0).unwrap_or('_');
    if inst == "inp" {
        return Instruction::Single(InstructionType::Input, c1);
    }

    let i2 = s[2].parse::<i64>();
    let mut var_num = VarNum::Number(-1);
    if i2.is_ok() {
        var_num = VarNum::Number(i2.unwrap());
    } else {
        var_num = VarNum::Variable(s[2].chars().nth(0).unwrap());
    }

    match inst {
        "add" => Instruction::Double(InstructionType::Add, c1, var_num),
        "mul" => Instruction::Double(InstructionType::Mul, c1, var_num),
        "div" => Instruction::Double(InstructionType::Div, c1, var_num),
        "mod" => Instruction::Double(InstructionType::Mod, c1, var_num),
        "eql" => Instruction::Double(InstructionType::Eql, c1, var_num),
        _ => Instruction::Unknown,
    }
}


fn parse_input(input_file: &str) -> Vec<Instruction> {
    let input = fs::read_to_string(input_file).unwrap();
    input
        .split("\n")
        .map(parse_instruction)
        .collect()
}

fn get_value<'a>(variables_value: &'a Vars, vn: &'a VarNum) -> Option<&'a i64> {
    match vn {
        VarNum::Number(v) => Some(v),
        VarNum::Variable(c) => variables_value.content.get(&c)
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Vars {
    content: HashMap<char, i64>,
}

impl Vars {
    fn new() -> Self {
        Self {
            content: HashMap::new(),
        }
    }
}

impl Hash for Vars {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for i in self.content.keys() {
            i.hash(state);
            self.content.get(i).unwrap().hash(state);
        }
    }
}

struct Executor {
    cache: HashMap<(usize, Vars, Vec<u8>), Vars>,
}

impl Executor {
    fn new() -> Self {
        Executor {
            cache: HashMap::new()
        }
    }

    fn exec_instr(&mut self, instruction: &Instruction, input: i64, mut vars: &mut Vars) -> (Vars, bool) {
        let mut input_read = false;
        match instruction {
            Instruction::Single(_, c) => {
                vars.content.insert(*c, input);
                input_read = true;
            }
            Instruction::Double(it, v, vn) => {
                let var_value = get_value(&vars, &vn);
                if var_value.is_none() {
                    return (vars.clone(), input_read);
                }

                let var_val = *var_value.unwrap();
                let mut e = vars.content.entry(*v).or_insert(0);

                match it {
                    InstructionType::Add => {
                        *e += var_val
                    }
                    InstructionType::Mul => {
                        *e *= var_val
                    }
                    InstructionType::Div => {
                        *e /= var_val
                    }
                    InstructionType::Mod => {
                        *e %= var_val
                    }
                    InstructionType::Eql => {
                        if *e == var_val {
                            *e = 1
                        } else {
                            *e = 0;
                        }
                    }
                    _ => {}
                }
            }
            Instruction::Unknown => {}
        };
        return (vars.clone(), input_read);
    }

    /*
        exec runs the instruction block against the input with the given input variables,
        then it returns the output variables.
    */
    fn exec(&mut self,
            digits: &[u8],
            input_vars: &Vars,
            block: &Vec<Instruction>,
            block_id: usize,
    ) -> Vars {
        // Check if we've already seen this block, with these vars and this input:
        let k = (block_id, input_vars.clone(), Vec::from(digits.clone()));
        if self.cache.contains_key(&k) {
            return self.cache.get(&k).unwrap().clone();
        }

        let mut offset = 0;
        let mut vars = Vars::new();
        let mut input = -1;
        if digits.len() != 0 {
            input = digits[offset] as i64;
        }

        for i in block {
            let (vars_new, input_read) = self.exec_instr(i, input, &mut vars);
            if input_read {
                offset += 1;
                if offset >= digits.len() {
                    input = -1;
                } else {
                    input = digits[offset] as i64;
                }
            }
            vars = vars_new;
        }
        return vars;
    }
}

fn u64_to_digits(mut input: u64) -> Vec<u8> {
    let mut digits: Vec<u8> = Vec::new();
    while input != 0 {
        let digit: u8 = (input % 10) as u8;
        digits.push(digit);
        input -= digit as u64;
        input /= 10;
    }
    digits.reverse();
    digits
}

fn get_blocks(input: &Vec<Instruction>) -> Vec<Vec<Instruction>> {
    let mut blocks: Vec<Vec<Instruction>> = Vec::new();
    let mut v: Vec<Instruction> = Vec::new();
    for i in input {
        match i {
            Instruction::Single(_, _) => {
                v.push(i.clone());
                if v.len() > 0 {
                    blocks.push(v);
                }
                v = Vec::new();
            }
            Instruction::Double(_, _, _) => {
                v.push(i.clone());
            }
            _ => {}
        }
    }
    blocks.push(v);
    blocks
}

fn single_instruction(b: &Instruction) -> bool {
    match b {
        Instruction::Single(_, _) => true,
        _ => false
    }
}

fn part_one(input_file: &str) -> i32 {
    let mut instructions = parse_input(input_file);
    let blocks = get_blocks(&instructions);

    let mut e = Executor::new();
    let len: usize = 14;

    for i in 0..10_u64.pow(len as u32) - 1 {
        let input_value = 10_u64.pow(len as u32) - 1 - i;

        let mut i_digits = u64_to_digits(input_value);
        let mut digits = Vec::new();
        while len > i_digits.len() + digits.len() {
            digits.push(0);
        }
        digits.append(&mut i_digits);

        let mut vars = Vars::new();

        let mut offset = 0;
        for (block_id, b) in blocks.iter().enumerate() {
            let input_count = b.iter().filter(|i| single_instruction(i)).count();
            vars = e.exec(&digits[offset..offset + input_count], &vars, b, block_id);
            offset += input_count;
        }


        if *vars.content.get(&'z').unwrap() == 0 {
            println!("Found it {}!", input_value);
        }
    }

    return 0;
}

fn part_two(input_file: &str) -> u32 {
    0
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!(
            "Invalid arguments provided: please use {} input|sample",
            args[0]
        );
        std::process::exit(1);
    }

    let path = (match args[1].as_str() {
        "sample" => Ok("input/sample.txt"),
        "input" => Ok("input/input.txt"),
        _ => Err("invalid choice"),
    })
        .unwrap();

    println!("Part 1: {}", part_one(path));
    println!("Part 2: {}", part_two(path));
}
