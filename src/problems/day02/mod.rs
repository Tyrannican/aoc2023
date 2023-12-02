use crate::utils::*;

pub struct Solution {}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self {};
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
            for subgame in game_outcomes.split("; ") {
                for selection in subgame.split(", ") {
                    println!("Selection: {selection}");
                }
            }
        }
    }

    fn part1(&mut self) {}

    fn part2(&mut self) {}
}
