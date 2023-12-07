use crate::utils::*;
use std::cmp::{Eq, Ord, Ordering, PartialOrd};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
enum HandType {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    Pair,
    High,
}

impl HandType {
    pub fn from_cards(cards: &str) -> Self {
        let mut count: HashMap<char, usize> = HashMap::new();
        for char in cards.chars() {
            count.entry(char).and_modify(|e| *e += 1).or_insert(1);
        }

        let values: Vec<usize> = count.values().into_iter().map(|c| *c).collect();
        match count.len() {
            1 => Self::FiveKind,
            2 => {
                if values.contains(&4) {
                    Self::FourKind
                } else {
                    Self::FullHouse
                }
            }
            3 => {
                if values.contains(&3) {
                    Self::ThreeKind
                } else {
                    Self::TwoPair
                }
            }
            4 => Self::Pair,
            _ => Self::High,
        }
    }
}

// MAybe
#[derive(Debug, PartialEq, Clone)]
enum CardValue {
    A,
    K,
    Q,
    J,
    T,
    Num(i32),
}

impl CardValue {
    pub fn from_char(ch: char) -> Self {
        match ch {
            'A' => CardValue::A,
            'K' => CardValue::K,
            'Q' => CardValue::Q,
            'J' => CardValue::J,
            'T' => CardValue::T,
            _ => {
                let num = ch.to_digit(10).unwrap();
                return CardValue::Num(num as i32);
            }
        }
    }

    pub fn cards_to_values(cards: &str) -> Vec<Self> {
        return cards.chars().map(|c| CardValue::from_char(c)).collect();
    }
}

impl Eq for CardValue {}

impl PartialOrd for CardValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}
impl Ord for CardValue {
    fn cmp(&self, other: &Self) -> Ordering {
        use CardValue::*;

        match (&self, &other) {
            (A, A) | (K, K) | (Q, Q) | (J, J) | (T, T) => Ordering::Equal,
            (A, _) => std::cmp::Ordering::Greater,
            (_, A) => std::cmp::Ordering::Less,
            (K, _) => std::cmp::Ordering::Greater,
            (_, K) => std::cmp::Ordering::Less,
            (Q, _) => std::cmp::Ordering::Greater,
            (_, Q) => std::cmp::Ordering::Less,
            (J, _) => std::cmp::Ordering::Greater,
            (_, J) => std::cmp::Ordering::Less,
            (T, _) => std::cmp::Ordering::Greater,
            (_, T) => std::cmp::Ordering::Less,
            (Num(i), Num(j)) => i.cmp(j),
        }
    }
}

const HAND_ORDER: [HandType; 7] = [
    HandType::High,
    HandType::Pair,
    HandType::TwoPair,
    HandType::ThreeKind,
    HandType::FullHouse,
    HandType::FourKind,
    HandType::FiveKind,
];

#[derive(Debug, Clone)]
struct Hand {
    cards: String,
    values: Vec<CardValue>,
    bid: i32,
    hand_type: HandType,
}

impl Hand {
    pub fn is_better_than(&self, other: &Self) -> Ordering {
        for (v, o) in self.values.iter().zip(other.values.iter()) {
            if v == o {
                continue;
            } else if v > o {
                return Ordering::Greater;
            }

            return Ordering::Less;
        }

        return Ordering::Equal;
    }
}

fn rank_hands(hands: &mut Vec<Hand>) {
    hands.sort_by(|a, b| a.is_better_than(b));
}

pub struct Solution {
    hands: Vec<Hand>,
}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self { hands: vec![] };
        sol.process_input("day07/input.txt");
        sol
    }
}

impl Solve for Solution {
    fn process_input(&mut self, path: &str) {
        self.hands = read_file(path)
            .lines()
            .map(|line| {
                let (hand, bid) = line.split_once(' ').unwrap();
                let bid = bid.parse::<i32>().unwrap();

                let hand = Hand {
                    cards: hand.to_string(),
                    values: CardValue::cards_to_values(hand),
                    bid,
                    hand_type: HandType::from_cards(hand),
                };

                hand
            })
            .collect();
    }

    fn part1(&mut self) {
        let mut result = 0;
        let mut current_rank = 1;
        for ht in HAND_ORDER.iter() {
            let mut hands: Vec<Hand> = self
                .hands
                .iter()
                .filter_map(|c| {
                    if c.hand_type == *ht {
                        return Some(c.clone());
                    }

                    None
                })
                .collect();

            rank_hands(&mut hands);
            for hand in hands.into_iter() {
                result += current_rank * hand.bid;
                current_rank += 1;
            }
        }

        println!("Day 07 / Part 1: {result}");
    }

    fn part2(&mut self) {}
}
