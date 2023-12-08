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

    pub fn custom_cmp(&self, other: &Self) -> Ordering {
        use CardValue::*;
        match (&self, &other) {
            (A, A) | (K, K) | (Q, Q) | (J, J) | (T, T) => Ordering::Equal,
            (A, _) => Ordering::Greater,
            (_, A) => Ordering::Less,
            (K, _) => Ordering::Greater,
            (_, K) => Ordering::Less,
            (Q, _) => Ordering::Greater,
            (_, Q) => Ordering::Less,
            (T, _) => Ordering::Greater,
            (_, T) => Ordering::Less,
            (Num(i), Num(j)) => i.cmp(j),
            (J, _) => Ordering::Less,
            (_, J) => Ordering::Greater,
        }
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
            (A, _) => Ordering::Greater,
            (_, A) => Ordering::Less,
            (K, _) => Ordering::Greater,
            (_, K) => Ordering::Less,
            (Q, _) => Ordering::Greater,
            (_, Q) => Ordering::Less,
            (J, _) => Ordering::Greater,
            (_, J) => Ordering::Less,
            (T, _) => Ordering::Greater,
            (_, T) => Ordering::Less,
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
    // Stepwise check
    pub fn is_better_than(&self, other: &Self) -> Ordering {
        for (v, o) in self.values.iter().zip(other.values.iter()) {
            if v == o {
                continue;
            }

            return v.cmp(o);
        }

        return Ordering::Equal;
    }

    // Custom rule for Jokers
    fn is_better_than_revised(&self, other: &Self) -> Ordering {
        for (v, o) in self.values.iter().zip(other.values.iter()) {
            if v == o {
                continue;
            }

            return v.custom_cmp(o);
        }

        return Ordering::Equal;
    }

    // Recalculate based on the Joker rule in Part 2
    fn reevaluate_hand(&mut self) {
        if !self.values.contains(&CardValue::J) {
            return;
        }

        let mut ranked_hand = self.values.clone();
        ranked_hand.sort_by(|a, b| b.custom_cmp(&a));
        let mut highest_rank = &ranked_hand[0];

        let mut counts = HashMap::new();

        for value in self.values.iter() {
            counts.entry(value).and_modify(|v| *v += 1).or_insert(1);
        }

        match self.hand_type {
            HandType::Pair => {
                for (k, v) in counts.iter() {
                    if *v == 2 && *k != &CardValue::J {
                        highest_rank = k;
                    }
                }
            }
            HandType::TwoPair => {
                for (k, v) in counts.iter() {
                    if *v == 2 && *k != &CardValue::J {
                        highest_rank = k;
                    }
                }
            }
            HandType::ThreeKind => {
                for (k, v) in counts.iter() {
                    if *v == 3 && *k != &CardValue::J {
                        highest_rank = k;
                    }
                }
            }
            _ => {}
        }

        let mut check_values = self.values.clone();
        for (pos, value) in self.values.iter().enumerate() {
            if value == &CardValue::J {
                check_values[pos] = highest_rank.clone();
            }
        }

        let new_hand = HandType::from_cards(&check_values);

        self.hand_type = new_hand;
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

    pub fn reset(&mut self) {
        self.process_input("day07/input.txt");
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
        self.reset();
        let mut result = 0;
        let mut current_rank = 1;

        self.hands.iter_mut().for_each(|h| h.reevaluate_hand());
        self.hands.sort_by(|a, b| {
            if a.hand_type == b.hand_type {
                return a.is_better_than_revised(b);
            }

            a.hand_type.cmp(&b.hand_type)
        });

        for hand in self.hands.iter() {
            result += current_rank * hand.bid;
            current_rank += 1;
        }

        println!("Day 07 / Part 2: {result}");
    }
}
