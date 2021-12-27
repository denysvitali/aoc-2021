use std::{env, fs, usize};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

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
    let var_num;
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
    input.split("\n").map(parse_instruction).collect()
}

fn get_value<'a>(variables_value: &'a Vars, vn: &'a VarNum) -> Option<&'a i64> {
    match vn {
        VarNum::Number(v) => Some(v),
        VarNum::Variable(c) => variables_value.content.get(&c),
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
            cache: HashMap::new(),
        }
    }

    fn exec_instr(
        &mut self,
        instruction: &Instruction,
        input: i64,
        vars: &mut Vars,
    ) -> (Vars, bool) {
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
                let e = vars.content.entry(*v).or_insert(0);

                match it {
                    InstructionType::Add => *e += var_val,
                    InstructionType::Mul => *e *= var_val,
                    InstructionType::Div => *e /= var_val,
                    InstructionType::Mod => *e %= var_val,
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
    fn exec(
        &mut self,
        digits: &[u8],
        input_vars: &Vars,
        block: &Vec<Instruction>,
        block_id: usize,
    ) -> Vars {

        // // Print instructions
        // for b in block {
        //     println!("{:?}", b);
        // }

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
        self.cache.insert(k, vars.clone());
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
        _ => false,
    }
}

fn do_calc(a: i32,
           b: i32,
           c: i32,
           d: i32,
           w: i32,
           mut x: i32,
           mut y: i32,
           mut z: i32)
           -> (i32, i32, i32)
{
    x = x + z;
    x %= a;
    z /= 1;
    x += b;
    if x == w {
        x = 1;
    } else {
        x = 0;
    }
    if x == 0 {
        x = 1
    } else {
        x = 0;
    }
    y *= 0;
    y += c;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += d;
    y *= x;
    z += y;
    return (x, y, z);
}

fn get_number(i: &Instruction) -> i32 {
    match i {
        Instruction::Double(it, a, b) => {
            return match b {
                VarNum::Variable(v) => {
                    -1
                }
                VarNum::Number(v) => {
                    *v as i32
                }
            };
        }
        _ => { 0 }
    }
}

fn part_one(input_file: &str) -> i32 {
    let instructions = parse_input(input_file);
    let blocks = get_blocks(&instructions);
    let number_size = 14;

    for i in 0..10_u64.pow(number_size) {
        let i = 10_u64.pow(number_size) - i - 1;
        let mut converted_digits = u64_to_digits(i);
        let mut digits = Vec::new();

        while (digits.len() + converted_digits.len()) < number_size as usize {
            digits.push(0);
        }
        digits.append(&mut converted_digits);

        let mut x = 0;
        let mut y = 0;
        let mut z = 0;
        let mut a = 0;
        let mut b = 0;
        let mut c = 0;
        let mut d = 0;
        let mut offset = 0;

        for i in &blocks {
            let w;
            if digits.len() == 0 {
                w = 0;
            } else {
                w = digits[offset] as i32;
            }
            offset += 1;
            a = get_number(&i[3]);
            b = get_number(&i[5]);
            c = get_number(&i[9]);
            d = get_number(&i[15]);

            let (x_1, y_1, z_1) = do_calc(a, b, c, d, w, x, y, z);
            x = x_1;
            y = y_1;
            z = z_1;
        }

        if z == 0 {
            println!("found it! {}", i);
            break
        }
    }

    return 0;
}

fn part_two(_input_file: &str) -> u32 {
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
