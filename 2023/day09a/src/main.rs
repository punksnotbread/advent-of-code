use aoc_utils::bench;
use std::fs;

fn parse_input() -> Vec<Vec<i32>> {
    return fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect();
}

fn solve() {
    let history = parse_input();
    let mut res: i32 = 0;

    for entry in history {
        let mut seqs = vec![];
        seqs.push(entry.clone());

        let mut current = entry;

        while !current.iter().all(|x| *x == 0) {
            let mut vecs = vec![];
            for (idx, value) in current.iter().enumerate() {
                if idx == current.len() - 1 {
                    continue;
                }
                vecs.push(current[idx + 1] - current[idx]);
            }
            seqs.push(vecs.clone());
            current = vecs;
            vecs = vec![];
        }

        let mut val: i32 = 0;
        for seq in seqs {
            val += seq.last().unwrap();
        }
        res += val;
    }
    println!("{res}");
}

fn main() {
    bench(solve)
}
