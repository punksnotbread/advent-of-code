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

        let jokers = cards_in_hand.iter().filter(|&&x| x == 0).count() as u8;

        if jokers == 5 {
            cards_in_hand = vec![14, 14, 14, 14, 14];
            hands.push(Hand {
                cards: cards_in_hand,
                htype: HandType::FiveOfAKind,
                bid: bid.parse::<i32>().unwrap(),
            });
        } else {
            let mut frequencies =
                cards_in_hand
                    .iter()
                    .copied()
                    .fold(HashMap::new(), |mut map, val| {
                        map.entry(val)
                            .and_modify(|frq| *frq += 1)
                            .or_insert(1);
                        map
                    });

            frequencies.remove(&0);
            let max_key = *frequencies.iter().max_by_key(|entry| entry.1).unwrap().0;
            *frequencies.entry(max_key).or_insert(0) += jokers;

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
            // cards_in_hand
            //     .iter_mut()
            //     .filter(|x| **x == 0)
            //     .for_each(|x| *x = 11);

            hands.push(Hand {
                cards: cards_in_hand,
                htype: hand_type,
                bid: bid.parse::<i32>().unwrap(),
            });
        }
    }

    hands.sort();
    for hand in &hands {
        println!("{hand:?}");
    }

    let res: i32 = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + (i + 1) as i32 * hand.bid);

    // assert_eq!(res, 253473930);
    println!("{res:?}");
}

fn main() {
    bench(solve)
}
