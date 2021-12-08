mod test;

use std::{env, fs};
use std::collections::{HashMap, HashSet};

fn parse_lines(input: &str) -> Vec<Vec<&str>> {
    input
        .split("\n")
        .map(|l|l.split(" | ").collect::<Vec<&str>>())
        .collect::<Vec<_>>()
}

fn recognize_value(input: &str) -> Option<i8> {
    return match input.len() {
        2 => Some(1),
        3 => Some(7),
        4 => Some(4),
        7 => Some(8),
        _ => None
    }
}

fn do_recognition(signal_pattern: &str, output: &str) -> i32 {
    let mut mapped : HashMap<i8, String> = HashMap::new();
    let mut candidates : HashMap<char, HashSet<char>> = HashMap::new();

    for e in signal_pattern.split(" ").chain(output.split(" ")) {
        let rv = recognize_value(e);
        if rv.is_some(){
            mapped.insert(rv.unwrap(), e.to_string());
        }
    }

    for e in vec!['a', 'b', 'c', 'd', 'e', 'f'] {
        candidates.insert(e, HashSet::new());
    }

    let one = mapped.get(&1).unwrap().chars().collect::<Vec<char>>();
    let four = mapped.get(&4).unwrap().chars().collect::<Vec<char>>();
    
    for e in signal_pattern.split(" ").chain(output.split(" ")) {
        if e.len() == 6 {
            // Check if it's a 6, 9 or 0
            let mut remaining_chars = e.chars().filter(|x| !one.contains(x)).collect::<String>();
            if remaining_chars.len() == 5 {
                // This is a 6
                mapped.insert(6, e.to_string());
                continue;
            }

            // Compare with 4
            remaining_chars  = e.chars().filter(|x| !four.contains(x)).collect::<String>();
            match remaining_chars.len() {
                2 => mapped.insert(9, e.to_string()),
                3 => mapped.insert(0, e.to_string()),
                _ => panic!("This should never happen: {}", remaining_chars.len())
            };
        }
    }

    let six = mapped.get(&6).unwrap().chars().collect::<Vec<char>>();

    for e in signal_pattern.split(" ") {
        if e.len() == 5 {
            // 2, 3, 5
            let remaining_chars = e.chars().filter(|x| !one.contains(x)).collect::<String>();
            if remaining_chars.len() == 3 {
                mapped.insert(3, e.to_string());
                continue
            }

            // 2 or 5?
            let remaining_chars = e.chars().filter(|x| six.contains(x)).collect::<String>();
            match remaining_chars.len() {
                5 => mapped.insert(5, e.to_string()),
                4 => mapped.insert(2, e.to_string()),
                _ => panic!("This should never happen {}", remaining_chars.len())
            };
        }
    }


    let mut inverse_map : HashMap<String, i8> = HashMap::new();
    for i in 0..10 {
        let mut chars = mapped.get(&i).unwrap().chars().collect::<Vec<char>>();
        chars.sort();
        inverse_map.insert(chars.iter().collect::<String>(), i);
    }

    

    return output.split(" ")
        .map(|x|{
            let mut chars = x.chars().collect::<Vec<char>>();
            chars.sort();
            chars.iter().collect::<String>()
        })
        .map(|x|*inverse_map.get(&x).unwrap())
        .collect::<Vec<i8>>()
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, x| acc + (*x.1 as i32) * (10 as i32).pow(x.0 as u32))
}


fn part_one(input_file: &str) -> i32 {
    let contents = fs::read_to_string(input_file).unwrap();
    let entries = parse_lines(&contents);

    let mut char_map: HashMap<i8, i32> = HashMap::new();

    for entry in &entries {
        let _signal_pattern = entry.get(0).unwrap();
        let output = entry.get(1).unwrap();

        let zero = 0;
        output
            .split(" ")
            .map(|x|recognize_value(x))
            .filter(|x| x.is_some())
            .for_each(|x|{
                let c = x.unwrap();
                char_map.insert(c, char_map.get(&c).unwrap_or(&zero) + 1);});
    }

    return char_map.iter().map(|(_k,v)|v).sum::<i32>();
}

fn part_two(input_file: &str) -> i32 {
    let contents = fs::read_to_string(input_file).unwrap();
    let entries = parse_lines(&contents);


    entries.iter()
        .map(|x|do_recognition(x.get(0).unwrap(), x.get(1).unwrap()))
        .sum::<i32>()
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
