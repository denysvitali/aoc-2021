use std::{env, fs};
use petgraph::{Directed, Graph};
use petgraph::algo::astar;
use petgraph::graph::NodeIndex;

mod test;
extern crate petgraph;

fn parse_input(input_file: &str) -> (Graph<i32, i32, Directed>, usize) {
    let input = fs::read_to_string(input_file).unwrap();
    let matrix: Vec<Vec<i32>> = input
        .split("\n")
        .map(|r|
            r.chars()
            .map(|c|c.to_string().parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
        )
        .collect();

    let mut g: Graph<i32, i32, Directed> = Graph::new();
    let max_x = matrix[0].len();
    let max_y = matrix.len();

    let target = max_y * max_x;

    for (y, r) in matrix.iter().enumerate() {
        for (x, v) in r.iter().enumerate() {
            let n = g.add_node(*v);

            // Get neighbors
            if y != 0 {
                // Has seen the first line, add top neighbor
                let tn = NodeIndex::new((y - 1) * max_x + x);
                g.add_edge(n, tn, g[tn]);
                g.add_edge(tn, n, *v);
            }

            // Add left neighbor
            if x != 0 {
                let ln = NodeIndex::new(y * max_x + x - 1);
                g.add_edge(n, ln, g[ln]);
                g.add_edge(ln, n, *v);
            }
        }
    }

    (g, target)
}


fn part_one(input_file: &str) -> i32 {
    let (g, target) = parse_input(input_file);
    let ni = NodeIndex::new(target -1);

    let (cost, path) = astar(&g,
                     NodeIndex::new(0),
                     |finish| finish == ni,
                     |e| *e.weight(),
                     |_| 0).unwrap_or((0, Vec::new()));

    println!("path={:?}", path);
    cost
}

fn part_two(input_file: &str) -> i64 {
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
