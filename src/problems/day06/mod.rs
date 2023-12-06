use crate::utils::*;

pub struct Solution {
    times: Vec<i64>,
    distances: Vec<i64>,
}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self {
            times: vec![],
            distances: vec![],
        };
        sol.process_input("day06/input.txt");
        sol
    }
}

fn convert_to_single_num(v: &Vec<i64>) -> i64 {
    let mut output = String::new();
    for number in v.iter() {
        output.push_str(&number.to_string());
    }

    return output.parse::<i64>().unwrap();
}

fn race_check(time: i64, distance: i64) -> i64 {
    let mut mid = time / 2;
    let mut total = mid * i64::abs(mid - time);
    let mut wins = 0;

    while total > distance {
        wins += 1;
        mid -= 1;
        total = mid * i64::abs(mid - time);
    }

    if time % 2 != 0 {
        wins *= 2;
    } else {
        wins = (wins * 2) - 1;
    }

    return wins;
}

impl Solve for Solution {
    fn process_input(&mut self, path: &str) {
        for line in read_file(path).lines() {
            let mut items: Vec<_> = line.split_whitespace().collect();
            if items.remove(0).contains("Time") {
                self.times = items
                    .into_iter()
                    .map(|t| t.parse::<i64>().unwrap())
                    .collect();
            } else {
                self.distances = items
                    .into_iter()
                    .map(|t| t.parse::<i64>().unwrap())
                    .collect();
            }
        }
    }

    fn part1(&mut self) {
        let answer: i64 = self
            .times
            .iter()
            .zip(self.distances.iter())
            .map(|(time, distance)| race_check(*time, *distance))
            .product();

        println!("Day 06 / Part 1: {answer}");
    }

    fn part2(&mut self) {
        let time = convert_to_single_num(&self.times);
        let distance = convert_to_single_num(&self.distances);

        let wins = race_check(time, distance);
        println!("Day 06 / Part 2: {wins}");
    }
}
