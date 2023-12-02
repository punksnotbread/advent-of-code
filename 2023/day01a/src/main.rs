use std::fs;
use aoc_utils::bench;

fn solution() {
    let file = fs::read_to_string("./input.txt").unwrap();
    let mut res: i32 = 0;
    for row in file.lines() {
        let mut row_res : String = "".to_string();

        for char in row.chars() {
            if char.is_digit(10) {
                row_res.push(char);
                break
            }
        }

        for char in row.chars().rev() {
            if char.is_digit(10) {
                row_res.push(char);
                break
            }
        }

        let row_res_int: i32 = row_res.parse().unwrap();

        res = res + row_res_int;
    }

    println!("{res}")
}

fn main() {
    bench(solution)
}
