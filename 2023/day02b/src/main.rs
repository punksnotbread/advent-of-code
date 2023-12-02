use aoc_utils::bench;
use std::fs;

fn solve() {
    let file = fs::read_to_string("./input.txt").unwrap();
    let mut res: usize = 0;

    for row in file.lines() {
        let Some((_, cubes)) = row.split_once(": ") else {
            panic!("could not split {row}")
        };

        let mut smallest_red: usize = 0;
        let mut smallest_green: usize = 0;
        let mut smallest_blue: usize = 0;

        for game in cubes.split("; ") {
            for cube_selection in game.split(", ") {
                let Some((amount, color)) = cube_selection.split_once(" ") else {
                    panic!("could not split selection {cube_selection}")
                };
                match color {
                    "red" => {
                        let parsed_amount = amount.parse::<usize>().unwrap();
                        if parsed_amount > smallest_red {
                            smallest_red = parsed_amount
                        }
                    }
                    "green" => {
                        let parsed_amount = amount.parse::<usize>().unwrap();
                        if parsed_amount > smallest_green {
                            smallest_green = parsed_amount
                        }
                    }
                    "blue" => {
                        let parsed_amount = amount.parse::<usize>().unwrap();
                        if parsed_amount > smallest_blue {
                            smallest_blue = parsed_amount
                        }
                    }
                    _ => panic!("no color {color}"),
                }
            }
        }
        let power = smallest_red * smallest_green * smallest_blue;
        res += power
    }

    println!("{res}")
}

fn main() {
    bench(solve)
}
