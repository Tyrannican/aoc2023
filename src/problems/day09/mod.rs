use crate::utils::*;

pub struct Solution {
    histories: Vec<Vec<i64>>,
}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self { histories: vec![] };
        sol.process_input("day09/input.txt");
        sol
    }
}

impl Solve for Solution {
    fn process_input(&mut self, path: &str) {
        for line in read_file(path).lines() {
            let history = line.split(' ').map(|n| n.parse::<i64>().unwrap()).collect();
            self.histories.push(history);
        }
    }

    fn part1(&mut self) {
        let histories = self.histories.clone();

        let total = histories.into_iter().map(|h| predict_next(h)).sum::<i64>();
        println!("Day 09 / Part 1: {total}");
    }

    fn part2(&mut self) {
        let histories = self.histories.clone();

        let total = histories.into_iter().map(|h| predict_first(h)).sum::<i64>();
        println!("Day 09 / Part 2: {total}");
    }
}

fn diffs(v: Vec<i64>) -> Vec<i64> {
    return (1..v.len()).into_iter().map(|i| v[i] - v[i - 1]).collect();
}

fn predict_next(mut history: Vec<i64>) -> i64 {
    let mut values = vec![*history.last().unwrap()];

    loop {
        history = diffs(history);
        if history.iter().all(|x| *x == 0) {
            break;
        }
        values.push(*history.last().unwrap());
    }

    return values.iter().sum::<i64>();
}

fn predict_first(mut history: Vec<i64>) -> i64 {
    let mut prev = 0;
    let mut values = vec![*history.first().unwrap()];

    loop {
        history = diffs(history);
        if history.iter().all(|x| *x == 0) {
            break;
        }
        values.insert(0, *history.first().unwrap());
    }

    values.into_iter().for_each(|n| prev = n - prev);

    return prev;
}
