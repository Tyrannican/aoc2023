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
        let mut total = 0;
        let histories = self.histories.clone();

        for history in histories.into_iter() {
            total += predict_next(history);
        }
        println!("Total: {total}");
    }

    fn part2(&mut self) {}
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
