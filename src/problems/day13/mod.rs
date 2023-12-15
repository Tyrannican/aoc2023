use crate::utils::*;

type MirrorIdx = (usize, usize);

#[derive(Debug, PartialEq)]
pub enum MirrorPlane {
    Horizontal(MirrorIdx),
    Vertical(MirrorIdx),
    None,
}

pub struct AshField {
    grid: Vec<Vec<char>>,
}

impl AshField {
    pub fn mirror_plane(&self) -> MirrorPlane {
        return MirrorPlane::None;
    }

    fn horizontal(&self) -> MirrorPlane {
        return MirrorPlane::None;
    }

    fn vertical(&self) -> MirrorPlane {
        return MirrorPlane::None;
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

    fn part1(&mut self) {}

    fn part2(&mut self) {}
}
