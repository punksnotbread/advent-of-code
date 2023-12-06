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

fn parse_input(line: &str) -> Vec<usize> {
    return line
        .split_at(10)
        .1
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
}

fn solve() {
    let file = fs::read_to_string("./input.txt").unwrap();

    let lines = file.split("\n").collect::<Vec<_>>();
    let times: Vec<usize> = parse_input(lines[0]);
    let distances: Vec<usize> = parse_input(lines[1]);

    let mut results: Vec<_> = vec![];
    for (time, dist) in times.iter().zip(distances) {
        println!("{:?} {:?} {:?}", time, dist, get_wins(*time, dist));
        results.push(get_wins(*time, dist))
    }

    let mut res: usize = 1;
    for i in results {
        res *= i;
    }
    println!("{res}");
}

fn main() {
    bench(solve)
}
