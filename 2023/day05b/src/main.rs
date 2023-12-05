use aoc_utils::bench;
use std::fs;

fn parse_map(map: &str) -> Vec<(usize, usize, usize)> {
    let mut res = Vec::new();
    for (i, line) in map.split('\n').enumerate() {
        if i == 0 {
            continue;
        }
        let data: Vec<usize> = line.split(' ').map(|x| x.parse().unwrap()).collect();
        res.push((data[0], data[1], data[2]));
    }
    res
}

fn lookup(seed: usize, map: &Vec<(usize, usize, usize)>) -> usize {
    for entry in map {
        if seed >= entry.1 && seed < entry.1 + entry.2 {
            return seed + entry.0 - entry.1;
        }
    }
    seed
}

fn solve() {
    let file = fs::read_to_string("./input.txt").unwrap();

    let lines = file.split("\n\n").collect::<Vec<_>>();
    let str_seeds: Vec<usize> = lines[0].split("seeds: ").collect::<Vec<_>>()[1]
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();
    let mut seeds: Vec<usize> = Vec::new();
    for seed_pair in str_seeds.chunks(2) {
        for seed in seed_pair[0]..(seed_pair[0] + seed_pair[1]) {
            seeds.push(seed);
        }
    }

    let str_seed_to_soil = lines[1];
    let str_soil_to_fert = lines[2];
    let str_fert_to_water = lines[3];
    let str_water_to_light = lines[4];
    let str_light_to_temp = lines[5];
    let str_temp_to_hum = lines[6];
    let str_hum_to_loc = lines[7];

    let seed_to_soil: Vec<(usize, usize, usize)> = parse_map(str_seed_to_soil);
    let soil_to_fert: Vec<(usize, usize, usize)> = parse_map(str_soil_to_fert);
    let fert_to_water: Vec<(usize, usize, usize)> = parse_map(str_fert_to_water);
    let water_to_light: Vec<(usize, usize, usize)> = parse_map(str_water_to_light);
    let light_to_temp: Vec<(usize, usize, usize)> = parse_map(str_light_to_temp);
    let temp_to_hum: Vec<(usize, usize, usize)> = parse_map(str_temp_to_hum);
    let hum_to_loc: Vec<(usize, usize, usize)> = parse_map(str_hum_to_loc);

    let mut locs = Vec::new();
    for seed in seeds {
        let soil = lookup(seed, &seed_to_soil);
        let fertilizer = lookup(soil, &soil_to_fert);
        let water = lookup(fertilizer, &fert_to_water);
        let light = lookup(water, &water_to_light);
        let temperature = lookup(light, &light_to_temp);
        let humidity = lookup(temperature, &temp_to_hum);
        let location = lookup(humidity, &hum_to_loc);
        locs.push(location);
    }
    println!("{}", locs.iter().min().unwrap());
}

fn main() {
    bench(solve)
    // Time taken 407.887827879s <-- oof.
}
