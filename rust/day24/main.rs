use std::{env, fs, usize};
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::{Display, Formatter, Write};
use std::ptr::hash;

use crate::Instruction::Single;

mod test;


#[derive(Debug, Clone)]
enum VarNum {
    Variable(char),
    Number(i64),
}

#[derive(Debug, Clone)]
enum Instruction {
    Single(InstructionType, char),
    Double(InstructionType, char, VarNum),
    Unknown,
}

#[derive(Debug, Clone)]
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

fn get_value<'a>(variables_value: &'a HashMap<char, i64>, vn: &'a VarNum) -> Option<&'a i64> {
    match vn {
        VarNum::Number(v) => Some(v),
        VarNum::Variable(c) => variables_value.get(&c)
    }
}

/*
    exec runs the instructions against the input, then it returns a boolean
    indicating whether the input was valid or not
 */
fn exec(mut input: u64, len: u32, instructions: &Vec<Instruction>) -> HashMap<char, i64> {
    let mut offset = 0;
    let mut digits: Vec<i32> = Vec::new();

    while input != 0 {
        digits.push((input % 10) as i32);
        input /= 10;
    }

    let to_add = len - (digits.len() as u32);
    for _ in 0..to_add {
        digits.push(0);
    }

    digits.reverse();

    let mut variables_value: HashMap<char, i64> = HashMap::new();
    for i in instructions {
        match i {
            Instruction::Single(it, c) => {
                variables_value.insert(*c, digits[offset].into());
                offset += 1;
            }
            Instruction::Double(it, v, vn) => {
                let var_value = get_value(&variables_value, vn);
                if var_value.is_none() {
                    continue;
                }

                let var_val = *var_value.unwrap();
                let mut e = variables_value.entry(*v).or_insert(0);

                match it {
                    InstructionType::Add => {
                        *e += var_val;
                    }
                    InstructionType::Mul => {
                        *e *= var_val;
                    }
                    InstructionType::Div => {
                        *e /= var_val;
                    }
                    InstructionType::Mod => {
                        *e %= var_val;
                    }
                    InstructionType::Eql => {
                        if *e == var_val {
                            *e = 1;
                        } else {
                            *e = 0;
                        }
                    }
                    _ => {}
                };
            }
            Instruction::Unknown => {}
        };
    }
    return variables_value;
}

fn part_one(input_file: &str) -> u64 {
    let mut input = parse_input(input_file);

    let input_counts = input.iter()
        .map(|i| match i {
            Single(it, v) => {
                match it {
                    InstructionType::Input => Some(v),
                    _ => None,
                }
            }
            _ => None,
        })
        .filter(Option::is_some)
        .map(|x| *x.unwrap())
        .collect::<Vec<char>>();

    println!("Input: {:?}", input_counts.len());

    let mut blocks : Vec<Vec<Instruction>> = Vec::new();
    let mut v: Vec<Instruction> = Vec::new();
    for i in input {
        match i {
            Instruction::Single(_, _) => {
                if v.len() > 0 {
                    blocks.push(v.clone());
                }
                v = Vec::new();
            }
            Instruction::Double(_, _, _) => {
                v.push(i);
            }
            _ => {}
        }
    }
    blocks.push(v);

    println!("blocks={}", blocks.len());

    let max: u64 = (10 as u64).pow(input_counts.len() as u32) as u64 - 1;
    let min: u64 = (10 as u64).pow(input_counts.len() as u32 - 1) as u64;

    for i in min..max + 1 {
        let exec_result = exec(i, input_counts.len() as u32, &input);
        if *exec_result.get(&'z').unwrap() == 0 {
            println!("{}", i);
        }
        if i % 10000 == 0 {
            println!("{}", i);
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
