use std::{env, fs};
use std::collections::{HashSet, VecDeque};
use std::fmt::{Display, Formatter};

mod test;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Coord {
    x: u32,
    y: u32,
}

impl Coord {
    fn fold(&self, c: Coord) -> Coord {
        // Folding

        if self.x >= c.x && self.y >= c.y {
            // X Folding
            if c.x > 0 {
                return Coord {
                    x: self.x - 2 * (self.x - c.x),
                    y: self.y,
                };
            }
            // Y Folding
            if c.y > 0 {
                return Coord {
                    x: self.x,
                    y: self.y - 2 * (self.y - c.y),
                };
            }
        }
        *self
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({},{})", self.x, self.y))
    }
}

#[derive(Debug)]
struct Origami {
    dots: HashSet<Coord>,
    folds: VecDeque<Coord>,
}

impl Origami {
    fn step(&mut self) {
        let fold = self.folds.pop_front().unwrap();
        let mut new_dots: HashSet<Coord> = HashSet::new();
        for d in &self.dots {
            if d.x > fold.x || d.y > fold.y {
                new_dots.insert(d.fold(fold));
            } else {
                new_dots.insert(*d);
            }
        }

        self.dots = new_dots;
    }

    fn has_steps(&self) -> bool {
        return self.folds.len() > 0
    }

    fn print(&self) {
        // Get size
        let mut max_x: u32 = 0;
        let mut max_y: u32 = 0;

        for c in &self.dots {
            if c.x > max_x {
                max_x = c.x;
            }

            if c.y > max_y {
                max_y = c.y;
            }
        }

        for y in 0..max_y+1 {
            for x in 0..max_x+1 {
                let c = Coord{x, y };
                if self.dots.contains(&c) {
                    print!("▓")
                } else {
                    print!(" ")
                }
            }
            println!()
        }
    }
}

fn parse_coords(contents: &str) -> Vec<Coord> {
    contents
        .split("\n")
        .map(|l| {
            let s = l.split(",").collect::<Vec<&str>>();
            Coord {
                x: s[0].parse::<u32>().unwrap(),
                y: s[1].parse::<u32>().unwrap(),
            }
        })
        .collect::<Vec<Coord>>()
}

fn get_fold(f: &str) -> Coord {
    let f = f.replace("fold along ", "");

    let fi = f.split("=").collect::<Vec<&str>>();
    match fi[0] {
        "x" => Coord {
            x: fi[1].parse::<u32>().unwrap(),
            y: 0,
        },
        "y" => Coord {
            x: 0,
            y: fi[1].parse::<u32>().unwrap(),
        },
        _ => Coord {
            x: 0,
            y: 0,
        }
    }
}

fn parse_folds(contents: &str) -> VecDeque<Coord> {
    contents.split("\n").map(get_fold).collect()
}

fn parse_puzzle_input(input_file: &str) -> Origami {
    let contents = fs::read_to_string(input_file).unwrap();
    let dots_instructions = contents.split("\n\n").collect::<Vec<&str>>();
    let (dots_s, instructions) = (dots_instructions[0].clone(), dots_instructions[1].clone());
    let dots_v = parse_coords(&dots_s);
    let dots = HashSet::from_iter(dots_v.iter().map(|x| *x));
    let folds = parse_folds(&instructions);

    Origami {
        folds,
        dots,
    }
}

fn part_one(input_file: &str) -> i32 {
    let mut c = parse_puzzle_input(input_file);
    c.step();
    c.dots.len() as i32
}

fn part_two(input_file: &str) -> i32 {
    let mut c = parse_puzzle_input(input_file);
    while c.has_steps() {
        c.step()
    }
    c.print();
    -1
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
