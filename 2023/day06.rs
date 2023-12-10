mod utils;

use std::iter::zip;
use utils::get_input;

fn parse_input(input: &str, spaces: bool) -> Vec<i128> {
    let parsed = input.split(':').nth(1).unwrap();
    let result = if !spaces {
        parsed.replace(" ", "")
    } else {
        parsed.replace(" ", " ") // This is silly, but I just want to appease the type system so I can move on...
    };

    result
        .split_whitespace()
        .map(|i| i.parse::<i128>().unwrap())
        .collect::<Vec<i128>>()
}

fn win_strategies(time: i128, distance: i128) -> i128 {
    let mut wins = 0;
    for i in 0..time {
        let new_dist = i * (time - i);
        if new_dist > distance {
            wins += 1;
        }
    }
    wins
}

fn main() {
    let input = get_input("inputs/06.txt");
    let parsed: Vec<Vec<i128>> = input.iter().map(|i| parse_input(i, false)).collect();

    let wins = zip(&parsed[0], &parsed[1]).fold(1, |prod, (t, d)| prod * win_strategies(*t, *d));

    println!("The solution for part 2 is: {:?}", wins);
}
