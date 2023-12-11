use crate::utils::*;

pub struct Solution {
    histories: Vec<Vec<i32>>,
}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self { histories: vec![] };
        sol.process_input("day09/input.txt");
        sol
    }
}

impl Solve for Solution {
    fn process_input(&mut self, path: &str) {
        for line in read_file(path).lines() {
            let history = line.split(' ').map(|n| n.parse::<i32>().unwrap()).collect();
            self.histories.push(history);
        }
    }

    fn part1(&mut self) {}

    fn part2(&mut self) {}
}

fn diffs(v: Vec<i32>) -> Vec<i32> {
    let mut diff = vec![];
    for i in 1..v.len() {
        diff.push(v[i] - v[i - 1]);
    }

    return diff;
}
