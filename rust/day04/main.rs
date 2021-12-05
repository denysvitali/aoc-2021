use std::{env, fs};
use std::fmt::{Debug, Display, Formatter};

mod test;

#[derive(Debug, Clone)]
struct Board {
    won: bool,
    content: Vec<Vec<(i32, bool)>>,
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Board\n")?;
        write!(f, "{}", self.content
            .iter()
            .map(|r|
                r.iter().map(
                    |x|
                        if !x.1 {
                            format!("{:^3}", x.0)
                        } else {
                            format!("{:^3}", "X")
                        }
                )
                    .collect::<Vec<_>>()
                    .join(" ")
            )
            .collect::<Vec<_>>()
            .join("\n")
        )
    }
}

impl Board {
    fn mark(&mut self, number: i32) {
        for r in &mut self.content {
            for mut c in r {
                if c.0 == number {
                    c.1 = true
                }
            }
        }
    }

    fn won(&mut self) -> bool {
        if self.won {
            return true;
        }
        // Check Rows
        for i in &mut self.content {
            // Count marked in this row
            if i.iter().filter(|x| !(x.1)).count() == 0 {
                self.won = true;
                return true;
            }
        }

        // Check Columns
        for j in 0..self.content[0].len() {
            let mut unmarked = 0;
            for (i, _) in self.content.iter().enumerate() {
                if !self.content[i][j].1 {
                    unmarked += 1;
                }
            }
            if unmarked == 0 {
                self.won = true;
                return true;
            }
        }
        return false;
    }

    fn score(&self) -> i32 {
        self.content
            .iter()
            .map(|x| x.iter()
                .filter(|y| !y.1)
                .map(|y| y.0).sum::<i32>()
            )
            .sum::<i32>()
    }
}

fn parse_line(number_line: &str) -> Vec<(i32, bool)> {
    number_line
        .trim()
        .split(" ")
        .filter(|x| x != &"")
        .map(|x| (x.parse::<i32>().unwrap(), false))
        .collect::<Vec<_>>()
}

fn parse_board(board_lines: &str) -> Board {
    Board {
        content: board_lines
            .split("\n")
            .filter(|x| x != &"")
            .map(parse_line)
            .collect::<Vec<_>>(),
        won: false,
    }
}

fn get_draw_list(content: &str) -> Vec<i32> {
    content
        .split("\n")
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

fn get_boards(content: &str) -> Vec<Board> {
    content.split("\n")
        .skip(2)
        .collect::<Vec<_>>()
        .join("\n")
        .split("\n\n")
        .map(parse_board)
        .collect::<Vec<Board>>()
}

fn part_one(input_file: &str) -> i32 {
    let content = fs::read_to_string(input_file).unwrap();
    let draw_list: Vec<_> = get_draw_list(&content);
    let mut boards = get_boards(&content);

    for d in draw_list {
        for b in &mut boards {
            b.mark(d);
            if b.won() {
                let s = b.score();
                println!("board={}", b);
                println!("d={}, score={}", d, s);
                return d * s;
            }
        }
    }
    return -1;
}

fn part_two(input_file: &str) -> i32 {
    let content = fs::read_to_string(input_file).unwrap();
    let draw_list: Vec<_> = get_draw_list(&content);
    let mut boards = get_boards(&content);

    for d in draw_list {
        // Player playing alone?
        let is_playing_alone = boards.iter().filter(|b|!b.won).count() == 1;

        for b in &mut boards.iter_mut().filter(|b|!b.won) {
            if b.won {
                continue
            }
            b.mark(d);
            if b.won() && is_playing_alone {
                return d * b.score();
            }
        }
    }
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
