use aoc_utils::bench;
use std::fs;

fn is_part_number(map: Vec<Vec<char>>, coords: (usize, usize)) -> bool {
    let checks: Vec<(i32, i32)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    for check in checks {
        let new_x = (coords.0 as i32 + check.0) as usize;
        let new_y = (coords.1 as i32 + check.1) as usize;

        // Check if new coords propose out of bounds.
        if new_x >= map[0].len() || new_y >= map.len() {
            continue;
        }

        let c = map[new_y][new_x];

        if c == '.' || c.is_digit(10) {
            continue;
        }

        return true;
    }
    return false;
}

fn solve() {
    let file = fs::read_to_string("./input.txt").unwrap();

    // Load map into matrix
    let mut map: Vec<Vec<char>> = vec![];
    for row in file.lines() {
        let mut row_els: Vec<char> = vec![];
        for char in row.chars() {
            row_els.push(char);
        }
        map.push(row_els);
    }

    let mut numbers: Vec<usize> = vec![];
    for (y, row) in map.iter().enumerate() {
        let mut number: String = "".to_string();
        let mut number_is_part_of_engine = false;
        for (x, char) in row.iter().enumerate() {
            if char.is_digit(10) {
                number.push(*char);
                if !number_is_part_of_engine {
                    number_is_part_of_engine = is_part_number(map.clone(), (x, y));
                }
            } else {
                if number.is_empty() {
                    continue;
                }
                if number_is_part_of_engine {
                    numbers.push(number.parse::<usize>().unwrap());
                }
                number = "".to_string();
                number_is_part_of_engine = false;
            }
        }

        // In case number ends at the end of line.
        if number.is_empty() {
            continue;
        }
        if number_is_part_of_engine {
            numbers.push(number.parse::<usize>().unwrap());
        }
    }

    // println!("{numbers:?}");
    let res: usize = numbers.iter().sum();
    println!("{res}");
}

fn main() {
    bench(solve)
    // 544664
}
