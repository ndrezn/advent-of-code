mod utils;

use std::collections::HashMap;
use utils::get_input;

fn parse_input(l: &str) -> Vec<HashMap<String, i32>> {
    let raw_cubes = l.split(':').nth(1).unwrap().split("; ");
    raw_cubes
        .map(|i| {
            let mut draw = HashMap::new();
            let raw_draw = i.split(", ");

            for j in raw_draw {
                let cube: Vec<&str> = j.split_whitespace().collect();
                if cube.len() == 2 {
                    if let Ok(count) = cube[0].parse::<i32>() {
                        draw.insert(cube[1].to_string(), count);
                    }
                }
            }
            draw
        })
        .collect::<Vec<HashMap<String, i32>>>()
}

fn is_valid_game(game: &[HashMap<String, i32>], valid_game: &HashMap<&str, i32>) -> bool {
    for color in vec!["green", "red", "blue"].iter() {
        for draw in game {
            if let Some(draw_color) = draw.get(*color) {
                if *draw_color > *valid_game.get(*color).unwrap() {
                    return false;
                }
            }
        }
    }
    true
}

fn pow(game: &[HashMap<String, i32>]) -> i32 {
    let mut min_draw = HashMap::new();
    for color in &["green", "red", "blue"] {
        let mut min_color = 0;
        for draw in game {
            if let Some(draw_count) = draw.get(*color) {
                if *draw_count > min_color {
                    min_color = *draw_count;
                }
            }
        }
        min_draw.insert(*color, min_color);
    }
    min_draw.values().product()
}

fn main() {
    let example = get_input("inputs/02.txt");
    let parsed: Vec<Vec<HashMap<String, i32>>> = example.iter().map(|i| parse_input(i)).collect();

    // filter where the sum of the colored cubes in each game matches:
    // 12 red cubes, 13 green cubes, and 14 blue cubes
    let mut valid_game: HashMap<&str, i32> = HashMap::new();
    valid_game.insert("red", 12);
    valid_game.insert("green", 13);
    valid_game.insert("blue", 14);

    let possible_games = parsed.iter().enumerate().fold(0, |acc, (index, game)| {
        if is_valid_game(&game, &valid_game) {
            acc + (index + 1)
        } else {
            acc // Keep the previous accumulator value
        }
    });
    println!("The solution for part 1 is: {:?}", possible_games);

    let example = get_input("inputs/02.txt");
    let parsed: Vec<Vec<HashMap<String, i32>>> = example.iter().map(|i| parse_input(i)).collect();

    let cube_sums: i32 = parsed.iter().map(|game| pow(game)).sum();

    println!("The solution for part 2 is: {:?}", cube_sums);
}
