use crate::utils::*;

#[derive(Clone, PartialEq)]
enum Slot {
    Symbol(char),
    Number(char),
    Null,
}

pub struct Solution {
    width: usize,
    height: usize,
    schematic: Vec<Slot>,
}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self {
            schematic: Vec::default(),
            width: 0,
            height: 0,
        };

        sol.process_input("day03/input.txt");
        sol
    }

    pub fn has_symbol(&self, x: isize, y: isize) -> bool {
        for i in x - 1..=x + 1 {
            for j in y - 1..=y + 1 {
                if i < 0 || i >= self.height as isize || j < 0 || j >= self.width as isize {
                    continue;
                }

                let idx = self.xy_idx(i as usize, j as usize);
                match self.schematic[idx] {
                    Slot::Symbol(_) => return true,
                    _ => {}
                }
            }
        }

        return false;
    }

    fn xy_idx(&self, x: usize, y: usize) -> usize {
        ((y * self.width) + x) as usize
    }

    fn display_schematic(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = self.xy_idx(x, y);
                match self.schematic[idx] {
                    Slot::Symbol(s) => print!("{}", s),
                    Slot::Number(num) => print!("{num}"),
                    Slot::Null => print!("."),
                }
            }
            println!();
        }
    }
}

impl Solve for Solution {
    fn process_input(&mut self, path: &str) {
        let raw = read_file(path);
        let lines = raw
            .split('\n')
            .filter(|l| !l.is_empty())
            .collect::<Vec<&str>>();

        self.height = lines.len();
        self.width = lines[0].len();

        for line in lines {
            for char in line.chars() {
                if char.is_numeric() {
                    self.schematic.push(Slot::Number(char));
                } else if char == '.' {
                    self.schematic.push(Slot::Null);
                } else {
                    self.schematic.push(Slot::Symbol(char));
                }
            }
        }
    }

    fn part1(&mut self) {
        let mut parts: Vec<i32> = Vec::new();
        let mut p_str = String::new();
        let mut prev: Slot = Slot::Null;
        let mut valid_num = false;

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = self.xy_idx(x, y);
                let current = self.schematic[idx].clone();
                match self.schematic[idx] {
                    Slot::Number(c) => {
                        if self.has_symbol(x as isize, y as isize) {
                            valid_num = true;
                        }
                        p_str.push(c);
                    }
                    Slot::Null | Slot::Symbol(_) => match prev {
                        Slot::Number(_) => {
                            if valid_num {
                                let num = p_str.parse::<i32>().unwrap();
                                parts.push(num);
                            }
                            p_str.clear();
                            valid_num = false;
                        }
                        _ => {}
                    },
                }

                prev = current;
            }
        }

        println!("Day 03/Part 1 - Total: {}", parts.iter().sum::<i32>());
    }

    fn part2(&mut self) {}
}
