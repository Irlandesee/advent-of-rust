use std::fs;
use std::path::PathBuf;
use regex::Regex;
use lazy_static::lazy_static;
pub fn solve_day3(input_file: PathBuf){
    let contents = fs::read_to_string(input_file)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>();

    let total_res: i32 = contents.iter()
        .map(|line| {
            let muls = extract_mul(line);
            let line_res: i32 = muls.iter()
                .map(|token| {
                    let (x, y) = extract_numbers(token);
                    x * y
                })
                .sum();
            line_res
        })
        .sum();
    println!("Total res: {}", total_res);
}

fn extract_mul(line: &String) -> Vec<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new("mul\\(\\d+,\\d+\\)").unwrap();
    }
    RE.find_iter(line)
        .map(|cap| cap.as_str().to_string())
        .collect()
}

fn extract_numbers(token: &String) -> (i32, i32) {
    lazy_static! {
        static ref RE: Regex = Regex::new("([0-9]+,[0-9]+)").unwrap();
    }
    let (x, y) = RE.find_iter(token)
        .filter_map(|cap| {
            let numbers: Vec<i32> = cap.as_str()
                .split(",")
                .filter_map(|num| num.trim().parse().ok())
                .collect();
            match numbers.len() {
                2 => Some((numbers[0], numbers[1])),
                _ => None
            }
        })
        .next()
        .expect("No valid number pair found");
    (x, y)
}