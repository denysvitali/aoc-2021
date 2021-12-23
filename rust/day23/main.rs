extern crate termion;

use std::{env, fs, usize};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::{Display, Formatter, Write};

use termion::{color, style};

use MapElement::*;

use crate::Amphipod::{Amber, Bronze, Copper, Desert};
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::ptr::hash;

mod test;

type Coord = (usize, usize);

#[derive(Clone, Debug)]
enum MapElement {
    Free,
    Wall,
    NotVisitable,
    AmphipodW(Amphipod),
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Amphipod {
    fn energy(self) -> u32 {
        match self {
            Amber => 1,
            Bronze => 10,
            Copper => 100,
            Desert => 1000,
        }
    }
}

impl ToString for Amphipod {
    fn to_string(&self) -> String {
        match self {
            Amber => "A",
            Bronze => "B",
            Copper => "C",
            Desert => "D",
        }.to_string()
    }
}

struct Map {
    content: Vec<Vec<MapElement>>,
}

impl Map {
    // Returns the position of the Amphipods in the map
    fn amphipod_positions(&self) -> HashMap<Coord, Amphipod> {
        let mut hm: HashMap<Coord, Amphipod> = HashMap::new();
        for (y, row) in self.content.iter().enumerate() {
            for (x, v) in row.iter().enumerate() {
                match v {
                    AmphipodW(a) => {
                        hm.insert((x, y), a.clone());
                    }
                    _ => {}
                };
            }
        }
        hm
    }

    fn possible_positions(&self) -> HashSet<Coord> {
        let mut positions: HashSet<Coord> = HashSet::new();
        for (y, row) in self.content.iter().enumerate() {
            for (x, v) in row.iter().enumerate() {
                match v {
                    Free => positions.insert((x, y)),
                    _ => true
                };
            }
        }
        positions
    }

    fn entrances(&self) -> Vec<Coord> {
        let mut positions: Vec<Coord> = vec![];
        for (y, row) in self.content.iter().enumerate() {
            for (x, v) in row.iter().enumerate() {
                match v {
                    Free => {
                        match self.content[y + 1][x] {
                            AmphipodW(_) => positions.push((x, y)),
                            _ => {}
                        };
                    }
                    _ => {}
                };
            }
        }
        positions
    }
}

#[derive(Debug)]
struct Move {
    from: Coord,
    to: Coord,
    whom: Amphipod,
    cost: u32,
}

#[derive(Clone)]
struct GameState {
    amphipods: HashMap<Coord, Amphipod>,
    possible_positions: HashSet<Coord>,
    entrances: Vec<Coord>,
    energy: u32,
    moves: u32,
}

impl Display for GameState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Energy: {}\n", self.energy));
        for y in 0..5 {
            for x in 0..15 {
                let coord: Coord = (x, y);
                match self.amphipods.get(&coord) {
                    None => {}
                    Some(x) => {f.write_str(&x.to_string()); continue}
                };

                if self.entrances.contains(&coord) || self.possible_positions.contains(&coord) {
                    f.write_str(".");
                    continue
                }

                f.write_str("#");
            }
            f.write_str("\n");
        }
        Ok(())
    }
}

impl <'a> PartialEq<Self> for GameState {
    fn eq(&self, other: &Self) -> bool {
        self.entrances.eq(&other.entrances) &&
            self.energy.eq(&other.energy) &&
            self.possible_positions.eq(&other.possible_positions) &&
            self.amphipods.eq(&other.amphipods)
    }
}

impl <'a> PartialOrd<Self> for GameState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        todo!()
    }
}

impl <'a> Eq for GameState{}

impl <'a> Ord for GameState {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.energy < other.energy {
            return Ordering::Greater
        }
        else if self.energy > other.energy {
            return Ordering::Less
        }
        return Ordering::Equal
    }
}

impl<'a,'b> GameState {
    fn new(amphipod_positions: HashMap<Coord, Amphipod>,
           possible_positions: HashSet<Coord>,
           entrances: Vec<Coord>,
    ) -> Self {
        GameState {
            amphipods: amphipod_positions,
            energy: 0,
            moves: 0,
            possible_positions,
            entrances,
        }
    }

    fn in_right_position(&self, c: &Coord, a: &Amphipod) -> bool {
        let pos = vec![Amber, Bronze, Copper, Desert];
        for (o, e) in self.entrances.iter().enumerate() {
            if e.0 == c.0 && e.1 < c.1 {
                return a == &pos[o]
            }
        }
        false
    }

    fn next_moves(&self, last_move: Option<&Move>) -> Vec<Move> {
        let mut output_moves: Vec<Move> = Vec::new();
        for (c, a) in &self.amphipods {
            // Check if this Amphipod can move
            if last_move.is_some() && last_move.unwrap().to == *c {
                // Ignore, otherwise we keep on moving the same piece
                continue
            }

            if self.in_right_position(c, a) {
                // Ignore
                continue
            }

            let mut moves: HashSet<(Coord, u32)> = HashSet::new();
            let mut visited: HashSet<Coord> = HashSet::new();
            self.evaluate_moves(*c, a, &mut moves, &mut visited, 0);
            let mut moves_arr: Vec<Move> = moves.iter().map(|m| Move {
                from: *c,
                to: m.0,
                whom: a.clone(),
                cost: m.1,
            }).collect::<Vec<Move>>();
            output_moves.append(&mut moves_arr);
        }
        output_moves
    }

