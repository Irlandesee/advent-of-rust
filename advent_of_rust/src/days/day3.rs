use std::fs;
use std::iter::Sum;
use std::path::PathBuf;
use regex::Regex;
use lazy_static::lazy_static;
pub fn solve_day3(input_file: PathBuf){
    let contents = fs::read_to_string(input_file)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>();

    //Part 1
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
    //Part 2
    //dont-do regex -> don\'t\(\).+do\(\)

    //Donts - blocks muls
    //dos - re-enables muls
    // for example here:
    //xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
    //mul(5,5) and mul(11,8) are disabled by the dont before them
    //mul(8,5) is re enabled by the do before it

    //Calculate the total muls
    let total_muls: Vec<String> = contents.iter()
        .flat_map(|line| extract_mul(line)).collect();
    println!("total muls len: {:?}", total_muls.len());
    //find all the blocks of dont-dos and extract all the muls from it
    let dont_do_muls = extract_dont_do_muls(contents);
    println!("dont_do_muls len: {:?}", dont_do_muls.len());
    //from the total muls, filter out the dont-dos-muls
    let filtered_muls = total_muls.iter()
        .filter(|mul| !dont_do_muls.contains(mul))
        .collect::<Vec<&String>>();
    println!("filterd muls len: {:?}", filtered_muls.len());

    //calculate the sum
    let part_two_res = calculate_sum_muls(filtered_muls);
    println!("part_two_res: {:?}", part_two_res);

}

fn calculate_sum_muls(vec: Vec<&String>) -> i32 {
    vec.iter()
        .map(|token| {
            let (x, y) = extract_numbers(token);
            x + y
        }).sum()
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
        static ref RE: Regex = Regex::new("(\\d+,\\d+)").unwrap();
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


fn extract_dont_do_muls(lines: Vec<String>) -> Vec<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new("don\'t\\(\\).+do\\(\\)").unwrap();
    }
    lines.iter().flat_map(|line| {
        //extract all muls in this dont-do block
        RE.find_iter(line)
            .flat_map(|cap| {
                let dont_do_block = cap.as_str().to_string();
                extract_mul(&dont_do_block)
            })
    }).collect()
}