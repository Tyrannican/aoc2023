use crate::utils::*;

pub struct Solution {
    codes: Vec<String>,
}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self { codes: vec![] };

        sol.process_input("day01/input.txt");
        sol
    }
}

fn sliding_window(word: &mut String) {
    let mut final_str = String::new();
    let arr = word.chars().collect::<Vec<char>>();

    for i in 0..arr.len() {
        for j in i..=arr.len() {
            if i == j && arr[i].is_numeric() {
                final_str.push(arr[i]);
            } else {
                add_to_final(&arr[i..j], &mut final_str);
            }
        }
    }

    if final_str.is_empty() {
        for ch in arr.iter() {
            if ch.is_numeric() {
                final_str.push(*ch);
            }
        }
    }
    *word = final_str;
}

fn add_to_final(window: &[char], s: &mut String) {
    let num = window.iter().collect::<String>();
    match num.as_str() {
        "one" => s.push('1'),
        "two" => s.push('2'),
        "three" => s.push('3'),
        "four" => s.push('4'),
        "five" => s.push('5'),
        "six" => s.push('6'),
        "seven" => s.push('7'),
        "eight" => s.push('8'),
        "nine" => s.push('9'),
        _ => {}
    }
}

impl Solve for Solution {
    fn process_input(&mut self, path: &str) {
        let raw = read_file(path);
        self.codes = raw
            .split('\n')
            .filter(|l| !l.is_empty())
            .map(|l| l.to_string())
            .collect::<Vec<String>>();
    }

    fn part1(&mut self) {
        let total: i32 = self
            .codes
            .iter()
            .map(|code| {
                let numbers = code
                    .chars()
                    .into_iter()
                    .filter(|c| c.is_numeric())
                    .collect::<Vec<char>>();

                format!("{}{}", numbers[0], numbers[numbers.len() - 1])
                    .parse::<i32>()
                    .unwrap()
            })
            .sum();

        println!("Part 1: {total}");
    }

    fn part2(&mut self) {
        let total: i32 = self
            .codes
            .iter_mut()
            .map(|code| {
                sliding_window(code);
                let numbers = code
                    .chars()
                    .into_iter()
                    .filter(|c| c.is_numeric())
                    .collect::<Vec<char>>();

                format!("{}{}", numbers[0], numbers[numbers.len() - 1])
                    .parse::<i32>()
                    .unwrap()
            })
            .sum();

        println!("Part 2: {total}");
    }
}
