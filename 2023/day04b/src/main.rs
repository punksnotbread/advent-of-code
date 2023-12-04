use aoc_utils::bench;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;

fn solve() {
    let file = fs::read_to_string("./input.txt").unwrap();

    let re = Regex::new(r"Card.*: (?P<winning>.*) \| (?P<owned>.*)").unwrap();
    let mut multipliers: HashMap<i32, i32> = HashMap::from([(1, 1)]);

    for (mut game_no, row) in file.lines().enumerate() {
        game_no += 1;

        let caps = re.captures(row).unwrap();

        let (winning, owned): (HashSet<i32>, HashSet<i32>) = (
            caps["winning"]
                .trim()
                .split_whitespace()
                .map(|s| s.parse::<i32>().ok().expect("parse panic"))
                .collect(),
            caps["owned"]
                .trim()
                .split_whitespace()
                .map(|s| s.parse::<i32>().ok().expect("parse panic"))
                .collect(),
        );

        let intersection = winning.intersection(&owned).collect::<Vec<_>>();

        // Run n times the result addition
        for _ in 0..*multipliers.entry(game_no as i32).or_insert(1) {
            // Add all upcoming copies
            for idx in game_no + 1..game_no + 1 + intersection.len() {
                let game_copy = multipliers.entry(idx as i32).or_insert(1);
                *game_copy += 1;
            }
        }
    }

    let res: i32 = multipliers.values().sum();
    println!("{res:?}")
}

fn main() {
    bench(solve)
}
