use crate::utils::*;

type Coord = (i64, i64);

pub struct Solution {
    map: Vec<Vec<char>>,
    galaxies: Vec<Coord>,
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,
}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self {
            map: vec![],
            galaxies: vec![],
            empty_rows: vec![],
            empty_cols: vec![],
        };
        sol.process_input("day11/input.txt");
        sol
    }

    pub fn make_galaxy_pairs(&self) -> Vec<(Coord, Coord)> {
        let mut pairs = vec![];
        for i in 0..self.galaxies.len() - 1 {
            for j in i + 1..self.galaxies.len() {
                pairs.push((self.galaxies[i], self.galaxies[j]));
            }
        }

        return pairs;
    }

    pub fn calculate_distance(&self, mut p1: Coord, mut p2: Coord, scale: i64) -> i64 {
        let mut incr = 0;
        for row in self.empty_rows.iter() {
            if p1.0 > *row as i64 + incr {
                p1.0 += scale;
            }

            if p2.0 > *row as i64 + incr {
                p2.0 += scale;
            }

            incr += scale;
        }

        incr = 0;
        for col in self.empty_cols.iter() {
            if p1.1 > *col as i64 + incr {
                p1.1 += scale;
            }

            if p2.1 > *col as i64 + incr {
                p2.1 += scale;
            }

            incr += scale;
        }

        let x_diff = i64::abs(p1.0 - p2.0);
        let y_diff = i64::abs(p1.1 - p2.1);

        return x_diff + y_diff;
    }
}

impl Solve for Solution {
    fn process_input(&mut self, path: &str) {
        let space = read_file(path)
            .lines()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        self.map = space;
        for (x, row) in self.map.iter().enumerate() {
            for (y, col) in row.iter().enumerate() {
                if *col == '#' {
                    self.galaxies.push((x as i64, y as i64));
                }
            }
        }

        self.empty_rows = self
            .map
            .iter()
            .enumerate()
            .filter_map(|(idx, row)| {
                if row.into_iter().all(|c| *c == '.') {
                    return Some(idx);
                }

                None
            })
            .collect::<Vec<usize>>();

        self.empty_cols = (0..self.map[0].len())
            .into_iter()
            .filter_map(|i| {
                let col = self.map.iter().map(|l| l[i]).collect::<Vec<char>>();

                if col.into_iter().all(|c| c == '.') {
                    return Some(i);
                }

                return None;
            })
            .collect::<Vec<usize>>();
    }

    fn part1(&mut self) {
        let pairs = self.make_galaxy_pairs();

        let total = pairs
            .iter()
            .map(|p| {
                let (p1, p2) = *p;
                self.calculate_distance(p1, p2, 1)
            })
            .sum::<i64>();

        println!("Day 11 / Part 1: {total}");
    }

    fn part2(&mut self) {}
}
