use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    let mut res: i32 = 0;
    for row in file.lines() {
        let mut row_res : String = "".to_string();
        let mut vec: Vec<_> = vec![];
        for number in [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
            "1", "2", "3", "4", "5", "6", "7", "8", "9"
        ] {
            vec.append(&mut row.match_indices(number).collect());
        }
        vec.sort_by_key(|k| k.0);

        let first = &vec[0];
        let last = &vec[vec.len() - 1];

        for tuple in [first, last] {
            let char = tuple.1;
            if char.len() == 1 {
                row_res = [row_res, char.to_string()].join("");
            } else {
                match char {
                    "one" => row_res.push('1'),
                    "two" => row_res.push('2'),
                    "three" => row_res.push('3'),
                    "four" => row_res.push('4'),
                    "five" => row_res.push('5'),
                    "six" => row_res.push('6'),
                    "seven" => row_res.push('7'),
                    "eight" => row_res.push('8'),
                    "nine" => row_res.push('9'),
                    _ => panic!("could not match found number"),
                }
            }
        }

        let row_res_int: i32 = row_res.parse().unwrap();

        res = res + row_res_int;
    }

    println!("{res}")
}
