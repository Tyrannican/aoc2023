use crate::utils::*;

type Coord = (isize, isize);

#[derive(Clone, PartialEq)]
enum Slot {
    Symbol(char),
    Number(char),
    Null,
}

#[derive(Debug)]
pub struct Part {
    num: i32,
    positions: Vec<Coord>,
}

#[derive(Debug)]
pub struct Symbol {
    glyph: char,
    position: Coord,
}

pub struct Solution {
    parts: Vec<Part>,
    symbols: Vec<Symbol>,
}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self {
            parts: vec![],
            symbols: vec![],
        };

        sol.process_input("day03/input.txt");
        sol
    }

    pub fn has_symbol(&self, x: isize, y: isize) -> bool {
        for i in x - 1..=x + 1 {
            for j in y - 1..=y + 1 {
                //
            }
        }

        return false;
    }

    pub fn valid_part(&self, part: &Part, symbol: &Symbol) -> bool {
        let (x, y) = symbol.position;
        for (n_x, n_y) in part.positions.iter() {
            if (x >= n_x - 1 && x <= n_x + 1) && (y >= n_y - 1 && y <= n_y + 1) {
                return true;
            }
        }
        return false;
    }
}

impl Solve for Solution {
    fn process_input(&mut self, path: &str) {
        let mut coords: Vec<Coord> = vec![];
        let mut part_number = String::new();

        let mut prev = '.';

        for (y, line) in read_file(path).lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                if char.is_numeric() {
                    part_number.push(char);
                    coords.push((x as isize, y as isize));
                } else {
                    if prev.is_numeric() {
                        let part = Part {
                            num: part_number.parse::<i32>().unwrap(),
                            positions: coords.drain(..).collect(),
                        };
                        self.parts.push(part);
                        part_number.clear();
                    }

                    if char != '.' {
                        let symbol = Symbol {
                            glyph: char,
                            position: (x as isize, y as isize),
                        };
                        self.symbols.push(symbol);
                    }
                }

                prev = char;
            }
        }
    }

    fn part1(&mut self) {
        let mut valid_parts = Vec::new();
        for symbol in self.symbols.iter() {
            for part in self.parts.iter() {
                if self.valid_part(part, symbol) {
                    valid_parts.push(part.num);
                }
            }
        }

        println!("Day 03 / Part 1 - {}", valid_parts.iter().sum::<i32>());
        // let mut parts: Vec<i32> = Vec::new();
        // let mut p_str = String::new();
        // let mut prev: Slot = Slot::Null;
        // let mut valid_num = false;

        // for y in 0..self.height {
        //     for x in 0..self.width {
        //         let idx = self.xy_idx(x, y);
        //         let current = self.schematic[idx].clone();
        //         match self.schematic[idx] {
        //             Slot::Number(c) => {
        //                 if self.has_symbol(x as isize, y as isize) {
        //                     valid_num = true;
        //                 }
        //                 p_str.push(c);
        //             }
        //             Slot::Null | Slot::Symbol(_) => match prev {
        //                 Slot::Number(_) => {
        //                     if valid_num {
        //                         let num = p_str.parse::<i32>().unwrap();
        //                         parts.push(num);
        //                     }
        //                     p_str.clear();
        //                     valid_num = false;
        //                 }
        //                 _ => {}
        //             },
        //         }

        //         prev = current;
        //     }
        // }

        // println!("Day 03/Part 1 - Total: {}", parts.iter().sum::<i32>());
    }

    fn part2(&mut self) {}
}
