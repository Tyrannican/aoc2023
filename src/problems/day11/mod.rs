use crate::utils::*;

type Coord = (i32, i32);

pub struct Solution {
    map: Vec<Vec<char>>,
    galaxies: Vec<Coord>,
}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self {
            map: vec![],
            galaxies: vec![],
        };
        sol.process_input("day11/input.txt");
        sol
    }
}

impl Solve for Solution {
    fn process_input(&mut self, path: &str) {
        let space = read_file(path)
            .lines()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        self.map = expand_space(space);
        for (x, row) in self.map.iter().enumerate() {
            for (y, col) in row.iter().enumerate() {
                if *col == '#' {
                    self.galaxies.push((x as i32, y as i32));
                }
            }
        }
    }

    fn part1(&mut self) {
        let mut pairs = vec![];
        for i in 0..self.galaxies.len() - 1 {
            for j in i + 1..self.galaxies.len() {
                pairs.push((self.galaxies[i], self.galaxies[j]));
            }
        }

        let total = pairs
            .iter()
            .map(|p| {
                let (p1, p2) = *p;
                let x_diff = i32::abs(p1.0 - p2.0);
                let y_diff = i32::abs(p1.1 - p2.1);

                x_diff + y_diff
            })
            .sum::<i32>();

        println!("Day 11 / Part 1: {total}");
    }

    fn part2(&mut self) {}
}

fn expand_space(space: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut empty_row_idxs = space
        .iter()
        .enumerate()
        .filter_map(|(idx, row)| {
            if row.into_iter().all(|c| *c == '.') {
                return Some(idx);
            }

            None
        })
        .collect::<Vec<usize>>();

    let empty_col_idxs = (0..space[0].len())
        .into_iter()
        .filter_map(|i| {
            let col = space.iter().map(|l| l[i]).collect::<Vec<char>>();

            if col.into_iter().all(|c| c == '.') {
                return Some(i);
            }

            return None;
        })
        .collect::<Vec<usize>>();

    let mut expanded = vec![];
    for row in space.iter() {
        let mut inner = vec![];
        for (i, col) in row.iter().enumerate() {
            if empty_col_idxs.contains(&i) {
                inner.push('.');
            }
            inner.push(*col);
        }
        expanded.push(inner);
    }

    let empty_row = (0..expanded[0].len())
        .into_iter()
        .map(|_| '.')
        .collect::<Vec<char>>();

    let mut diff = 0;
    for idx in empty_row_idxs.iter_mut() {
        *idx -= diff;
        diff -= 1;
        expanded.insert(*idx, empty_row.clone());
    }

    return expanded;
}
