use aoc_utils::bench;
use std::fs;

fn get_wins(time: usize, dist: usize) -> usize {
    let mut wins: usize = 0;
    for hold in 0..time {
        if hold * (time - hold) > dist {
            wins += 1
        }
    }

    wins
}

fn parse_input(line: &str) -> usize {
    return line
        .split_at(10)
        .1
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<usize>>()
        .into_iter()
        .map(|digit| digit.to_string())
        .collect::<String>()
        .parse()
        .unwrap();
}

fn solve() {
    let file = fs::read_to_string("./input.txt").unwrap();

    let lines = file.split("\n").collect::<Vec<_>>();
    let times: usize = parse_input(lines[0]);
    let distances: usize = parse_input(lines[1]);

    println!("{}", get_wins(times, distances));
}

fn main() {
    bench(solve)
}
