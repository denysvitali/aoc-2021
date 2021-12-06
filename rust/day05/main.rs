use std::{env, fs};
use std::collections::HashMap;
use std::fmt::{Debug};

#[derive(Debug)]
struct Line {
    from: (i32, i32),
    to: (i32, i32),
}

struct IntersectionMap {
    content: HashMap<i32, HashMap<i32, i32>> // Y,X, Count
}

impl IntersectionMap {
    pub fn print(&self){
        for i in 0..self.content.len() {
            let has_i = self.content.contains_key(&(i as i32));
            for j in 0..self.content.iter().next().unwrap().1.len()+1 {
                if !has_i  {
                    print!(".");
                    continue
                }
                let has_j = self.content.get(&(i as i32)).unwrap().contains_key(&(j as i32));
                if !has_j {
                    print!(".");
                    continue
                }
                let v = self.content
                    .get(&(i as i32))
                    .unwrap()
                    .get(&(j as i32))
                    .unwrap_or(&0);
                print!("{}", v)
            }
            println!()
        }
    }

    pub fn overlaps(&self) -> i32 {
        self.content
            .iter()
            .map(|x| x.1)
            .map(
                |m|
                    m.iter()
                        .filter(|x|*(x.1) >= 2)
            )
            .count() as i32
    }
}

impl Line {
    pub fn new(from: (i32, i32), to: (i32, i32)) -> Self {
        return Line{
            from,
            to,
        }
    }

    pub fn draw(&self, map: &mut IntersectionMap){
        // Y
        let (mut from_y, mut to_y) = (self.from.1, self.to.1);
        if from_y > to_y {
            let tmp = from_y;
            from_y = to_y;
            to_y = tmp;
        }

        // X
        let (mut from_x, mut to_x) = (self.from.0, self.to.0);
        if from_x > to_x {
            let tmp = from_x;
            from_x = to_x;
            to_x = tmp;
        }

        for i in from_y..to_y+1 {
            if !map.content.contains_key(&i) {
                map.content.insert(i, HashMap::new());
            }
            let map_i = map.content.get_mut(&i).unwrap();
            for j in from_x..to_x+1 {
                if !map_i.contains_key(&j) {
                    map_i.insert(j, 0);
                }
                let prev_value = map_i.get(&j).unwrap().clone();
                map_i.insert(j, prev_value + 1);
            }
        }
    }

    pub fn is_horizontal(&self) -> bool {
        // Y doesn't change
        if self.from.1 == self.to.1 {
            return true
        }
        return false
    }

    pub fn is_vertical(&self) -> bool {
        // X doesn't change
        if self.from.0 == self.to.0 {
            return true
        }
        return false
    }
}

fn parse_coord(input: &str) -> (i32, i32) {
    let v = input
        .split(",")
        .map(|x|x.parse::<i32>().unwrap_or(-1))
        .collect::<Vec<_>>();
    return (v[0], v[1]);
}

fn parse_line(input: &str) -> Line {
    let v = input
        .split(" -> ")
        .map(parse_coord)
        .collect::<Vec<_>>();
    Line::new(v[0], v[1])
}

fn parse_lines(input: &str) -> Vec<Line> {
    input
        .split("\n")
        .map(parse_line)
        .collect::<Vec<_>>()
}


fn part_one(input_file: &str) -> i32 {
    let content = fs::read_to_string(input_file).unwrap();
    let lines = parse_lines(&content);

    let mut map : IntersectionMap = IntersectionMap{ content: HashMap::new() };
    for line in lines {
        if line.is_horizontal() || line.is_vertical() {
            line.draw(&mut map)
        }
        println!("{:?}", line)
    }

    map.print();
    return map.overlaps();
}

fn part_two(input_file: &str) -> i32 {
    let _content = fs::read_to_string(input_file).unwrap();
    return -1;
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
