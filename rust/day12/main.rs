use std::collections::{HashMap};
use std::{env, fs};

mod test;

extern crate petgraph;
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use petgraph::{Graph, Undirected};

struct CaveSystem {
    start: NodeIndex,
    end: NodeIndex,
    g: Graph<String, u8, Undirected>,
    visit_small: u8,
    paths: Vec<Vec<NodeIndex>>,
}

impl CaveSystem {
    fn is_uppercase(&self, n: NodeIndex) -> bool {
        self.g[n].get(0..1).unwrap() == self.g[n].get(0..1).unwrap().to_uppercase()
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

    // Not proud of this part
    fn dfs_simple(&mut self, v: NodeIndex, mut visited: Vec<NodeIndex>){
        if !self.is_uppercase(v) && visited.contains(&v) {
            return
        }
        visited.push(v);
        if v == self.end {
            self.paths.push(
                visited
            );
            return
        }

        let neighbors = self.neighbor(v);
        for n in neighbors {
            self.dfs_simple(n, visited.clone());
        }
    }

    fn dfs(
        &mut self,
        v: NodeIndex,
        mut visited: HashMap<NodeIndex, u8>,
        mut steps: Vec<NodeIndex>,
        mut sc_twice: bool,
    ) {
        if v == self.end {
            steps.push(v);
            self.paths.push(steps.to_vec());
            return;
        }

        if !self.is_uppercase(v) {
            // Already visited a small cave ?
            let already_visited_small_cave_twice = sc_twice || match visited
                .iter()
                .find(|(_, v)| **v >= self.visit_small) {
                    Some(_) => true,
                    None => false
                };

            let visit_times = *visited.get(&v).unwrap_or(&0);
            if already_visited_small_cave_twice && visit_times >= 1 {
                // Can't visit a small cave again if we
                // already visited a small cave twice
                return;
            }

            if v == self.start && visit_times == 1 {
                // Can't visit start twice
                return;
            }
            sc_twice = already_visited_small_cave_twice;
            visited.insert(v, visit_times + 1);
        }
        steps.push(v);

        let neighbors = self.neighbor(v);
        for n in neighbors {
            self.dfs(n, visited.clone(), steps.clone(), sc_twice);
        }
    }
}

fn parse_lines(contents: &str) -> Vec<(&str, &str)> {
    contents
        .split("\n")
        .map(|l| {
            let s = l.split("-").collect::<Vec<&str>>();
            (*s.get(0).unwrap_or(&"?"), *s.get(1).unwrap_or(&"?"))
        })
        .collect::<Vec<(&str, &str)>>()
}
fn parse_puzzle_input(input_file: &str) -> CaveSystem {
    let contents = fs::read_to_string(input_file).unwrap();
    let entries = parse_lines(&contents).clone();

    let mut cave_system = CaveSystem {
        start: NodeIndex::new(0),
        end: NodeIndex::new(0),
        g: Graph::new_undirected(),
        visit_small: 1,
        paths: Vec::new(),
    };

    let mut node_map: HashMap<String, NodeIndex> = HashMap::new();

    for e in &entries {
        let g = &mut cave_system.g;
        let start = match node_map.get(&e.0.to_string()) {
            Some(n) => *n,
            None => {
                let n = g.add_node(e.0.to_string());
                node_map.insert(e.0.to_string(), n);
                match e.0 {
                    "start" => {
                        cave_system.start = n;
                    }
                    "end" => {
                        cave_system.end = n;
                    }
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
                    "start" => {
                        cave_system.start = n;
                    }
                    "end" => {
                        cave_system.end = n;
                    }
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
    if c.visit_small == 1 {
        // Part 1
        c.dfs_simple(c.start, vec![]);
    } else {
        c.dfs(c.start, HashMap::new(), vec![], false);
    }

    c.paths
        .iter()
        .map(|x| x.iter().map(|e| c.g[*e].clone()).collect::<Vec<String>>())
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
    })
    .unwrap();

    println!("Part 1: {}", part_one(path));
    println!("Part 2: {}", part_two(path));
}
