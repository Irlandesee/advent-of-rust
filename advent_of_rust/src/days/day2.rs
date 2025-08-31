use std::fs;
use std::path::PathBuf;
pub fn solve_day2(input_file: PathBuf) -> (i32, i32){
    let contents = fs::read_to_string(input_file)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| line.trim()
            .split(' ')
            .map(|value| value.parse::<i32>().unwrap())
            .collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>();

    //Part one
    //A vector to be "SAFE" must be in ascending order or descending order
    //any two adjacent levels differ by at least one and at most three
    let mut safe_count = 0;
    contents.iter()
        .for_each(|vec| {
            if is_level_valid(vec) {
                println!("{:?} is safe ", vec);
                safe_count += 1;
            } else {
                println!("{:?} is unsafe ", vec);
            }

        });


    //Part two
    //Now a report is considered safe when removing a sigle bad level
    //would make the report safe
    let mut safe_count_part_two = 0;
    contents.iter()
        .for_each(|vec| {
            if is_level_valid(vec) {
                println!("{:?} is safe", vec);
                safe_count_part_two += 1;
            } else {
                println!("{:?} is unsafe, checking if it could be safe...", vec);
                if check_if_vec_can_be_safe(vec) {
                    println!("{:?} can be made safe", vec);
                    safe_count_part_two += 1;
                } else {
                    println!("{:?} cannot be made safe", vec);
                }
            }
        });

    (safe_count, safe_count_part_two)
}

/**
Checks if a vector is an ascending or descending order
**/
fn check_ascending_or_descending(vec: &Vec<i32>) -> String {

    let mut vec_copy = vec.clone();
    vec_copy.sort();
    let is_ascending = vec.iter()
        .enumerate()
        .all(|(index, value)| vec_copy[index] == *value);
    if is_ascending { return "safe".to_string(); }

    vec_copy.reverse();
    let is_descending = vec.iter()
        .enumerate()
        .all(|(index, value)| vec_copy[index] == *value);
    if is_descending { return "safe".to_string(); }

    "unsafe".to_string()
}

/**
Check if the delta is between 0 and 3 and that two adjacent elements are not equal
**/
fn check_delta(vec: &Vec<i32>) -> bool {
    vec.iter()
        .enumerate()
        .all(|(index, value)| {
            index + 1 >= vec.len() || (value - vec[index + 1]).abs() <= 3 && vec[index + 1 ] != *value
        })
}

fn is_level_valid(vec: &Vec<i32>) -> bool {
    let ordered = check_ascending_or_descending(vec);
    let delta_ok = check_delta(vec);
    ordered == "safe" && delta_ok == true
}

/**
Finds the first "BAD" level of the vector and returns its index
**/
fn vec_find_bad_level(vec: &Vec<i32>) -> usize {
    for (index, value) in vec.iter().enumerate() {
        if index < vec.len() {
            
        }

    }

    0
}

fn check_if_vec_can_be_safe(vec: &Vec<i32>) -> bool {
    let mut temp_vec = vec.clone();


    false
}