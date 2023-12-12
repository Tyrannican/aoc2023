use crate::utils::*;
use kurve::Kurve;
use std::collections::HashSet;

type Coord = (usize, usize);

pub struct Solution {
    start: Coord,
    map: Kurve<Coord, char>,
}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self {
            start: (0, 0),
            map: Kurve::new(),
        };
        sol.process_input("day10/input.txt");
        sol
    }

    fn find_loop(&self) -> HashSet<Coord> {
        let mut current = self.start.clone();
        let mut seen: HashSet<Coord> = HashSet::new();
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

            if finished && iter.next().is_none() {
                break;
            }

            current = *next;
        }

        return seen;
    }
}

impl Solve for Solution {
    fn process_input(&mut self, path: &str) {
        for (i, line) in read_file(path).lines().enumerate() {
            for (j, char) in line.chars().enumerate() {
                let src = (i, j);
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
        println!("Seen: {}", path.len() / 2);
    }

    fn part2(&mut self) {}
}
