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

    let _ = contents.iter()
        .map(|line| {
        println!("Line: {}", line);
        let muls = extract_mul(line);
        println!("{:?}", muls)
    });

}

fn extract_mul(line: &String) -> Vec<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new("mul\\(\\d+,\\d+\\)").unwrap();
    }
    RE.find_iter(line)
        .map(|cap| cap.as_str().to_string())
        .collect()
}