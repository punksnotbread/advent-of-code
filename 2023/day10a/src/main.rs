use aoc_utils::bench;
use std::fs;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
struct Coordinate {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
    Start,
}

fn find_next(
    visited: &Vec<(Coordinate, Direction)>,
    current: &Coordinate,
    map: &Vec<Vec<char>>,
) -> (Coordinate, Direction) {
    let directions: Vec<(Direction, (i32, i32))> = vec![
        (Direction::Down, (1, 0)),
        (Direction::Right, (0, 1)),
        (Direction::Up, (-1, 0)),
        (Direction::Left, (0, -1)),
    ];

    let last_char: char = if visited.is_empty() {
        'S'
    } else {
        let last_coords = visited.last().unwrap();
        map[last_coords.0.y as usize][last_coords.0.x as usize]
    };

    let last_dir: Direction = if visited.is_empty() {
        Direction::Start
    } else {
        visited.last().unwrap().1
    };

    for (direction, direction_coords) in directions {
        let new_coord = Coordinate {
            x: current.x + direction_coords.1,
            y: current.y + direction_coords.0,
        };
        //println!("Trying {:?} {:?}", new_coord, direction);

        // Boundary checks.
        if new_coord.x < 0
            || new_coord.y < 0
            || new_coord.x > map[0].len() as i32 - 1
            || new_coord.y > map.len() as i32 - 1
        {
            //println!("Skipping out-of-bounds visit.");
            continue;
        }

        let new_char = map[new_coord.y as usize][new_coord.x as usize];

        // No-op.
        if new_char == '.' {
            //println!("New character is {new_char}, skipping");
            continue;
        }

        // Don't try going via previous direction.
        if direction == Direction::Up && last_dir == Direction::Down {
            continue;
        }
        if direction == Direction::Down && last_dir == Direction::Up {
            continue;
        }
        if direction == Direction::Left && last_dir == Direction::Right {
            continue;
        }
        if direction == Direction::Right && last_dir == Direction::Left {
            continue;
        }

        // Don't try and visit illogical directions.
        if direction == Direction::Up && !"S|JL".contains(|c| c == last_char) {
            //println!("Can't go via {last_coord} because of direction {direction}");
            continue;
        }
        if direction == Direction::Down && !"S|7F".contains(|c| c == last_char) {
            //println!("Can't go via {last_coord} because of direction {direction}");
            continue;
        }
        if direction == Direction::Right && !"S-LF".contains(|c| c == last_char) {
            //println!("Can't go via {last_coord} because of direction {direction}");
            continue;
        }
        if direction == Direction::Left && !"S-7J".contains(|c| c == last_char) {
            //println!("Can't go via {last_coord} because of direction {direction}");
            continue;
        }

        // Don't visit same coord.
        if visited.iter().any(|&(first, _)| first == new_coord) {
            if visited[0].0 == new_coord {
                //println!("Found finishing node.");
                return (new_coord, direction);
            }
            continue;
        }

        match direction {
            Direction::Up => match new_char {
                '|' => return (new_coord, direction),
                'F' => return (new_coord, direction),
                '7' => return (new_coord, direction),
                _ => continue,
            },
            Direction::Down => match new_char {
                '|' => return (new_coord, direction),
                'J' => return (new_coord, direction),
                'L' => return (new_coord, direction),
                _ => continue,
            },
            Direction::Left => match new_char {
                '-' => return (new_coord, direction),
                'L' => return (new_coord, direction),
                'F' => return (new_coord, direction),
                _ => continue,
            },
            Direction::Right => match new_char {
                '-' => return (new_coord, direction),
                'J' => return (new_coord, direction),
                '7' => return (new_coord, direction),
                _ => continue,
            },
            _ => unreachable!(),
        };
    }

    unreachable!();
}

fn solve() {
    let input = fs::read_to_string("./input.txt");
    let mut start_coords: Coordinate = Coordinate { x: 0, y: 0 };
    let mut map: Vec<Vec<char>> = vec![];
    for (row, entry) in input.unwrap().lines().enumerate() {
        for (col, char) in entry.chars().enumerate() {
            if char == 'S' {
                start_coords = Coordinate {
                    x: col as i32,
                    y: row as i32,
                };
                break;
            }
        }
        map.push(entry.chars().collect());
    }

    let mut visited: Vec<(Coordinate, Direction)> = vec![];
    let mut current = start_coords.clone();
    visited.push((current.clone(), Direction::Start));
    let mut steps = 0;
    loop {
        let (new_current, direction) = find_next(&visited, &current, &map);
        current = new_current;
        steps += 1;
        //println!(
        //    "-- Visited {current:?} {:?} {steps}",
        //    map[current.y as usize][current.x as usize]
        //);

        if current == start_coords {
            break;
        }

        visited.push((current.clone(), direction));
    }
    println!("{}", steps / 2);
}

fn main() {
    bench(solve)
}
