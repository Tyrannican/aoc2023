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

        println!("Day 08 / Part 1: {steps}");
    }

    fn part2(&mut self) {
        let nodes = self
            .map
            .keys()
            .filter_map(|k| {
                if k.ends_with("A") {
                    return Some(k.clone());
                }

                None
            })
            .collect::<Vec<String>>();

        let mut total_steps = vec![];
        for node in nodes.iter() {
            let mut steps = 0;
            let mut src = node;
            let mut found = false;
            loop {
                for direction in self.directions.iter() {
                    let dsts = self.map.get(src).unwrap();
                    src = match direction {
                        Direction::Left => &dsts[0],
                        Direction::Right => &dsts[1],
                    };
                    steps += 1;

                    if src.ends_with("Z") {
                        total_steps.push((node.clone(), steps as usize));
                        found = true;
                        break;
                    }
                }

                if found {
                    break;
                }
            }
        }

        let common_point = find_convergence_point(&total_steps);
        println!("Day 08 / Part 2: {common_point}");
    }
}

// To be fair, I recognised _what_ the solution was, I just didnt know how to implement...
// Wee bit of help getting the below solution.
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn find_convergence_point(data: &Vec<(String, usize)>) -> usize {
    let mut result = data[0].1;
    for i in 1..data.len() {
        result = lcm(result, data[i].1);
    }
    result
}
