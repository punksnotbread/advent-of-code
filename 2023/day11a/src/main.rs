use aoc_utils::bench;
use std::collections::HashSet;
use std::fs;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
struct Coordinate {
    x: i32,
    y: i32,
}

fn manhattan_distance(coord1: Coordinate, coord2: Coordinate) -> i32 {
    (coord1.x - coord2.x).abs() + (coord1.y - coord2.y).abs()
}

fn solve() {
    let input = fs::read_to_string("./input.txt");

    let mut galaxy_locations: Vec<Coordinate> = vec![];
    let mut map: Vec<Vec<char>> = vec![];
    let mut no_galaxy_y: Vec<i32> = vec![];
    for (row_no, row) in input.unwrap().lines().enumerate() {
        map.push(row.chars().collect());
        if !row.contains('#') {
            map.push(row.chars().collect());
            no_galaxy_y.push(row_no as i32);
        }
        for (col_no, col) in row.chars().enumerate() {
            if col == '#' {
                galaxy_locations.push(Coordinate {
                    x: col_no as i32,
                    y: row_no as i32,
                });
            }
        }
    }
    let galaxy_height: i32 = map[0].len() as i32;

    let mut no_galaxy_x: Vec<i32> = vec![];
    let unique_x_values: HashSet<_> = galaxy_locations.iter().map(|coord| coord.x).collect();
    for col in 0..galaxy_height {
        if !unique_x_values.contains(&col) {
            no_galaxy_x.push(col as i32);
        }
    }

    // Expand rows.
    let mut expanded_map: Vec<Vec<char>> = vec![];
    for (_, row) in map.iter().enumerate() {
        let mut expanded_row = row.clone();
        for (col_no, _) in row.iter().enumerate().rev() {
            if no_galaxy_x.contains(&(col_no as i32)) {
                expanded_row.insert(col_no + 1, '.');
            }
        }
        expanded_map.push(expanded_row.clone());
        let row_str: String = expanded_row.iter().collect();
        println!("{row_str}");
    }

    let mut expanded_galaxy_locations: Vec<Coordinate> = vec![];
    for (row_no, row) in expanded_map.iter().enumerate() {
        for (col_no, col) in row.iter().enumerate() {
            if col == &'#' {
                expanded_galaxy_locations.push(Coordinate {
                    x: col_no as i32,
                    y: row_no as i32,
                });
            }
        }
    }

    println!("Cols: {no_galaxy_x:?}");
    println!("Rows: {no_galaxy_y:?}");
    println!("Galaxy_locs: {expanded_galaxy_locations:?}");

    let mut res: i32 = 0;

    for i in 0..expanded_galaxy_locations.len() {
        for j in i + 1..expanded_galaxy_locations.len() {
            let distance =
                manhattan_distance(expanded_galaxy_locations[i], expanded_galaxy_locations[j]);

            res += distance;
        }
    }
    println!("{res}");
}

fn main() {
    bench(solve)
}
