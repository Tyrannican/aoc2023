use crate::utils::*;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Card {
    winner: HashSet<i32>,
    numbers: HashSet<i32>,
}

impl Card {
    pub fn new(winner: &str, numbers: &str) -> Self {
        let winner = winner
            .split_whitespace()
            .map(|n| n.trim().parse::<i32>().unwrap())
            .collect::<HashSet<i32>>();

        let numbers = numbers
            .split_whitespace()
            .map(|n| n.trim().parse::<i32>().unwrap())
            .collect::<HashSet<i32>>();

        return Self { winner, numbers };
    }
}

pub struct Solution {
    cards: Vec<Card>,
}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self { cards: vec![] };
        sol.process_input("day04/input.txt");
        sol
    }
}

fn power_it(times: usize) -> usize {
    let mut total = 1;
    for _ in 0..times - 1 {
        total *= 2;
    }

    return total;
}

impl Solve for Solution {
    fn process_input(&mut self, path: &str) {
        for line in read_file(path).lines() {
            let (_, card_numbers) = line.split_once(": ").unwrap();
            let (winner, numbers) = card_numbers.split_once(" | ").unwrap();
            let card = Card::new(winner, numbers);
            self.cards.push(card);
        }
    }

    fn part1(&mut self) {
        let total = self
            .cards
            .iter()
            .map(|c| {
                let winners = c.winner.intersection(&c.numbers).count();
                if winners == 0 {
                    0
                } else {
                    power_it(winners)
                }
            })
            .sum::<usize>();

        println!("Day 04 / Part 1 - {total}");
    }

    fn part2(&mut self) {
        let mut stack = vec![];
        let mut total_cards = self.cards.len();
        for (idx, card) in self.cards.iter().enumerate() {
            let wins = card.winner.intersection(&card.numbers).count();
            for i in idx + 1..=idx + wins {
                stack.push((i, &self.cards[i]));
                total_cards += 1;
            }
        }

        while let Some((idx, card)) = stack.pop() {
            let wins = card.winner.intersection(&card.numbers).count();
            for i in idx + 1..=idx + wins {
                stack.push((i, &self.cards[i]));
                total_cards += 1;
            }
        }

        println!("Day 04 / Part 2: {total_cards}");
    }
}
