use std::{env, fs};
use std::collections::HashMap;

mod test;


fn get_template_and_pairs(input_file: &str) -> (String, HashMap<String, char>) {
    let contents = fs::read_to_string(input_file).unwrap();
    let s = contents.split("\n\n").collect::<Vec<&str>>();
    let mut hm: HashMap<String, char> = HashMap::new();

    for l in s[1].split("\n") {
        let s2 = l.split(" -> ").collect::<Vec<&str>>();
        hm.insert(String::from(s2[0]), String::from(s2[1]).chars().next().unwrap());
    }

    (String::from(s[0]), hm)
}

fn solve(t: String, p: HashMap<String, char>, steps: usize) -> i64 {
    let mut char_frequency: HashMap<char, i64> = HashMap::new();
    let mut pairs: HashMap<String, i64> = HashMap::new();

    for c in t.chars() {
        *char_frequency.entry(c).or_insert(0)+=1;
    }

    let mut prev_char: Option<char> = None;
    for c in t.chars() {
        if prev_char.is_none() {
            prev_char = Some(c);
            continue;
        }
        let pair = String::from_iter(vec![prev_char.unwrap(), c]);
        prev_char = Some(c);
        pairs.entry(pair).or_insert(1);
    }

    for _ in 1..(steps + 1) {
        for (k, v) in pairs.clone() {
            let c = *p.get(&k).unwrap();
            *char_frequency.entry(c).or_insert(0)+=v;

            // Remove this pair
            *pairs.entry(k.clone()).or_insert(0) -= v;

            // Add new pair
            let p1 = String::from_iter(vec![k.chars().nth(0).unwrap(), c]);
            let p2 = String::from_iter(vec![c, k.chars().nth(1).unwrap()]);
            *pairs.entry(p1).or_insert(0)+=v;
            *pairs.entry(p2).or_insert(0)+=v;
        }
    }

    let (_, mc_count) = char_frequency.iter().max_by_key(|c| c.1).unwrap();
    let (_, lc_count) = char_frequency.iter().min_by_key(|c| c.1).unwrap();
    mc_count - lc_count
}

fn part_one(input_file: &str) -> i64 {
    let (t, p) = get_template_and_pairs(input_file);
    solve(t, p, 10)
}

fn part_two(input_file: &str) -> i64 {
    let (t, p) = get_template_and_pairs(input_file);
    solve(t, p, 40)
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
