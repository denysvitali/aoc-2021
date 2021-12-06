mod test;

use std::{env, fs};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

struct LanternFish {
    timer: i8,
}

impl LanternFish {
    pub fn new(timer: i8) -> Self {
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

fn parse_lines(input: &str) -> Vec<i8> {
    input
        .split(",")
        .map(|x|x.parse::<i8>().unwrap())
        .collect::<Vec<_>>()
}

fn get_population(input_file: &str, generation: i32) -> i64 {
    let content = fs::read_to_string(input_file).unwrap();
    let mut population = parse_lines(&content);
    let mut count: HashMap<i8, i64> = HashMap::new();

    for i in 0..10 {
        count.insert(i, 0);
    }

    for i in &population {
        let p = *count.get(&i).unwrap();
        count.insert(*i, p+1);
    }

    for i in 1..(generation+1) {
        let z = *count.get(&0).unwrap();
        for j in 0..9 {
            let next: i8 = j+1;
            count.insert(j, *count.get(&next).unwrap());
        }
        let eight = 8;
        let six = 6;
        count.insert(eight, z);
        count.insert(six, count.get(&six).unwrap() + z);

    }

    return count.iter().map(|(k,v)| v).sum::<i64>();
}


fn part_one(input_file: &str) -> i64 {
    return get_population(input_file, 80);
}

fn part_two(input_file: &str) -> i64 {
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
