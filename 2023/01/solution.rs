// Trebuchet?!

// Something is wrong with global snow production, and you've been selected to take a look. The Elves have even given you a 
// map; on it, they've used stars to mark the top fifty locations that are likely to be having problems. 
// You've been doing this long enough to know that to restore snow operations, you need to check all fifty stars by 
// December 25th. 
// Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second 
// puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck! 
// You try to ask why they can't just use a weather machine ("not powerful enough") and where they're even sending you 
// ("the sky") and why your map looks mostly blank ("you sure ask a lot of questions") and hang on did you just say the sky 
// ("of course, where do you think snow comes from") when you realize that the Elves are already loading you into a 
// trebuchet ("please hold still, we need to strap you in"). 
// As they're making the final adjustments, they discover that their calibration document (your puzzle input) has been 
// amended by a very young Elf who was apparently just excited to show off her art skills. Consequently, the Elves are 
// having trouble reading the values on the document. 
// The newly-improved calibration document consists of lines of text; each line originally contained a specific calibration 
// value that the Elves now need to recover. On each line, the calibration value can be found by combining the first digit 
// and the last digit (in that order) to form a single two-digit number. 
// For example:
// 1abc2
// pqr3stu8vwx
// a1b2c3d4e5f
// treb7uchet

// In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.
// Consider your entire calibration document. What is the sum of all of the calibration values?

use std::fs;

// Part 1 notes
// Iterate through all rows
// Grab all characters which can be cast as i32 and add to a list.
// We now have a vector of all numeric values from each row -- cast them as string
// Get the 0 and -1 index from each vector and append them as strings
// Convert the string to i32
// Sum the vector.

// Write helper function which consumes string and gets all ints from that string

// --- Part Two ---

// Your calculation isn't quite right. It looks like some of the digits are actually spelled out with letters: one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".

// Equipped with this new information, you now need to find the real first and last digit on each line. For example:

// two1nine
// eightwothree
// abcone2threexyz
// xtwone3four
// 4nineeightseven2
// zoneight234
// 7pqrstsixteen

// In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding these together produces 281.

// What is the sum of all of the calibration values?

fn convert_to_ints(s: &str) -> String {
    let strs = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let mut digits = String::new();
    let cur = s.to_string();

    let mut i = 0;
    while i < cur.len() {
        for idx in (0..strs.len()).step_by(1) {
            if cur[i..cur.len()].starts_with(strs[idx]) {
                digits.push_str(&idx.to_string());
                i += strs[idx].len()-2;
                break;
            }
        };
        if cur.chars().nth(i).unwrap().is_digit(10) {
            digits.push_str(&cur.chars().nth(i).unwrap().to_string());
        };

        i+=1;
    }
    
    digits
}

fn get_ends(s: &str) -> i32 {
    // Parse all numeric values from string
    let nums: Vec<String> = s.chars()
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

fn get_input(file_path: &str) -> Vec<String> {
    let contents: Vec<String> = fs::read_to_string(file_path)
        .expect("Should have been able to read the file")
        .split("\n")
        .map(|s| s.to_string()) // Convert each &str to a String
        .collect();
    contents
}

fn main() {
    let part_one_example = get_input("example.txt").into_iter();
    // Sum up our values using the `get_ends` helper
    let example_value = part_one_example
        .fold(0, |sum, l| sum + get_ends(&l));
    // Validate we correctly answered the example
    assert_eq!(example_value, 142, "Example result for part 1 should be 142, found {:?}", example_value);
    
    let part_one_input= get_input("input.txt").into_iter();
    // Sum up our values using the `get_ends` helper
    let calibration_value = part_one_input
        .fold(0, |sum, l| sum + get_ends(&l));
    
    // Complete solution
    println!("Solution for part 1: {:?}", calibration_value);

    // Start part two
    let part_two_example = get_input("example_two.txt").into_iter();
    let calibration_two_value = part_two_example
        .fold(0, |sum, l| sum + get_ends(&convert_to_ints(&l)));
    
    assert_eq!(calibration_two_value, 281, "Example result for part 1 should be 281, found {:?}", calibration_two_value);
    
    let part_two_input = get_input("input.txt").into_iter();
    let calibration_two = part_two_input
        .fold(0, |sum, l| sum + get_ends(&convert_to_ints(&l)));
    
    println!("Solution for part 2: {:?}", calibration_two);


}