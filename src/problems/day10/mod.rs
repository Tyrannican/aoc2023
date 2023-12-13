use crate::utils::*;
use kurve::Kurve;
use std::collections::HashSet;

type Coord = (usize, usize);

pub struct Solution {
    start: Coord,
    map: Kurve<Coord, char>,
    input: Vec<Vec<Coord>>,
}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self {
            start: (0, 0),
            map: Kurve::new(),
            input: vec![],
        };
        sol.process_input("day10/input.txt");
        sol
    }

    fn find_loop(&self) -> Vec<Coord> {
        let mut current = self.start.clone();
        let mut seen: HashSet<Coord> = HashSet::new();
        let mut path = vec![];
        loop {
            seen.insert(current);
            let neighbors: Vec<Coord> = self
                .map
                .get_neighbors(current)
                .unwrap()
                .into_keys()
                .collect();

            let mut iter = neighbors.iter();
            let mut next = iter.next().unwrap();

            let mut finished = false;
            while seen.contains(next) {
                match iter.next() {
                    Some(n) => next = n,
                    None => {
                        finished = true;
                        break;
                    }
                }
            }

            path.push(next.clone());
            if finished {
                break;
            }

            current = *next;
        }

        return path;
    }
}

impl Solve for Solution {
    fn process_input(&mut self, path: &str) {
        for (i, line) in read_file(path).lines().enumerate() {
            let mut inner = vec![];
            for (j, char) in line.chars().enumerate() {
                let src = (i, j);
                inner.push(src);
                if char == '.' {
                    continue;
                }
                self.map.add_vertex(src, char);

                match char {
                    '|' => {
                        self.map.add_edge(src, (i + 1, j));
                        self.map.add_edge(src, (i - 1, j));
                    }
                    '-' => {
                        self.map.add_edge(src, (i, j - 1));
                        self.map.add_edge(src, (i, j + 1));
                    }
                    'L' => {
                        self.map.add_edge(src, (i - 1, j));
                        self.map.add_edge(src, (i, j + 1));
                    }
                    'J' => {
                        self.map.add_edge(src, (i, j - 1));
                        self.map.add_edge(src, (i - 1, j));
                    }
                    '7' => {
                        self.map.add_edge(src, (i, j - 1));
                        self.map.add_edge(src, (i + 1, j));
                    }
                    'F' => {
                        self.map.add_edge(src, (i, j + 1));
                        self.map.add_edge(src, (i + 1, j));
                    }
                    'S' => self.start = src,
                    _ => {}
                }
            }
            self.input.push(inner);
        }

        // Set neighbors for the start position (S)
        let start = self.start.clone();
        let vertices = self.map.get_all_neighbors();
        for (vtx, n) in vertices.into_iter() {
            if n.contains_key(&start) {
                self.map.add_edge(start, vtx);
            }
        }
    }

    fn part1(&mut self) {
        let path = self.find_loop();
        println!("Day 10 / Part 1: {}", path.len() / 2);
    }

    // Off by one at times
    fn part2(&mut self) {
        let path = self.find_loop();

        let n = path.len();
        let (mut left_sum, mut right_sum) = (0, 0);

        for i in 0..n {
            let j = (i + 1) % n;
            left_sum += path[i].0 * path[j].1;
            right_sum += path[j].0 * path[i].1;
        }

        let area = (left_sum as i32 - right_sum as i32).abs() / 2;
        let inside = area - (n as i32 / 2) + 1;
        println!("Day 10 / Part 2: {inside} <-- (Can be off by one...)");
    }
}
