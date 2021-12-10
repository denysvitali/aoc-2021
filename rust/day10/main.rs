use std::{env, fs};
use std::collections::{HashMap, VecDeque};
use std::fmt::{Display, Formatter, Write};

use crate::SyntaxError::{Corrupted, Incomplete};
use crate::Token::*;

mod test;

const DEBUG: bool = false;

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
enum Token {
    OpnRound,
    ClsRound,
    OpnSquare,
    ClsSquare,
    OpnCurly,
    ClsCurly,
    OpnAng,
    ClsAng,
    Unknown,
}

impl Token {
    fn is_open(&self) -> bool {
        match self {
            OpnAng => true,
            OpnRound => true,
            OpnSquare => true,
            OpnCurly => true,
            _ => false,
        }
    }

    fn closed_by(&self) -> Token {
        match self {
            OpnAng => ClsAng,
            OpnRound => ClsRound,
            OpnSquare => ClsSquare,
            OpnCurly => ClsCurly,
            _ => Unknown,
        }
    }

    fn inverse(&self) -> Token {
        match self {
            OpnAng => ClsAng,
            OpnRound => ClsRound,
            OpnSquare => ClsSquare,
            OpnCurly => ClsCurly,
            ClsAng => OpnAng,
            ClsRound => OpnRound,
            ClsSquare => OpnSquare,
            ClsCurly => OpnCurly,
            _ => Unknown,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OpnAng => f.write_char('<'),
            OpnSquare => f.write_char('['),
            OpnCurly => f.write_char('{'),
            OpnRound => f.write_char('('),
            ClsAng => f.write_char('>'),
            ClsSquare => f.write_char(']'),
            ClsCurly => f.write_char('}'),
            ClsRound => f.write_char(')'),
            _ => f.write_char('?')
        }
    }
}

fn identify_token(c: char) -> Token {
    match c {
        '(' => OpnRound,
        ')' => ClsRound,
        '[' => OpnSquare,
        ']' => ClsSquare,
        '{' => OpnCurly,
        '}' => ClsCurly,
        '<' => OpnAng,
        '>' => ClsAng,
        _ => Unknown
    }
}

fn parse_lines(input: &str) -> Vec<Vec<Token>> {
    input
        .split("\n")
        .map(|l| l.chars().map(identify_token).collect::<Vec<Token>>())
        .collect::<Vec<_>>()
}

#[derive(Debug, PartialEq)]
enum SyntaxError {
    Incomplete(Vec<Token>),
    Corrupted(Token),
}

fn check_line(line: &Vec<Token>) -> SyntaxError {
    let mut stack: VecDeque<Token> = VecDeque::new();
    for t in line {
        if t.is_open() {
            stack.push_back(t.clone());
        } else {
            // Check if matching closing
            if stack.len() == 0 {
                return Incomplete(vec![]);
            }
            let back = stack.pop_back().unwrap();
            let closed_by = back.closed_by();

            if *t != closed_by {
                // Syntax error!
                if DEBUG {
                    println!("Expected {} but found {} instead", closed_by, t);
                }
                return Corrupted(t.clone());
            }
        }
    }

    return Incomplete(stack.iter().map(|x| *x).rev().collect::<Vec<Token>>());
}

fn part_one(input_file: &str) -> i32 {
    let contents = fs::read_to_string(input_file).unwrap();
    let entries = parse_lines(&contents);

    let mut score_map: HashMap<Token, u32> = HashMap::new();
    score_map.insert(ClsRound, 3);
    score_map.insert(ClsSquare, 57);
    score_map.insert(ClsCurly, 1197);
    score_map.insert(ClsAng, 25137);

    let mut token_score: HashMap<Token, u32> = HashMap::new();
    entries.iter().map(check_line)
        .filter(|e| match e {
            Corrupted(_) => true,
            _ => false
        })
        .map(|x| match x {
            Corrupted(n) => n,
            _ => Unknown
        })
        .for_each(
            |t| {
                token_score.insert(t.clone(), 1 + *token_score.get(&t).unwrap_or(&0));
            }
        );

    token_score.iter()
        .map(|(k, v)| score_map.get(&k).unwrap() * v)
        .sum::<u32>() as i32
}

fn part_two(input_file: &str) -> u64 {
    let contents = fs::read_to_string(input_file).unwrap();
    let entries = parse_lines(&contents);

    let mut score_map: HashMap<Token, u64> = HashMap::new();
    score_map.insert(ClsRound, 1);
    score_map.insert(ClsSquare, 2);
    score_map.insert(ClsCurly, 3);
    score_map.insert(ClsAng, 4);

    let inc = entries.iter().map(check_line)
        .filter(|e| match e {
            Incomplete(_) => true,
            _ => false
        })
        .map(|x| match x {
            Incomplete(i) => i,
            _ => vec![]
        })
        .map(|x| x.iter().map(|y| y.inverse()).collect())
        .collect::<Vec<Vec<Token>>>();


    let mut results = inc.iter()
        .map(|x| {
            let mut res: u64 = 0;
            for t in x {
                res = 5 * res + score_map.get(t).unwrap();
            }
            return res;
        })
        .collect::<Vec<u64>>();

    results.sort();
    results.reverse();
    *results.get(results.len() / 2).unwrap() as u64
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
    }).unwrap();

    println!("Part 1: {}", part_one(path));
    println!("Part 2: {}", part_two(path));
}