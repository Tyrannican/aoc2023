use crate::utils::*;

// Add your own custom fields for the solution here
pub struct Solution {}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self {};
        sol.process_input("day17/input.txt");
        sol
    }
}

impl Solve for Solution {
    // Perform any manipulations to the input here
    fn process_input(&mut self, path: &str) {
        let _raw = read_file(path);
    }

    fn part1(&mut self) {
        println!("Day17 - Part 1: Edit me to start!");
    }

    fn part2(&mut self) {
        println!("Day17 - Part 2: Edit me to start!");
    }
}
