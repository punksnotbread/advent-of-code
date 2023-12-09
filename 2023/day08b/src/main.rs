use aoc_utils::bench;
use num_integer::lcm;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone)]
struct Node {
    left: String,
    right: String,
}

impl Node {
    fn new(left: &str, right: &str) -> Self {
        Node {
            left: left.to_string(),
            right: right.to_string(),
        }
    }
}
fn parse_input() -> (String, HashMap<String, Node>) {
    let file = fs::read_to_string("./input.txt").unwrap();

    let mut lines = file.split("\n").collect::<Vec<_>>();
    let commands = lines[0].to_string();
    lines.remove(0);
    lines.remove(0);
    let re = Regex::new(r"(?<node>.*) = \((?<left>.*), (?<right>.*)\)").unwrap();

    let mut nodes: HashMap<String, Node> = HashMap::new();

    for line in lines {
        let caps = re.captures(line).unwrap();
        let node = caps["node"].trim().to_string();
        let left = caps["left"].trim();
        let right = caps["right"].trim();
        nodes.insert(node, Node::new(left, right));
    }

    (commands, nodes)
}
fn solve() {
    let (commands, nodes) = parse_input();

    // Get all starting nodes.
    let starting_nodes: Vec<_> = nodes.keys().filter(|&key| key.ends_with("A")).collect();
    let mut node_steps: Vec<i32> = Vec::new();

    for starting_node in starting_nodes {
        let mut steps = 0;
        let mut current = starting_node;

        for command in commands.chars().cycle() {
            match command {
                'L' => current = &nodes[current].left,
                'R' => current = &nodes[current].right,
                _ => panic!("Invalid instruction: {}", command),
            }

            steps += 1;

            if current.ends_with("Z") {
                node_steps.push(steps);
                break;
            }
        }
    }

    let mut steps: u64 = 1;
    for step in node_steps {
        steps = lcm(step as u64, steps);
    }
    println!("{steps:?}");
}

fn main() {
    bench(solve)
}
