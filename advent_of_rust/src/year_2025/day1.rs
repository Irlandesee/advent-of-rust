use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;
use regex::Regex;
use lazy_static::lazy_static;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Op{
    left: i32,
    right: i32,
    op: char
}

impl Op{
    fn new(left: i32, right: i32, op: char) -> Op {
        Op{left, right, op }
    }
}

pub fn solve(input_file: PathBuf) -> (i32, i32) {
    let contents = fs::read_to_string(input_file)
        .expect("Something went wrong reading the file");
    let strings = contents
        .lines()
        .map(|line| line.trim())
        .collect::<Vec<&str>>();

    (solve_part_one(strings.clone()), solve_part_two(strings.clone()))
}


fn solve_part_one(strings: Vec<&str>) -> i32 {
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
    password
}



fn solve_part_two(strings: Vec<&str>) -> i32 {
    //let mut map = HashMap::new(); //Use to cache the results
    let mut total_zero_count = 0;
    let mut current_dial = 50;
    for line in strings.iter(){
        match parse_line(line) {
            (Some(dir), Some(num)) => {
                let (new_pos, hits) = step_recursive(current_dial, dir, num);
                total_zero_count += hits;
                current_dial = new_pos;

            }
            _ => println!("Invalid line: {}", line)
        }

    }

    total_zero_count
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
    let new_dial: i32;
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


fn step_recursive(start: i32, direction: &str, steps: i32) -> (i32, i32) {
    // Base case: no steps left
    if steps == 0 {
        return (start, 0);
    }

    // Move one click
    let mut next = match direction {
        "L" => start - 1,
        "R" => start + 1,
        _ => panic!("Invalid direction"),
    };

    // Wrap into 0..=99
    next = ((next % 100) + 100) % 100;

    // Count if this click lands on 0
    let hit_zero = if next == 0 { 1 } else { 0 };

    // Recurse for the remaining steps
    let (final_pos, zero_hits) = step_recursive(next, direction, steps - 1);

    // Return combined result
    (final_pos, hit_zero + zero_hits)
}