use std::fs;

pub fn get_input(file_path: &str) -> Vec<String> {
    let contents: Vec<String> = fs::read_to_string(file_path)
        .expect("Should have been able to read the file")
        .split("\n")
        .map(|s| s.to_string()) // Convert each &str to a String
        .collect();
    contents
}
