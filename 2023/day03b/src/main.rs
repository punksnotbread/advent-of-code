use std::collections::HashMap;
use aoc_utils::bench;
use std::fs;

fn is_touching_gear(map: Vec<Vec<char>>, coords: (usize, usize)) -> (i32, i32) {
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

        if c == '*' {
            return (new_x as i32, new_y as i32);
        }
    }
    return (-1, -1);
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

    let mut gears: HashMap<(i32, i32), Vec<usize>>= HashMap::new();

    for (y, row) in map.iter().enumerate() {
        let mut number: String = "".to_string();
        let mut possible_gear_coords: (i32, i32) = (-1, -1);
        for (x, char) in row.iter().enumerate() {
            if char.is_digit(10) {
                number.push(*char);
                if possible_gear_coords.0 == -1 {
                    possible_gear_coords = is_touching_gear(map.clone(), (x, y));
                }
            } else {
                if number.is_empty() {
                    continue;
                }
                if possible_gear_coords.0 != -1 {
                    let parsed_number = number.parse::<usize>().unwrap();
                    gears.entry(possible_gear_coords).or_insert(vec![]).push(parsed_number)
                }
                number = "".to_string();
                possible_gear_coords = (-1, -1);
            }
        }

        // In case number ends at the end of line.
        if number.is_empty() {
            continue;
        }

        if possible_gear_coords.0 != -1 {
            let parsed_number = number.parse::<usize>().unwrap();
            gears.entry(possible_gear_coords).or_insert(vec![]).push(parsed_number)
        }
    }

    let mut sums: Vec<usize> = vec![];
    for gear in gears.iter() {
        if gear.1.len() == 2 {
            let sum: usize = gear.1[0] * gear.1[1];
            sums.push(sum);
        }
    }
    let res: usize = sums.iter().sum();
    println!("{res:?}");
}

fn main() {
    bench(solve)
    // 544664
}
