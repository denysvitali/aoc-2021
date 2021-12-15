extern crate termion;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::{env, fs, usize};
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
        size: (max_x, max_y),
        real_size: (max_x, max_y),
    }
}

type Coord = (usize, usize);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Node {
    g: i32,
    h: i32,
    coord: Coord,
}

impl Node {
    fn new(coord: Coord, g: i32, h: i32) -> Node {
        return Node { coord, g, h };
    }

    fn f(&self) -> i32 {
        return self.g + self.h;
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({},{}): g={}, h={}",
            self.coord.0, self.coord.1, self.g, self.h,
        )
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
    size: Coord,
    real_size: Coord,
}

impl Matrix {
    fn neighbors(&self, at: Coord) -> Vec<(Coord, i32)> {
        let mut neighbors: Vec<(Coord, i32)> = vec![];

        if at.1 < self.size.1 - 1 {
            // Bottom neighbor
            let c = (at.0, at.1 + 1);
            neighbors.push((c, self.get_value(c)));
        }

        if at.0 < self.size.0 - 1 {
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
        if c.0 >= self.real_size.0 || c.1 >= self.real_size.1 {
            let mult_x = c.0 / self.real_size.0;
            let mult_y = c.1 / self.real_size.1;
            let mult_max = mult_x + mult_y;
            let risk_level =
                self.m[c.1 % (self.real_size.1)][c.0 % (self.real_size.0)] + (mult_max as i32);

            return if risk_level >= 10 {
                risk_level % 10 + 1
            } else {
                risk_level
            };
        }
        self.m[c.1][c.0]
    }

    /*
       Based on the A* algorithm pseudocode:
       https://en.wikipedia.org/wiki/A*_search_algorithm
    */

    fn route(&self, start: Coord, end: Coord) -> (u32, Vec<Coord>) {
        let mut open: BinaryHeap<Node> = BinaryHeap::new();
        let mut closed: HashSet<Coord> = HashSet::new();
        let mut came_from: HashMap<Coord, Node> = HashMap::new();
        let mut g_score: HashMap<Coord, i32> = HashMap::new();

        open.push(Node::new(start, self.get_value(start), 0));
        g_score.insert(start, 0);

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
                break;
            }

            let neighbours = self.neighbors(current_node.coord);
            for neighbor in neighbours {
                if closed.contains(&neighbor.0) {
                    // Already visited
                    continue;
                }

                let tentative_gscore = g_score.get(&current_node.coord).unwrap() + neighbor.1;

                if tentative_gscore < *g_score.get(&neighbor.0).unwrap_or(&i32::MAX) {
                    came_from.insert(neighbor.0, current_node);
                    g_score.insert(neighbor.0, tentative_gscore);

                    if open.iter().find(|e| e.coord == neighbor.0).is_none() {
                        open.push(Node::new(neighbor.0, tentative_gscore, 0));
                    }
                }
            }
        }

        let mut current = end;
        let mut score: u32 = 0;
        let mut path: Vec<Coord> = Vec::new();

        path.push(end);

        loop {
            if current == start {
                path.reverse();
                return (score, path);
            }

            // Get From
            let from = came_from.get(&current).unwrap();
            score += self.get_value(current) as u32;
            current = from.coord;
            path.push(current);
        }
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

fn part_one(input_file: &str) -> u32 {
    let matrix = parse_input(input_file);
    let target = (matrix.real_size.0 - 1, matrix.real_size.1 - 1);
    let (cost, _) = matrix.route((0, 0), target);
    cost
}

fn part_two(input_file: &str) -> u32 {
    let mut matrix = parse_input(input_file);
    matrix.size = (matrix.real_size.0 * 5, matrix.real_size.1 * 5);

    let target = (matrix.size.0 - 1, matrix.size.1 - 1);
    let (cost, _) = matrix.route((0, 0), target);
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
