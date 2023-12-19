use std::{
    collections::{HashMap, HashSet},
    usize,
};

use crate::utils::*;

type MirrorIdx = (usize, usize);

#[derive(Debug, PartialEq)]
pub enum MirrorPlane {
    Horizontal(MirrorIdx),
    Vertical(MirrorIdx),
    None,
}

#[derive(Debug)]
pub struct AshField {
    grid: Vec<Vec<char>>,
}

impl AshField {
    pub fn print(&self) {
        for row in self.grid.iter() {
            for ch in row.iter() {
                print!("{}", ch);
            }
            println!();
        }
    }
    pub fn mirror_plane(&self) -> MirrorPlane {
        if let Some(horz) = self.horizontal() {
            return MirrorPlane::Horizontal(horz);
        } else if let Some(vert) = self.vertical() {
            return MirrorPlane::Vertical(vert);
        } else {
            return MirrorPlane::None;
        }
    }

    fn horizontal(&self) -> Option<(usize, usize)> {
        self.print();
        let mut checker: HashMap<String, (usize, usize)> = HashMap::new();
        for (pos, row) in self.grid.iter().enumerate() {
            let key = row.iter().collect::<String>();
            checker
                .entry(key)
                .and_modify(|e| e.1 = pos)
                .or_insert((pos, usize::MAX));
        }

        let filtered = checker
            .values()
            .filter_map(|v| {
                if v.1 != usize::MAX && i32::abs(v.0 as i32 - v.1 as i32) == 1 {
                    return Some(*v);
                }
                None
            })
            .collect::<Vec<(usize, usize)>>();

        println!("Horz filtered: {filtered:?}");
        if filtered.is_empty() || filtered.len() == 1 {
            return None;
        }

        return Some(filtered[0].clone());
    }

    fn vertical(&self) -> Option<(usize, usize)> {
        println!("Entered Vert");
        self.print();
        let mut verts = vec![];
        for i in (0..self.grid.len()).into_iter() {
            let mut inner = vec![];
            for row in self.grid.iter() {
                inner.push(row[i]);
            }
            verts.push(inner);
        }

        let mut checker: HashMap<String, (usize, usize)> = HashMap::new();
        for (pos, row) in verts.iter().enumerate() {
            let key = row.iter().collect::<String>();
            checker
                .entry(key)
                .and_modify(|e| e.1 = pos)
                .or_insert((pos, usize::MAX));
        }

        let filtered = checker
            .values()
            .filter_map(|v| {
                if v.1 != usize::MAX && i32::abs(v.0 as i32 - v.1 as i32) == 1 {
                    return Some(*v);
                }
                None
            })
            .collect::<Vec<(usize, usize)>>();

        println!("Vert filtered: {filtered:?}");

        if filtered.is_empty() || filtered.len() == 1 {
            return None;
        }

        return Some(filtered[0].clone());
    }
}

pub struct Solution {
    ash_fields: Vec<AshField>,
}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self { ash_fields: vec![] };
        sol.process_input("day13/input.txt");
        sol
    }
}

impl Solve for Solution {
    fn process_input(&mut self, path: &str) {
        self.ash_fields = read_file(path)
            .split("\n\n")
            .map(|f| AshField {
                grid: f
                    .lines()
                    .map(|l| l.chars().collect::<Vec<char>>())
                    .collect::<Vec<Vec<char>>>(),
            })
            .collect::<Vec<AshField>>();
    }

    fn part1(&mut self) {
        for ash_field in self.ash_fields.iter() {
            ash_field.mirror_plane();
        }
    }

    fn part2(&mut self) {}
}
