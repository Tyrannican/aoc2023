use crate::utils::*;

// Add your own custom fields for the solution here
pub struct Solution {
    codes: Vec<String>,
}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self { codes: vec![] };

        sol.process_input("day01/input.txt");
        sol
    }
}

impl Solve for Solution {
    // Perform any manipulations to the input here
    fn process_input(&mut self, path: &str) {
        let raw = read_file(path);
        self.codes = raw
            .split('\n')
            .filter(|l| !l.is_empty())
            .map(|l| l.to_string())
            .collect::<Vec<String>>();
    }

    fn part1(&mut self) {
        let total: i32 = self
            .codes
            .iter()
            .map(|code| {
                let numbers = code
                    .chars()
                    .into_iter()
                    .filter(|c| c.is_numeric())
                    .collect::<Vec<char>>();

                format!("{}{}", numbers[0], numbers[numbers.len() - 1])
                    .parse::<i32>()
                    .unwrap()
            })
            .sum();

        println!("Day 01 / Part 1: {total}");
    }

    fn part2(&mut self) {}
}
