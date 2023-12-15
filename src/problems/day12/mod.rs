use crate::utils::*;

#[derive(Debug, PartialEq)]
pub enum Status {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug)]
pub struct Record {
    config: Vec<usize>,
    springs: Vec<Status>,
}

impl Record {
    // Taken from https://github.com/andypymont/advent2023-rust/blob/main/src/bin/12.rs
    pub fn arrangements(&self, spring_idx: usize, cfg_idx: usize) -> u64 {
        let consume_group = self.config.get(cfg_idx).map_or(0, |c_len| {
            if (spring_idx + c_len) > self.springs.len() {
                return 0;
            }

            if (0..*c_len)
                .any(|idx| self.springs.get(spring_idx + idx) == Some(&Status::Operational))
            {
                return 0;
            }

            if self.springs.get(spring_idx + c_len) == Some(&Status::Damaged) {
                return 0;
            }

            self.arrangements(spring_idx + c_len + 1, cfg_idx + 1)
        });

        let skip = match self.springs.get(spring_idx) {
            None => u64::from(cfg_idx >= self.config.len()),
            Some(Status::Damaged) => 0,
            Some(_) => self.arrangements(spring_idx + 1, cfg_idx),
        };

        return consume_group + skip;
    }
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

            let springs = springs
                .chars()
                .filter_map(|c| match c {
                    '#' => Some(Status::Damaged),
                    '.' => Some(Status::Operational),
                    '?' => Some(Status::Unknown),
                    _ => None,
                })
                .collect();

            let config = config
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            self.records.push(Record { springs, config });
        }
    }

    fn part1(&mut self) {
        let total = self
            .records
            .iter()
            .map(|r| r.arrangements(0, 0))
            .sum::<u64>();

        println!("Day 12 / Part 1: {total}");
    }

    fn part2(&mut self) {}
}
