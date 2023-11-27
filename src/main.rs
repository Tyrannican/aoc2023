mod cli;
mod utils;

#[allow(dead_code)]
mod problems;

use cli::*;
use utils::Solve;

pub fn solve(mut solution: Box<dyn Solve>) {
    solution.part1();
    solution.part2();
}

fn main() {
    let cli = Cli::parse();

    let solution: Box<dyn Solve> = match cli.day {
        1 => Box::new(problems::day01::Solution::new()),
        2 => Box::new(problems::day02::Solution::new()),
        3 => Box::new(problems::day03::Solution::new()),
        4 => Box::new(problems::day04::Solution::new()),
        5 => Box::new(problems::day05::Solution::new()),
        6 => Box::new(problems::day06::Solution::new()),
        7 => Box::new(problems::day07::Solution::new()),
        8 => Box::new(problems::day08::Solution::new()),
        9 => Box::new(problems::day09::Solution::new()),
        10 => Box::new(problems::day10::Solution::new()),
        11 => Box::new(problems::day11::Solution::new()),
        12 => Box::new(problems::day12::Solution::new()),
        13 => Box::new(problems::day13::Solution::new()),
        14 => Box::new(problems::day14::Solution::new()),
        15 => Box::new(problems::day15::Solution::new()),
        16 => Box::new(problems::day16::Solution::new()),
        17 => Box::new(problems::day17::Solution::new()),
        18 => Box::new(problems::day18::Solution::new()),
        19 => Box::new(problems::day19::Solution::new()),
        20 => Box::new(problems::day20::Solution::new()),
        21 => Box::new(problems::day21::Solution::new()),
        22 => Box::new(problems::day22::Solution::new()),
        23 => Box::new(problems::day23::Solution::new()),
        24 => Box::new(problems::day24::Solution::new()),
        25 => Box::new(problems::day25::Solution::new()),
        _ => {
            println!("Day {:0>2} is not a valid AoC day!", cli.day);
            std::process::exit(0);
        }
    };

    solve(solution);
}
