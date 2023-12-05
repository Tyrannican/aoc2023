use crate::utils::*;

type MapRange = (i64, i64, i64);

pub struct Solution {
    seeds: Vec<i64>,
    maps: Vec<Vec<MapRange>>,
}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self {
            seeds: vec![],
            maps: vec![],
        };
        sol.process_input("day05/input.txt");
        sol
    }

    pub fn get_final_location(&self, mut seed: i64) -> i64 {
        for map in self.maps.iter() {
            for ranges in map.into_iter() {
                let (dst, src, rng) = ranges;
                if seed >= *src && seed <= *src + rng {
                    let offset = i64::abs(src - dst);
                    let value = if src < dst {
                        seed + offset
                    } else {
                        seed - offset
                    };
                    seed = value;
                    break;
                }
            }
        }

        return seed;
    }
}

fn get_seeds(s: &str) -> Vec<i64> {
    let (_, seeds) = s.split_once(": ").unwrap();
    return seeds
        .split_whitespace()
        .into_iter()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
}

fn build_map_range(s: &str) -> MapRange {
    let ranges = s
        .split_whitespace()
        .into_iter()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    return (ranges[0], ranges[1], ranges[2]);
}

impl Solve for Solution {
    fn process_input(&mut self, path: &str) {
        let raw = read_file(path);
        let mut maps = raw.split("\n\n").collect::<Vec<&str>>();
        self.seeds = get_seeds(&maps.remove(0));

        self.maps = maps
            .iter()
            .map(|m| {
                let mut split = m
                    .split("\n")
                    .filter_map(|s| {
                        if s.is_empty() {
                            return None;
                        }

                        return Some(s);
                    })
                    .collect::<Vec<&str>>();

                // Ditch the name
                split.remove(0);
                split.into_iter().map(|n| build_map_range(n)).collect()
            })
            .collect::<Vec<Vec<MapRange>>>();
    }

    fn part1(&mut self) {
        let lowest = self
            .seeds
            .iter()
            .map(|s| self.get_final_location(*s))
            .min()
            .unwrap_or(-1);

        println!("Day 05 / Part 1: {lowest}");
    }

    fn part2(&mut self) {}
}
