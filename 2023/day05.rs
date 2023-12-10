mod utils;

use std::ops::Range;
use utils::get_input;

fn parse_maps(maps_raw: &[String]) -> Vec<Vec<Vec<i64>>> {
    let mut map: Vec<Vec<i64>> = Vec::new();
    let mut maps: Vec<Vec<Vec<i64>>> = Vec::new();

    for i in maps_raw {
        let j = i.as_str();
        let numeric = match j {
            _ if i.chars().all(|c| c.is_numeric() || c.is_whitespace()) => {
                j.split_whitespace()
                    .map(|i| i.parse::<i64>().unwrap()) // convert to i64
                    .collect()
            } // find the lines which contain matricies
            _ => vec![], // toss out everything else
        };
        if numeric.is_empty() {
            if !map.is_empty() {
                maps.push(map);
                map = Vec::new();
            }
        } else {
            map.push(numeric)
        }
    }
    maps
}

fn parse_seeds(seeds_raw: &String) -> Vec<i64> {
    let seeds = seeds_raw
        .split(":")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|i| i.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    seeds
}

fn spread_seeds(mut seeds: Vec<i64>) -> Vec<Range<i64>> {
    let mut all_seeds = Vec::new();

    while seeds.len() >= 2 {
        let start = seeds.remove(0);
        let count = seeds.remove(0);
        let full_spread = start..start + count;
        all_seeds.push(full_spread);
    }

    all_seeds
}

fn map_seeds(maps: &Vec<Vec<Vec<i64>>>, seeds: &Vec<i64>) -> i64 {
    let mut soil_numbers: Vec<i64> = vec![];
    for i in seeds {
        let mut seed_loc = *i;

        for map in maps {
            for rec in map {
                if (seed_loc >= rec[1]) && (seed_loc < (rec[1] + rec[2])) {
                    // matched a map
                    seed_loc += rec[0] - rec[1];
                    break;
                }
            }
        }
        soil_numbers.push(seed_loc);
    }
    let min = soil_numbers.iter().min();
    *min.unwrap()
}

fn main() {
    let example = get_input("examples/05.txt");

    let seeds_raw = &example[0];
    let maps_raw = &example[1..example.len()];

    let maps = parse_maps(maps_raw);
    let seeds = parse_seeds(seeds_raw);

    let example_min = map_seeds(&maps, &seeds);
    assert_eq!(example_min, 35);

    let example = get_input("inputs/05.txt");

    let seeds_raw = &example[0];
    let maps_raw = &example[1..example.len()];

    let maps = parse_maps(maps_raw);
    let seeds = parse_seeds(seeds_raw);

    let example_min = map_seeds(&maps, &seeds);

    println!("The solution for part 1 is: {:?}", example_min);

    let example = get_input("examples/05.txt");

    let seeds_raw = &example[0];
    let maps_raw = &example[1..example.len()];

    let maps = parse_maps(maps_raw);
    let seeds = parse_seeds(seeds_raw);
    let spread_seeds = spread_seeds(seeds);

    let min = spread_seeds
        .iter()
        .map(|i| map_seeds(&maps, &i.clone().collect()))
        .min()
        .unwrap();

    assert_eq!(min, 46);

    let example = get_input("inputs/05.txt");

    let seeds_raw = &example[0];
    let maps_raw = &example[1..example.len()];

    let maps = parse_maps(maps_raw);
    let seeds = parse_seeds(seeds_raw);
    let spread_seeds = spread_seeds(seeds);

    let min = spread_seeds
        .iter()
        .map(|i| map_seeds(&maps, &i.clone().collect()))
        .min()
        .unwrap();

    println!("The solution for part 2 is: {:?}", min);
}
