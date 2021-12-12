use std::{env, fs};
use std::collections::{HashMap};

mod test;

extern crate petgraph;
use petgraph::visit::EdgeRef;
use petgraph::{Graph, Undirected};
use petgraph::graph::{NodeIndex};

struct CaveSystem {
    start: NodeIndex,
    end: NodeIndex,
    g: Graph<String, u8, Undirected>,
    visit_small: u8,
    paths: Vec<Vec<NodeIndex>>
}

impl CaveSystem {
    fn is_uppercase(&self, n: NodeIndex) -> bool {
        self.g[n] == self.g[n].to_uppercase()   
    }

    fn neighbor(&self, v: NodeIndex) -> Vec<NodeIndex> {
        let mut neighbors = Vec::new();
        for e in self.g.edges(v) {
            if e.target() == v {
                neighbors.push(e.source());
            } else {
                neighbors.push(e.target());
            }
        }
        neighbors
    }

    fn dfs(&mut self, v: NodeIndex, mut visited: Vec<NodeIndex>) {
        if !self.is_uppercase(v) && visited.contains(&v) {
            return
        }
        visited.push(v);
        if self.g[v] == "end" {
            self.paths.push(
                visited
            );
            return
        }

        let neighbors = self.neighbor(v);
        for n in neighbors {
            self.dfs(n, visited.clone());
        }
    }
}

fn parse_lines(contents: &str) -> Vec<(&str, &str)> {
    contents.split("\n")
        .map(|l|{
            let s = l.split("-").collect::<Vec<&str>>();
            (
                *s.get(0).unwrap_or(&"?"),
                *s.get(1).unwrap_or(&"?")
            )
        }
        )
        .collect::<Vec<(&str,&str)>>()
}
 
fn parse_puzzle_input(input_file: &str) -> CaveSystem {
    let contents = fs::read_to_string(input_file).unwrap();
    let entries = parse_lines(&contents).clone();

    let mut cave_system = CaveSystem{
        start: NodeIndex::new(0),
        end: NodeIndex::new(0),
        g: Graph::new_undirected(),
        visit_small: 1,
        paths: vec![],
    };

    let mut node_map : HashMap<String, NodeIndex> = HashMap::new();

    for e in &entries {
        let g = &mut cave_system.g;
        let start = match node_map.get(&e.0.to_string()) {
            Some(n) => *n,
            None => {
                let n = g.add_node(e.0.to_string());
                node_map.insert(e.0.to_string(), n);
                match e.0 {
                    "start" => {cave_system.start = n;}
                    "end" => {cave_system.end = n;}
                    _ => {}
                };
                n
            }
        };

        let dest = match node_map.get(e.1) {
            Some(n) => *n,
            None => {
                let n = g.add_node(e.1.to_string());
                node_map.insert(e.1.to_string(), n);
                match e.1 {
                    "start" => {cave_system.start = n;}
                    "end" => {cave_system.end = n;}
                    _ => {}
                };
                n
            }
        };

        // Create edge
        g.add_edge(start, dest, 1);
    }
    cave_system
}

/*
    dfs_paths returns a set of paths starting at "start"
    and ending at "end", where the following rules apply:

    - uppercase nodes can be visited multiple times
    - lowercase nodes can be visited only once
*/
fn dfs_paths(c: &mut CaveSystem) -> Vec<Vec<String>> {
    c.dfs(c.start, vec![]);

    c.paths
        .iter()
        .map(|x|
            x.iter().map(|e| c.g[*e].clone())
            .collect::<Vec<String>>()
        )
        .collect::<Vec<Vec<String>>>()
}

fn part_one(input_file: &str) -> i32 {
    let mut c = parse_puzzle_input(input_file);
    let paths = dfs_paths(&mut c);
    paths.len() as i32
}

fn part_two(input_file: &str) -> i32 {
    let mut c = parse_puzzle_input(input_file);
    c.visit_small = 2;
    let paths = dfs_paths(&mut c);
    paths.len() as i32
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
