use std::fs;
use std::path::PathBuf;
use regex::Regex;
use lazy_static::lazy_static;

pub fn solve(input_file: PathBuf) -> (i32, i32) {
    let contents = fs::read_to_string(input_file)
        .expect("Something went wrong reading the file");
    let strings = contents
        .lines()
        .map(|line| line.trim())
        .collect::<Vec<&str>>();


    let mut current_dial = 50;
    let mut password = 0;
    for line in strings.iter() {
        match parse_line(line){
            (Some(dir), Some(num)) => {
                current_dial = calc_move(dir, num, current_dial);
                //println!("{}:{} -> {}", dir, num, current_dial);
                if current_dial == 0 {
                    password += 1;
                }
            },
            _ => println!("Invalid line: {}", line)
        }
    }
    //println!("password: {}", password);

    (password, -1)
}

fn parse_line(line: &str) -> (Option<&str>, Option<i32>)  {
    (extract_direction(line), extract_num(line))
}
lazy_static! {static ref DIR_RE: Regex = Regex::new(r"(?P<direction>[A-Z])").unwrap();}
lazy_static! {static ref NUM_RE: Regex = Regex::new(r"(?P<num>\d+)").unwrap();}

fn extract_direction(input: &str) -> Option<&str> {
    DIR_RE.captures(input).and_then(|cap| {
        cap.name("direction").map(|direction| direction.as_str())
    })
}

fn extract_num(input: &str) -> Option<i32> {
    NUM_RE.captures(input)
        .and_then(|cap|{cap.name("num")})
        .map(|num| num.as_str().parse::<i32>().unwrap())
}

fn calc_move(direction: &str, num: i32, current_dial: i32) -> i32 {
    let mut new_dial: i32;
    match direction {
        "L" => {
            new_dial = current_dial - num;
        }
        "R" => {
            new_dial = current_dial + num;
        }
        _ => {
            panic!("Unknown direction: {}", direction);
        }
    }
    //Rust % operator keeps the sign
    ((new_dial % 100) + 100) % 100
}