mod test;

use std::{env, fs};
use std::collections::{HashMap, HashSet, VecDeque};

extern crate termion;
use termion::style;

const DEBUG : bool = false;
const CARDINAL: &'static[(i32,i32); 4] = &[(1, 0), (0, -1), (-1, 0), (0, 1)];

fn parse_lines(input: &str) -> Vec<Vec<u32>> {
    input
        .split("\n")
        .map(|l|l.chars().map(|c|c.to_digit(10).unwrap()).collect::<Vec<u32>>())
        .collect::<Vec<_>>()
}

fn adj(x: usize, y: usize, v: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut near : Vec<(usize, usize)> = vec![];

    let max_x = v.get(0).unwrap().len() as i32;
    let max_y = v.len() as i32;

    for c in CARDINAL {
        let x_t = c.0 + (x as i32);
        let y_t = c.1 + (y as i32);

        if x_t < 0 || y_t < 0 || x_t >= (max_x as i32) || y_t >= max_y {
            continue
        }
        near.push((x_t as usize, y_t as usize));
    }
    near
}

fn adj_values(x: usize, y: usize, v: &Vec<Vec<u32>>) -> Vec<u32> {
    adj(x, y, v)
        .iter()
        .map(|(x,y)| *v.get(*y).unwrap().get(*x).unwrap())
        .collect::<Vec<u32>>()
}

fn get_low_points(entries: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut low_points : Vec<(usize, usize)> = Vec::new();
    for (y, row) in entries.iter().enumerate() {
        for (x, el) in row.iter().enumerate() {

            if el < adj_values(x, y, entries).iter().min().unwrap() {
                // Low point
                low_points.push((x, y));
            }
        }
    }
    low_points
}


fn part_one(input_file: &str) -> i64 {
    let contents = fs::read_to_string(input_file).unwrap();
    let entries = parse_lines(&contents);
    get_low_points(&entries)
        .iter()
        .map(|(x, y)|*entries.get(*y).unwrap().get(*x).unwrap())
        .map(|x|x+1).sum::<u32>() as i64
}

fn highlight_points(v: &Vec<Vec<u32>>, points: &Vec<(usize, usize)>) {
    for (y, row) in v.iter().enumerate() {
        for (x, el) in row.iter().enumerate() {
            if points.contains(&(x, y)) {
                print!("{}{}{}", style::Bold, el, style::Reset);
            } else {
                print!("{}", el);
            }
        }
        println!();
    }
}

fn get_basin(
    entries: &Vec<Vec<u32>>,
    q: &mut VecDeque<(usize, usize)>,
    visited: &mut HashMap<usize, bool>
) -> HashSet<(usize, usize)> {
    let mut points : HashSet<(usize, usize)> = HashSet::new();
    let max_x = entries.get(0).unwrap().len();

    while !q.is_empty() {
        let front = q.front().unwrap();
        let x = front.0;
        let y = front.1;
        q.pop_front();

        if *entries.get(y).unwrap().get(x).unwrap() != 9 {
            let idx = y * max_x + x;
            if *visited.get(&idx).unwrap() {
                // Visited, ignoring
                continue
            }
            visited.insert(idx, true);
            points.insert((x, y));
            let the_adj = adj(x, y, &entries);
            for el in &the_adj {
                if *entries.get(el.1).unwrap().get(el.0).unwrap() != 9 {
                    let el_idx = el.1 * max_x + el.0;
                    if *visited.get(&el_idx).unwrap() {
                        // Visited, ignoring
                        continue
                    }
                    points.insert((el.0, el.1));
                    q.push_back((el.0, el.1));
                }
            }
        }
    }
    points
}

fn part_two(input_file: &str) -> i32 {
    let contents = fs::read_to_string(input_file).unwrap();
    let entries = parse_lines(&contents);

    // Using flooding fill
    let mut visited : HashMap<usize, bool> = HashMap::new();

    let max_x = entries.get(0).unwrap().len();
    let max_y = entries.len();
    for i in 0..max_x * max_y {
        visited.insert(i, false);
    }

    let mut q : VecDeque<(usize, usize)> = VecDeque::new();
    let mut basin_sizes : Vec<usize> = Vec::new();

    for (x,y) in get_low_points(&entries) {
        q.push_back((x, y));
        let points = get_basin(&entries, &mut q, &mut visited);
        basin_sizes.push(points.len());
        if DEBUG {
            highlight_points(&entries, &points.iter().map(|x| *x).collect::<Vec<_>>());
            println!()
        }
    }

    basin_sizes.sort();
    basin_sizes.reverse();
    basin_sizes.iter()
        .take(3)
        .fold(1, |acc, x| acc * (*x as i32))
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