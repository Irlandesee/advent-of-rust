mod year_2024;
mod year_2025;

use advent_of_rust::parse_args;
use year_2024::{day1 as day1_2024, day2, day3};
use year_2025::{day1 as day1_2025};

fn main() {
    let (year, day, file) = parse_args();
    match year.as_str(){
        "2024" => {
            match day.as_str() {
                "1" => {
                    let (part_one_res, part_two_res) = day1_2024::solve_day1(file);
                    println!("Part one: {:?}", part_one_res);
                    println!("Part two: {:?}", part_two_res);
                },
                "2" => {
                    let(part_one_res, part_two_res) = day2::solve_day2(file);
                    println!("Part one: {:?}", part_one_res);
                    println!("Part two: {:?}", part_two_res);
                }
                "3" => {
                    day3::solve_day3(file)

                }
                _ => println!("Unknown day")
            }

        }
        "2025" => {
            match day.as_str(){
                "1" => {
                    let (part_one_res, _) = day1_2025::solve(file);
                    println!("Part one: {:?}", part_one_res);

                }
                _ => println!("Unknown day")
            }
        }
        _ => println!("Unknown year")
    }

}
