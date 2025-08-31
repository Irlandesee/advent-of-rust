mod days;
use advent_of_rust::parse_args;
use days::{day1, day2};

fn main() {
    let (day, file) = parse_args();

    match day.as_str() {
        "1" => {
            let (part_one_res, part_two_res) = day1::solve_day1(file);
            println!("Part one: {:?}", part_one_res);
            println!("Part two: {:?}", part_two_res);
        },
        "2" => println!("Solving day2: {}", day),
        _ => println!("Unknown day")
    }


}
