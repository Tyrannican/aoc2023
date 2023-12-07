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
    pub fn from_cards(cards: &Vec<CardValue>) -> Self {
        let mut count: HashMap<CardValue, usize> = HashMap::new();
        for value in cards.iter() {
            count
                .entry(value.clone())
                .and_modify(|e| *e += 1)
                .or_insert(1);
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
impl Eq for HandType {}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}
impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        use HandType::*;

        match (&self, &other) {
            (FiveKind, FiveKind)
            | (FourKind, FourKind)
            | (FullHouse, FullHouse)
            | (ThreeKind, ThreeKind)
            | (TwoPair, TwoPair)
            | (Pair, Pair)
            | (High, High) => Ordering::Equal,
            (FiveKind, _) => std::cmp::Ordering::Greater,
            (_, FiveKind) => std::cmp::Ordering::Less,
            (FourKind, _) => std::cmp::Ordering::Greater,
            (_, FourKind) => std::cmp::Ordering::Less,
            (FullHouse, _) => std::cmp::Ordering::Greater,
            (_, FullHouse) => std::cmp::Ordering::Less,
            (ThreeKind, _) => std::cmp::Ordering::Greater,
            (_, ThreeKind) => std::cmp::Ordering::Less,
            (TwoPair, _) => std::cmp::Ordering::Greater,
            (_, TwoPair) => std::cmp::Ordering::Less,
            (Pair, _) => std::cmp::Ordering::Greater,
            (_, Pair) => std::cmp::Ordering::Less,
        }
    }
}

// MAybe
#[derive(Debug, PartialEq, Clone, Hash)]
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

#[derive(Debug, Clone)]
struct Hand {
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

    // Recalculate based on the Joker rule in Part 2
    // Todo:: fix
    fn reevaluate_hand(&mut self) {
        let mut values = self.values.clone();
        let mut counter: HashMap<CardValue, usize> = HashMap::new();
        for value in values.clone().into_iter() {
            counter.entry(value).and_modify(|v| *v += 1).or_insert(1);
        }

        // Determine how to deal with issues when there is only one of a given type
        // e.g. [2, Q, A, J, 5] -> [2, Q, A, J, 5]
        if counter.contains_key(&CardValue::J) {
            let (mut largest_key, total) = counter.iter().max_by_key(|k| k.1).unwrap();
            for (pos, value) in self.values.iter().enumerate() {
                if *value == CardValue::J {
                    values[pos] = largest_key.clone();
                }
            }
        }

        self.hand_type = HandType::from_cards(&values);
    }
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

                let values = CardValue::cards_to_values(hand);
                let hand = Hand {
                    values: values.clone(),
                    bid,
                    hand_type: HandType::from_cards(&values),
                };

                hand
            })
            .collect();
    }

    fn part1(&mut self) {
        let mut result = 0;
        let mut current_rank = 1;

        self.hands.sort_by(|a, b| {
            if a.hand_type == b.hand_type {
                return a.is_better_than(b);
            }
            a.hand_type.cmp(&b.hand_type)
        });

        for hand in self.hands.iter() {
            result += current_rank * hand.bid;
            current_rank += 1;
        }

        println!("Day 07 / Part 1: {result}");
    }

    fn part2(&mut self) {
        let mut result = 0;
        let mut current_rank = 1;

        self.hands.iter_mut().for_each(|h| h.reevaluate_hand());
        self.hands.sort_by(|a, b| {
            if a.hand_type == b.hand_type {
                return a.is_better_than(b);
            }

            a.hand_type.cmp(&b.hand_type)
        });

        for hand in self.hands.iter() {
            result += current_rank * hand.bid;
            current_rank += 1;
        }
    }
}
