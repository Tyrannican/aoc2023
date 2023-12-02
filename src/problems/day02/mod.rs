use crate::utils::*;

#[derive(Debug)]
enum Cube {
    Red(i32),
    Blue(i32),
    Green(i32),
    Invalid,
}

impl Cube {
    fn from_str(s: &str) -> Self {
        let (color, value) = s.split_once(' ').unwrap();
        let count = value.parse::<i32>().unwrap();
        match color {
            "red" => Self::Red(count),
            "blue" => Self::Blue(count),
            "green" => Self::Green(count),
            _ => Self::Invalid,
        }
    }
}

struct Game {
    id: i32,
    subgames: Vec<String>,
}

impl Game {
    pub fn is_possible_game(&self, red: i32, green: i32, blue: i32) -> Option<i32> {
        let mut rc = 0;
        let mut gc = 0;
        let mut bc = 0;

        for subgame in self.subgames.iter() {
            for selection in subgame.split(", ") {
                let (num, color) = selection.split_once(' ').unwrap();
                let num = num.parse::<i32>().unwrap();
                match color {
                    "red" => {
                        if num > rc {
                            rc = num;
                        }
                    }
                    "green" => {
                        if num > gc {
                            gc = num;
                        }
                    }
                    "blue" => {
                        if num > bc {
                            bc = num;
                        }
                    }
                    _ => {}
                }
            }
        }

        if rc <= red && gc <= green && bc <= blue {
            return Some(self.id);
        }
        return None;
    }

    pub fn minimum_number_of_cubes(&self) -> (i32, i32, i32) {
        let mut rc = 0;
        let mut gc = 0;
        let mut bc = 0;

        for subgame in self.subgames.iter() {
            for selection in subgame.split(", ") {
                let (num, color) = selection.split_once(' ').unwrap();
                let num = num.parse::<i32>().unwrap();
                match color {
                    "red" => {
                        if num > rc {
                            rc = num;
                        }
                    }
                    "green" => {
                        if num > gc {
                            gc = num;
                        }
                    }
                    "blue" => {
                        if num > bc {
                            bc = num;
                        }
                    }
                    _ => {}
                }
            }
        }

        return (rc, gc, bc);
    }
}

pub struct Solution {
    games: Vec<Game>,
}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self { games: vec![] };
        sol.process_input("day02/input.txt");
        sol
    }
}

impl Solve for Solution {
    fn process_input(&mut self, path: &str) {
        let games = read_file(path)
            .split('\n')
            .filter(|g| !g.is_empty())
            .map(|g| g.to_string())
            .collect::<Vec<String>>();

        for game in games.iter() {
            let (game_id, game_outcomes) = game.split_once(": ").unwrap();
            let (_, id_str) = game_id.split_once(' ').unwrap();
            let id = id_str.parse::<i32>().unwrap();
            self.games.push(Game {
                id,
                subgames: game_outcomes
                    .split("; ")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
            });
        }
    }

    fn part1(&mut self) {
        let total: i32 = self
            .games
            .iter()
            .filter_map(|g| g.is_possible_game(12, 13, 14))
            .sum();

        println!("Day 02 / Part 1: {total}");
    }

    fn part2(&mut self) {
        let total: i32 = self
            .games
            .iter()
            .map(|g| {
                let (r, g, b) = g.minimum_number_of_cubes();
                r * g * b
            })
            .sum();

        println!("Day 02 / Part 2: {total}");
    }
}
