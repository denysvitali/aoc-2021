extern crate priority_queue;
extern crate termion;

use std::{env, fs};
use std::borrow::BorrowMut;
use std::cmp::{min, Ordering};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::collections::hash_map::Entry;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::hash::Hash;

use priority_queue::PriorityQueue;
use termion::{color, style};

mod test;

fn parse_input(input_file: &str) -> Matrix {
    let input = fs::read_to_string(input_file).unwrap();
    let matrix: Vec<Vec<i32>> = input
        .split("\n")
        .map(|r| {
            r.chars()
                .map(|c| c.to_string().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    let max_y = matrix.len();
    let max_x = matrix[0].len();

    Matrix {
        m: matrix,
        size: (max_x - 1, max_y - 1),
    }
}

type Coord = (usize, usize);

#[derive(Debug, Clone, Copy)]
struct Node {
    g: i32,
    h: i32,
    coord: Coord,
}

impl Node {
    fn new(coord: Coord, g: i32, h: i32) -> Node {
        return Node {
            coord,
            g,
            h
        };
    }

    fn f(&self) -> i32 {
        return self.g + self.h;
    }
}

impl Eq for Node {}

impl PartialEq<Self> for Node {
    fn eq(&self, other: &Self) -> bool {
        self.coord == other.coord && self.g == other.g && self.h == other.h
    }
}

impl PartialOrd<Self> for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Option::from(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.f() < other.f() {
            return Ordering::Greater;
        }

        if self.f() < other.f() {
            return Ordering::Less;
        }

        Ordering::Equal
    }
}

struct Matrix {
    m: Vec<Vec<i32>>,
    size: (usize, usize),
}

impl Matrix {
    fn neighbors(&self, at: Coord, generate: bool) -> Vec<(Coord, i32)> {
        let mut neighbors: Vec<(Coord, i32)> = vec![];

        if at.1 < self.size.1 {
            // Bottom neighbor
            let c = (at.0, at.1 + 1);
            neighbors.push((c, self.get_value(c)));
        }

        if at.0 < self.size.0 {
            // Right Neighbor
            let c = (at.0 + 1, at.1);
            neighbors.push((c, self.get_value(c)));
        }

        if at.0 != 0 {
            // Left neighbor
            let c = (at.0 - 1, at.1);
            neighbors.push((c, self.get_value(c)));
        }

        if at.1 != 0 {
            // Top neighbor
            let c = (at.0, at.1 - 1);
            neighbors.push((c, self.get_value(c)));
        }

        neighbors
    }

    fn get_value(&self, c: Coord) -> i32 {
        self.m[c.1][c.0]
    }

    fn route(&self, start: Coord, end: Coord) -> (u32, Vec<Coord>) {
        let mut open: BinaryHeap<Node> = BinaryHeap::new();
        let mut closed: HashSet<Coord> = HashSet::new();
        let mut path_to: HashMap<Coord, (Coord, i32)> = HashMap::new();

        let should_generate = false;

        open.push(Node::new(start, 0, 0));

        loop {
            let current_node_wrapped = open.pop();
            if current_node_wrapped.is_none() {
                // No more node to visit
                panic!("Unexpected behaviour")
            }

            let current_node = current_node_wrapped.unwrap();
            closed.insert(current_node.coord);

            if current_node.coord == end {
                // Done
                break
            }

            let neighbours = self.neighbors(current_node.coord, should_generate);
            for neighbor in neighbours {
                if closed.contains(&neighbor.0){
                    // Already visited
                    continue
                }

                let g = current_node.f() + self.get_value(neighbor.0);
                let h = 0; //(euclidean_distance(neighbor.0, end)) as i32;
                let n = Node::new(neighbor.0, g, h);

                match path_to.entry(neighbor.0) {
                    Occupied(mut e) => {
                        if n.f() < e.get().1 {
                            e.insert((current_node.coord, n.f()));
                        }
                    }
                    Vacant(e) => {
                        e.insert((current_node.coord, n.f()));
                    }
                }

                if open.iter().find(|e|*e==&n).is_none() {
                    open.push(n);
                }
            }
        }

        let mut current = end;
        let mut score = 0;
        let mut path : Vec<Coord> = Vec::new();

        path.push(end);

        loop {
            if current == start {
                path.reverse();
                return (score, path);
            }

            // Get From
            let (from, _) = path_to.get(&current).unwrap();
            score += self.get_value(current) as u32;
            current = *from;
            path.push(current);
        }

        (0, Vec::new())

    }

    fn draw_path(&self, path: &Vec<Coord>) {
        let path_set: HashSet<Coord> = HashSet::from_iter(path.iter().map(|x| *x));

        for (x, row) in self.m.iter().enumerate() {
            for (y, v) in row.iter().enumerate() {
                if path_set.contains(&(x, y)) {
                    print!("{}{}{}", color::Bg(color::Red), v, style::Reset);
                } else {
                    print!("{}", v);
                }
            }
            println!()
        }
    }
}

fn euclidean_distance(p: Coord, q: Coord) -> f32 {
    ((
        (p.0 as i32 - q.0 as i32).pow(2) +
        (p.1 as i32 - q.1 as i32).pow(2)
    ) as f32).sqrt()
}

fn part_one(input_file: &str) -> u32 {
    let matrix = parse_input(input_file);
    let (cost, path) = matrix.route((0, 0), matrix.size);
    println!("path={:?}", &path);
    cost
}

fn part_two(input_file: &str) -> u32 {
    let matrix = parse_input(input_file);
    let target = (matrix.size.0 * 5, matrix.size.1 * 5);
    let (cost, path) = matrix.route((0, 0), target);
    println!("path={:?}", path);
    cost
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
