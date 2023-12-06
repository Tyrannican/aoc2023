use crate::utils::*;
use std::sync::{Arc, Mutex};
use std::thread;

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

    pub fn convert_seeds_to_ranges(&mut self) -> Vec<(i64, i64)> {
        return self
            .seeds
            .chunks(2)
            .map(|chunk| (chunk[0], chunk[1]))
            .collect();
    }
}

pub fn get_final_location(maps: &Vec<Vec<MapRange>>, mut seed: i64) -> i64 {
    for map in maps.iter() {
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
            .map(|s| get_final_location(&self.maps, *s))
            .min()
            .unwrap_or(-1);

        println!("Day 05 / Part 1: {lowest}");
    }

    // There's an obvious trick here that i'm too dumb to recognise
    // so brute force go brrrrrrrrr
    fn part2(&mut self) {
        let seeds: Vec<_> = self
            .seeds
            .clone()
            .chunks(2)
            .map(|chunk| chunk[0]..chunk[0] + chunk[1])
            .collect();

        let mut hdls = vec![];
        let min = Arc::new(Mutex::new(i64::MAX));
        for ranges in seeds.iter() {
            let maps = self.maps.clone();
            let ranges = ranges.clone();
            let min = Arc::clone(&min);
            hdls.push(thread::spawn(move || {
                for seed in ranges.into_iter() {
                    let loc = get_final_location(&maps, seed);
                    let mut value = min.lock().unwrap();
                    if loc < *value {
                        *value = loc;
                    }
                }
            }));
        }

        for hdl in hdls {
            let _ = hdl.join();
        }

        let lock = Arc::try_unwrap(min).expect("Unable to unwrap ARC");
        let answer = lock.into_inner().expect("mutex broken");

        println!("Day 05 / Part 2: {answer}");
    }
}
