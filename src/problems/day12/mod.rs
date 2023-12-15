use crate::utils::*;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Status {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Clone, Debug)]
pub struct Record {
    config: Vec<usize>,
    springs: Vec<Status>,
}

impl Record {
    // Taken from https://github.com/andypymont/advent2023-rust/blob/main/src/bin/12.rs
    pub fn arrangements(
        &self,
        spring_idx: usize,
        cfg_idx: usize,
        cache: &mut HashMap<(usize, usize), u64>,
    ) -> u64 {
        if let Some(val) = cache.get(&(spring_idx, cfg_idx)) {
            return *val;
        }

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

            self.arrangements(spring_idx + c_len + 1, cfg_idx + 1, cache)
        });

        let skip = match self.springs.get(spring_idx) {
            None => u64::from(cfg_idx >= self.config.len()),
            Some(Status::Damaged) => 0,
            Some(_) => self.arrangements(spring_idx + 1, cfg_idx, cache),
        };

        let arrangements = consume_group + skip;
        cache.insert((spring_idx, cfg_idx), arrangements);

        return arrangements;
    }

    pub fn total_arrangements(&self) -> u64 {
        let mut cache = HashMap::new();
        return self.arrangements(0, 0, &mut cache);
    }

    pub fn expand(self) -> Self {
        let expanded_config = self.config.repeat(5);
        let mut expanded_springs = vec![];

        for i in 0..5 {
            expanded_springs.extend(self.springs.clone());
            if i != 4 {
                expanded_springs.push(Status::Unknown);
            }
        }

        return Self {
            springs: expanded_springs,
            config: expanded_config,
        };
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
            .map(|r| r.total_arrangements())
            .sum::<u64>();

        println!("Day 12 / Part 1: {total}");
    }

    fn part2(&mut self) {
        let records = self.records.clone();
        let total = records
            .into_iter()
            .map(|r| r.expand().total_arrangements())
            .sum::<u64>();

        println!("Day 12 / Part 2: {total}");
    }
}
