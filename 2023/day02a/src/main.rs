use aoc_utils::bench;
use std::fs;

const LIMIT_RED: usize = 12;
const LIMIT_GREEN: usize = 13;
const LIMIT_BLUE: usize = 14;

fn solve() {
    let file = fs::read_to_string("./input.txt").unwrap();
    let mut res: usize = 0;

    for (mut i, row) in file.lines().enumerate() {
        i += 1;
        let Some((_, cubes)) = row.split_once(": ") else {
            panic!("could not split {row}")
        };

        let mut game_is_eligible: bool = true;
        for game in cubes.split("; ") {
            for cube_selection in game.split(", ") {
                let Some((amount, color)) = cube_selection.split_once(" ") else {
                    panic!("could not split cube {cube_selection}")
                };
                match color {
                    "red" => {
                        if amount.parse::<usize>().unwrap() > LIMIT_RED {
                            game_is_eligible = false;
                            break;
                        }
                    }
                    "green" => {
                        if amount.parse::<usize>().unwrap() > LIMIT_GREEN {
                            game_is_eligible = false;
                            break;
                        }
                    }
                    "blue" => {
                        if amount.parse::<usize>().unwrap() > LIMIT_BLUE {
                            game_is_eligible = false;
                            break;
                        }
                    }
                    _ => panic!("no color {color}"),
                }
            }
            if !game_is_eligible {
                break;
            };
        }

        if game_is_eligible {
            res += i
        };
    }

    println!("{res}")
}

fn main() {
    bench(solve)
}
