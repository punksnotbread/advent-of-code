use std::collections::HashSet;
use aoc_utils::bench;
use regex::Regex;
use std::fs;

fn solve() {
    let file = fs::read_to_string("./input.txt").unwrap();

    let re = Regex::new(r"Card.*: (?<winning>.*) \| (?<owned>.*)").unwrap();
    let mut result: i32 = 0;

    for row in file.lines() {
        let caps = re.captures(row).unwrap();

        let winning = &caps["winning"]
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<i32>().ok().expect("parse panic"))
            .collect::<HashSet<_>>();

        let owned = &caps["owned"]
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<i32>().ok().expect("parse panic"))
            .collect::<HashSet<_>>();

        let intersection = winning.intersection(&owned).collect::<Vec<_>>();

        if intersection.len() > 0 {
            result += i32::pow(2, (intersection.len() - 1) as u32);
        }

    }
    println!("{result}")
}

fn main() {
    bench(solve)
}
