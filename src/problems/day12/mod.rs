use crate::utils::*;

#[derive(Debug)]
pub struct Record {
    config: Vec<usize>,
    springs: String,
}

pub struct Solution {
    records: Vec<Record>,
}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self { records: vec![] };
        sol.process_input("day12/input.txt");
        sol
    }
}

impl Solve for Solution {
    fn process_input(&mut self, path: &str) {
        for line in read_file(path).lines() {
            let (springs, config) = line.split_once(' ').unwrap();

            let config = config
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            self.records.push(Record {
                springs: springs.to_string(),
                config,
            });
        }
    }

    fn part1(&mut self) {}

    fn part2(&mut self) {}
}
