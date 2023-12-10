mod utils;
use utils::get_input;

fn convert_to_ints(s: &str) -> String {
    let strs = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut digits = String::new();
    let cur = s.to_string();

    let mut i = 0;
    while i < cur.len() {
        for idx in (0..strs.len()).step_by(1) {
            if cur[i..cur.len()].starts_with(strs[idx]) {
                digits.push_str(&idx.to_string());
                i += strs[idx].len() - 2;
                break;
            }
        }
        if cur.chars().nth(i).unwrap().is_digit(10) {
            digits.push_str(&cur.chars().nth(i).unwrap().to_string());
        };

        i += 1;
    }

    digits
}

fn get_ends(s: &str) -> i32 {
    // Parse all numeric values from string
    let nums: Vec<String> = s
        .chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_string())
        .collect();

    // Sum the first and last values from the string
    let result: i32 = match nums.len() {
        0 => 0,
        _ => {
            let first = nums.first().unwrap().to_string();
            let last = nums.last().unwrap().to_string();
            let concatenated = first + &last;
            concatenated.parse::<i32>().unwrap()
        }
    };

    result
}

fn main() {
    let part_one_example = get_input("examples/01_0.txt").into_iter();
    // Sum up our values using the `get_ends` helper
    let example_value = part_one_example.fold(0, |sum, l| sum + get_ends(&l));
    // Validate we correctly answered the example
    assert_eq!(
        example_value, 142,
        "Example result for part 1 should be 142, found {:?}",
        example_value
    );

    let part_one_input = get_input("inputs/01.txt").into_iter();
    // Sum up our values using the `get_ends` helper
    let calibration_value = part_one_input.fold(0, |sum, l| sum + get_ends(&l));

    // Complete solution
    println!("Solution for part 1: {:?}", calibration_value);

    // Start part two
    let part_two_example = get_input("examples/01_1.txt").into_iter();
    let calibration_two_value =
        part_two_example.fold(0, |sum, l| sum + get_ends(&convert_to_ints(&l)));

    assert_eq!(
        calibration_two_value, 281,
        "Example result for part 1 should be 281, found {:?}",
        calibration_two_value
    );

    let part_two_input = get_input("inputs/01.txt").into_iter();
    let calibration_two = part_two_input.fold(0, |sum, l| sum + get_ends(&convert_to_ints(&l)));

    println!("Solution for part 2: {:?}", calibration_two);
}
