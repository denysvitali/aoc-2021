mod test;

use std::{env, fs};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

struct LanternFish {
    timer: i32,
}

impl LanternFish {
    pub fn new(timer: i32) -> Self {
        return LanternFish {
            timer
        }
    }
    pub(crate) fn tick(&mut self) -> Option<LanternFish> {
        if self.timer == 0 {
            self.timer = 6;
            return Some(LanternFish::new(8))
        }
        self.timer -= 1;
        return None
    }
}

impl Debug for LanternFish {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "LF({})", self.timer)
    }
}

fn parse_lines(input: &str) -> Vec<LanternFish> {
    input
        .split(",")
        .map(|x|x.parse::<i32>().unwrap())
        .map(|x|LanternFish::new(x))
        .collect::<Vec<_>>()
}

fn get_population(input_file: &str, generation: i32) -> i32 {
    let content = fs::read_to_string(input_file).unwrap();
    let mut population = parse_lines(&content);

    for _ in 1..(generation)+1 {
        let mut new_fish: Vec<LanternFish>  = Vec::new();
        for el in &mut population {
            match el.tick() {
                Some(l) => new_fish.push(l),
                None => {}
            }
        }
        population.append(&mut new_fish);
    }

    return population.len() as i32;
}


fn part_one(input_file: &str) -> i32 {
    return get_population(input_file, 80);
}

fn part_two(input_file: &str) -> i32 {
    return get_population(input_file, 256);
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
