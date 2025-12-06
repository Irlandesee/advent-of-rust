use std::fs;
use std::path::PathBuf;
use regex::Regex;
use lazy_static::lazy_static;

pub fn solve(input_file: PathBuf) -> (i32, i32) {
    let contents = fs::read_to_string(input_file).expect("something went wrong reading the file");
    let strings = contents
        .lines()
        .map(|line| line.trim())
        .collect::<Vec<&str>>();

    (-1, -1)
}


// fn parse_nums(input: str) -> Vec<u8> {
//     input.map(|line|{
//         line.trim()
//             .chars()
//             .filter_map(|c| c.to_digit(10))
//             .map(|d| d as u8)
//             .collect()
//     })
//         .collect()
// }
