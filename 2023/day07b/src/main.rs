use aoc_utils::bench;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
enum HandType {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

use std::ops::Add;

impl Add<u8> for HandType {
    type Output = HandType;

    fn add(self, rhs: u8) -> HandType {
        let new_value = (self as u8 + rhs).min(6);
        match new_value {
            0 => HandType::HighCard,
            1 => HandType::OnePair,
            2 => HandType::TwoPair,
            3 => HandType::ThreeOfAKind,
            4 => HandType::FourOfAKind,
            5 => HandType::FourOfAKind,
            6 => HandType::FiveOfAKind,
            _ => panic!("Unsupported HandType value"),
        }
    }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
struct Hand {
    htype: HandType,
    cards: Vec<u8>,
    bid: i32,
}

fn solve() {
    let file = fs::read_to_string("./input.txt").unwrap();

    let mut hands: Vec<Hand> = Vec::new();

    for line in file.lines() {
        let [hand, bid]: [&str; 2] = line.split(" ").collect::<Vec<&str>>().try_into().unwrap();

        let mut cards_in_hand: Vec<u8> = Vec::new();
        for card in hand.chars() {
            if card.is_digit(10) {
                cards_in_hand.push(card as u8 - 48)
            } else {
                match card {
                    'T' => cards_in_hand.push(10),
                    'J' => cards_in_hand.push(0),
                    'Q' => cards_in_hand.push(12),
                    'K' => cards_in_hand.push(13),
                    'A' => cards_in_hand.push(14),
                    _ => unreachable!(),
                }
            }
        }

        let frequencies = cards_in_hand
            .iter()
            .copied()
            .fold(HashMap::new(), |mut map, val| {
                map.entry(val).and_modify(|frq| *frq += 1).or_insert(1);
                map
            });

        let mut hand_type: HandType = match frequencies.len() {
            1 => HandType::FiveOfAKind,
            2 => match frequencies.values().any(|&x| x == 4) {
                true => HandType::FourOfAKind,
                _ => HandType::FullHouse,
            },
            3 => match frequencies.values().any(|&x| x == 3) {
                true => HandType::ThreeOfAKind,
                _ => HandType::TwoPair,
            },
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => unreachable!(),
        };

        let jokers = cards_in_hand.iter().filter(|&&x| x == 0).count() as u8;
        hand_type = hand_type + jokers;

        hands.push(Hand {
            cards: cards_in_hand,
            htype: hand_type,
            bid: bid.parse::<i32>().unwrap(),
        });
    }

    hands.sort();

    let res: i32 = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + (i + 1) as i32 * hand.bid);

    println!("{res:?}");
}

fn main() {
    bench(solve)
}
