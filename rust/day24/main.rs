extern crate crossbeam;

use std::{env, fs, thread, usize};
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::collections::hash_map::DefaultHasher;
use std::fmt::{Display, Formatter, Write};
use std::hash::{Hash, Hasher};
use std::process::exit;
use std::ptr::hash;
use std::thread::JoinHandle;

use crossbeam::thread::ScopedJoinHandle;

use crate::Instruction::Single;

mod test;
mod a;


#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum VarNum {
    Variable(char),
    Number(i64),
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Instruction {
    Single(InstructionType, char),
    Double(InstructionType, char, VarNum),
    Unknown,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
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

#[derive(Clone, Debug, Eq, PartialEq)]
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
    cache: HashSet<(usize, i64)>,
}

impl Executor {
    fn new() -> Self {
        Executor {
            cache: HashSet::new()
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
    ) -> Vars {
        let mut offset = 0;
        let mut vars = input_vars.clone();
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

    fn solve(&mut self,
             blocks: &Vec<Vec<Instruction>>,
             block_id: usize,
             mut original_digits: Vec<u8>,
             v: Vars
    ) -> Option<i32> {
        if block_id == blocks.len() {
            let z = *v.content.get(&'z').unwrap_or(&-1);
            if z == 0 {
                return Some(0);
            }
            return None
        }

        for mut digit in 0..10 {
            let mut digits = original_digits.clone();
            digits.push(digit);
            let z = *v.content.get(&'z').unwrap_or(&i64::MIN);
            if self.cache.contains(&(block_id+1, z)) {
                continue
            }
            let v = self.exec(&[digit], &v, &blocks[block_id]);
            let s = self.solve(blocks, block_id + 1, digits, v);
            if s.is_none() {
                self.cache.insert((block_id, z));
            } else {
                println!("Found!");
                return Some(s.unwrap());
            }
        }
        None
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
                if v.len() > 0 {
                    blocks.push(v);
                }
                v = Vec::new();
                v.push(i.clone());
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
    let res = e.solve(&blocks, 0, vec![], Vars::new());
    println!("res={:?}", res);
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
