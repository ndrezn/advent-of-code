mod utils;

use std::cmp;
use std::num::ParseIntError;
use utils::get_input;

// lol
fn breed_cards(wins: impl Iterator<Item = i32>) -> i32 {
    let wins_vec: Vec<_> = wins.collect();
    let cards_count = wins_vec.len();
    let mut card_counts: Vec<i32> = vec![1; cards_count]; // We have one of each card at the start

    for (idx, win) in wins_vec.iter().enumerate() {
        let win_usize: usize = *win as usize;
        for i in (idx + 1..cmp::min(1 + idx + win_usize, cards_count)).step_by(1) {
            card_counts[i] += card_counts[idx]
        }
    }

    let total_cards: i32 = card_counts.into_iter().sum();

    total_cards
}

fn count_winners(card: &Result<Vec<Vec<i32>>, ParseIntError>) -> i32 {
    let lists = card.as_ref().expect("Failed to get card");
    let mut pts = 0;
    for &num in &lists[0] {
        if lists[1].contains(&num) {
            pts += 1;
        }
    }
    pts
}

fn generate_vectors(line: &str, split: usize) -> Result<Vec<Vec<i32>>, ParseIntError> {
    let cards = line[split..line.len()].split("|");
    let parsed_cards = cards
        .map(|c| {
            c.split_whitespace()
                .map(|i| i.parse::<i32>())
                .collect::<Result<Vec<i32>, _>>()
        })
        .collect();
    parsed_cards
}

fn exponent(wins: i32) -> i32 {
    let base: i32 = 2;
    let result: i32;
    match wins {
        0 => result = 0,
        _ => result = base.pow(wins as u32 - 1),
    }

    result
}

fn main() {
    let part_one_example = get_input("examples/04.txt").into_iter();
    let sol: i32 = part_one_example
        .map(|i| generate_vectors(&i, 7)) // The split is a bit shorter here. I don't feel like properly parsing the colon.
        .map(|i| exponent(count_winners(&i)))
        .collect::<Vec<i32>>()
        .iter()
        .sum();

    // Validate we correctly answered the example
    assert_eq!(
        sol, 13,
        "Example result for part 1 should be 13, found {:?}",
        sol
    );

    let part_one_solution = get_input("inputs/04.txt").into_iter();
    let sol: i32 = part_one_solution
        .map(|i| generate_vectors(&i, 10)) // The split is a bit longer here. I don't feel like properly parsing the colon.
        .map(|i| exponent(count_winners(&i)))
        .collect::<Vec<i32>>()
        .iter()
        .sum();

    println!("Solution for part 1: {:?}", sol);

    let part_two_example = get_input("examples/04.txt").into_iter();
    let vectors = part_two_example.map(|i| generate_vectors(&i, 7));
    let wins = vectors.clone().map(|i| count_winners(&i));

    let total_ex_cards = breed_cards(wins);
    // Validate we correctly answered the example
    assert_eq!(
        total_ex_cards, 30,
        "Example result for part 2 should be 30, found {:?}",
        sol
    );

    let part_two_example = get_input("inputs/04.txt").into_iter();
    let vectors = part_two_example.map(|i| generate_vectors(&i, 10));
    let wins = vectors.clone().map(|i| count_winners(&i));

    let total_cards = breed_cards(wins);
    println!("Solution for part 2: {:?}", total_cards);
}