    fn evaluate_moves(&self, c: Coord,
                      a: &Amphipod,
                      moves: &mut HashSet<(Coord, u32)>,
                      visited: &mut HashSet<Coord>,
                      energy: u32
    ) {
        for (x, y) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_x = (c.0 as i32 + x) as i32;
            let new_y = (c.1 as i32 + y) as i32;
            if new_x < 0 || new_y < 0 {
                continue
            }
            let coord: Coord = (new_x as usize, new_y as usize);

            if visited.contains(&coord) {
                // Won't visit again, it's useless
                continue
            }

            if self.entrances.contains(&coord){
                // Entrances cannot be final positions
                visited.insert(coord);
                self.evaluate_moves(coord, a, moves, visited, energy + a.energy());
                continue
            }

            if self.possible_positions.contains(&coord) {
                // Valid position
                visited.insert(coord);
                moves.insert((coord, energy + a.energy()));

                // Validate moves from this next position
                self.evaluate_moves(coord, a, moves, visited,energy + a.energy());
            }
        }
    }

    fn play_move(&self, m: &Move) -> Option<GameState> {
        if self.moves > 10 {
            return None
        }
        let mut new_game_state = self.clone();
        new_game_state.amphipods.remove(&m.from);
        new_game_state.amphipods.insert(m.to, m.whom);
        new_game_state.possible_positions.remove(&m.to);
        new_game_state.possible_positions.insert(m.from);
        new_game_state.consume(m.cost);
        new_game_state.moves += 1;

        let mut game_states: BinaryHeap<GameState> = BinaryHeap::new();
        let next_moves = new_game_state.next_moves(Some(&m));
        for next_move in &next_moves{
            let cgs = new_game_state.play_move(&next_move.clone());
            if cgs.as_ref().is_none() {
                continue
            }
            if cgs.as_ref().unwrap().win(){
                game_states.push(cgs.unwrap().clone());
            }
        }
        if game_states.len() == 0 {
            return None
        }
        Some(game_states.peek().unwrap().clone())
    }

    fn win(&self) -> bool {
        let mut last_x: Option<usize> = Option::None;
        let mut offset = 0;
        let amphipods_list: Vec<Amphipod> = vec![Amber, Bronze, Copper, Desert];

        for r in &self.entrances {
            let x = r.0;
            if last_x.is_none() {
                last_x = Some(x);
            }
            if x > last_x.unwrap() {
                // Next
                last_x = Some(x);
                offset += 1;
            }

            // Find entries for this X
            for y_offset in 1..10 {
                let a_c = (x, r.1 + y_offset);
                match self.amphipods.get(&a_c) {
                    Some(v) => {
                        if *v != amphipods_list[offset] {
                            return false
                        }
                    },
                    None => break
                }
            }
        }
        return true
    }

    fn consume(&mut self, amount: u32){
        self.energy += amount;
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for r in &self.content {
            for v in r {
                f.write_fmt(format_args!("{}", v));
            }
            f.write_str("\n");
        }
        Ok(())
    }
}

impl Display for MapElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let repr = match self {
            AmphipodW(a) => a.to_string(),
            Wall => "#".to_string(),
            Free => ".".to_string(),
            NotVisitable => " ".to_string(),
            Unknown => "?".to_string(),
        };
        f.write_str(&repr)
    }
}

fn parse_input<'a>(input_file: &str) -> GameState {
    let input = fs::read_to_string(input_file).unwrap();
    let content = input
        .split("\n")
        .map(|r| {
            r.chars().map(|x| match x {
                '#' => Wall,
                'A' => AmphipodW(Amber),
                'B' => AmphipodW(Bronze),
                'C' => AmphipodW(Copper),
                'D' => AmphipodW(Desert),
                ' ' => NotVisitable,
                '.' => Free,
                _ => Unknown
            }).collect()
        })
        .collect();
    let map = Map {
        content,
    };
    let mut entrances = map.entrances();
    // sort entrances by X
    entrances.sort_by(|(x1,y1), (x2, y2)| x1.cmp(x2));

    let amphipod_positions = map.amphipod_positions();
    let possible_positions = map.possible_positions();

    GameState::new(amphipod_positions, possible_positions, entrances)
}

fn part_one(input_file: &str) -> u32 {
    let game_state = parse_input(input_file);
    let next_moves = game_state.next_moves(None);

    for m in &next_moves {
        println!("Playing move {:?}", m);
        game_state.play_move(m);
    }

    println!("next_moves={:?}", next_moves);
    0
}

fn part_two(input_file: &str) -> u32 {
    0
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
