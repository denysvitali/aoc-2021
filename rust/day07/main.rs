mod test;

use std::{env, fs};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};


fn parse_lines(input: &str) -> Vec<i32> {
    input
        .split(",")
        .map(|x|x.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}


fn part_one(input_file: &str) -> i32 {
    let contents = fs::read_to_string(input_file).unwrap();
    let mut pos = parse_lines(&contents);
    println!("len(pos)={}", pos.len());
    let mut common: HashMap<i32, i8> = HashMap::new();
    for i in &pos {
        let zero = 0;
        let c = common.get(&i).unwrap_or(&zero);
        common.insert(*i, c + 1);
    }
    pos.sort();

    let mid_point = pos.len() / 2;
    let mean = pos.get(mid_point).unwrap();

    // Fuel = distance from mean
    let fuel = pos.iter().map(|x| (x-mean).abs()).sum();
    return fuel
}

fn part_two(input_file: &str) -> i32 {
    let contents = fs::read_to_string(input_file).unwrap();
    let mut pos = parse_lines(&contents);
    let mut common: HashMap<i32, i8> = HashMap::new();
    for i in &pos {
        let zero = 0;
        let c = common.get(&i).unwrap_or(&zero);
        common.insert(*i, c + 1);
    }
    pos.sort();

    let max_pos = pos.iter().max().unwrap();
    let mut prev_fuel = -1;
    for i in 0..max_pos+1 {
        let current_fuel : i32 = pos.iter().map(|x| val_at_x(*x, i)).sum();
        if prev_fuel == -1 {
            // Ignore
        } else if current_fuel > prev_fuel  {
            return prev_fuel;
        }
        prev_fuel = current_fuel;
    }
    return -1;
}

fn val_at_x(x: i32, i: i32) -> i32 {
    let n = (x-i).abs();
    return (n * (n+1))/2;
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
