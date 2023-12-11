use aoc_utils::bench;
use std::collections::HashSet;
use std::fs;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
struct Coordinate {
    x: i32,
    y: i32,
}

const DISTANCE: i32 = 1_000_000;

fn manhattan_distance(
    coord1: Coordinate,
    coord2: Coordinate,
    no_galaxy_x: &Vec<i32>,
    no_galaxy_y: &Vec<i32>,
) -> i128 {
    let mut final_coord1 = Coordinate {
        x: coord1.x,
        y: coord1.y,
    };
    let mut final_coord2 = Coordinate {
        x: coord2.x,
        y: coord2.y,
    };

    if coord1.x < coord2.x {
        for x in coord1.x..coord2.x {
            if no_galaxy_x.contains(&x) {
                // println!("Adding distance to final_coord2.x {final_coord2:?} for x {x}");
                final_coord2.x += DISTANCE - 1;
            }
        }
    } else if coord1.x > coord2.x {
        for x in coord2.x..coord1.x {
            if no_galaxy_x.contains(&x) {
                // println!("Adding distance to final_coord1.x {final_coord1:?} for x {x}");
                final_coord1.x += DISTANCE - 1;
            }
        }
    }
    if coord1.y < coord2.y {
        for x in coord1.y..coord2.y {
            if no_galaxy_y.contains(&x) {
                // println!("Adding distance to final_coord2.y {final_coord1:?} for y {x}");
                final_coord2.y += DISTANCE - 1;
            }
        }
    } else if coord1.y > coord2.y {
        for x in coord2.y..coord1.y {
            if no_galaxy_y.contains(&x) {
                // println!("Adding distance to final_coord1.x {final_coord1:?} for y {x}");
                final_coord1.y += DISTANCE - 1;
            }
        }
    }

    let res: i128 =
        ((final_coord1.x - final_coord2.x).abs() + (final_coord1.y - final_coord2.y).abs()) as i128;
    // println!("Comparing {final_coord1:?} againts {final_coord2:?}, result {res}");

    res
}

fn solve() {
    let input = fs::read_to_string("./input.txt");

    let mut galaxy_locations: Vec<Coordinate> = vec![];
    let mut map: Vec<Vec<char>> = vec![];
    let mut no_galaxy_y: Vec<i32> = vec![];
    for (row_no, row) in input.unwrap().lines().enumerate() {
        map.push(row.chars().collect());
        if !row.contains('#') {
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
    println!("Len of no galaxy y: {}", no_galaxy_y.len());
    let galaxy_height: i32 = map[0].len() as i32;

    let mut no_galaxy_x: Vec<i32> = vec![];
    let unique_x_values: HashSet<_> = galaxy_locations.iter().map(|coord| coord.x).collect();
    for col in 0..galaxy_height {
        if !unique_x_values.contains(&col) {
            no_galaxy_x.push(col as i32);
        }
    }
    println!("Len of no galaxy x: {}", no_galaxy_x.len());

    println!("Cols: {no_galaxy_x:?}");
    println!("Rows: {no_galaxy_y:?}");

    let mut res: i128 = 0;

    for i in 0..galaxy_locations.len() {
        for j in i + 1..galaxy_locations.len() {
            let distance = manhattan_distance(
                galaxy_locations[i],
                galaxy_locations[j],
                &no_galaxy_x,
                &no_galaxy_y,
            );

            res += distance;
        }
    }
    println!("{res}");
}

fn main() {
    bench(solve)
}
