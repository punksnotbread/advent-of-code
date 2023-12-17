use aoc_utils::bench;
use std::fs;

fn hash(command: String) -> i32 {
    let mut curr: i32 = 0;
    for c in command.chars() {
        curr += c as i32;
        curr *= 17;
        curr %= 256;
    }

    curr
}

fn solve() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let commands: Vec<_> = input.split(',').collect();
    let res: i32 = commands
        .iter()
        .map(|command| hash(command.to_string()))
        .sum();

    println!("{:?}", res);
}

fn main() {
    assert_eq!(hash("HASH".to_string()), 52);

    bench(solve)
}
