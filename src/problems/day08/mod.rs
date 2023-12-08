use crate::utils::*;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
}

impl Direction {
    pub fn from_char(ch: char) -> Self {
        match ch {
            'R' => Self::Right,
            'L' => Self::Left,
            _ => Self::Left,
        }
    }
}

pub struct Solution {
    pub directions: Vec<Direction>,
    pub map: HashMap<String, Vec<String>>,
}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self {
            directions: vec![],
            map: HashMap::new(),
        };
        sol.process_input("day08/input.txt");
        sol
    }
}

impl Solve for Solution {
    fn process_input(&mut self, path: &str) {
        let raw = read_file(path);
        let (directions, mapping) = raw.split_once("\n\n").unwrap();
        self.directions = directions
            .chars()
            .map(|c| Direction::from_char(c))
            .collect();

        for line in mapping.lines() {
            let (src, targets) = line.split_once(" = ").unwrap();
            let dsts = targets
                .chars()
                .filter_map(|c| match c {
                    '(' | ')' | ',' => None,
                    _ => Some(c),
                })
                .collect::<String>();

            let dsts = dsts
                .split(' ')
                .map(|t| t.to_string())
                .collect::<Vec<String>>();

            self.map.insert(src.to_string(), dsts);
        }
    }

    fn part1(&mut self) {
        let mut steps = 0;
        let mut current = String::from("AAA");
        let mut found = false;

        loop {
            for direction in self.directions.iter() {
                let current_map = self.map.get(&current).unwrap();
                current = match direction {
                    Direction::Left => current_map[0].clone(),
                    Direction::Right => current_map[1].clone(),
                };
                steps += 1;

                if &current == "ZZZ" {
                    found = true;
                    break;
                }
            }

            if found {
                break;
            }
        }

        println!("Steps: {steps}");
    }

    fn part2(&mut self) {}
}
