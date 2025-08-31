mod days;
use clap::{Arg, Command};
use days::{day1, day2};

fn main() {
    let matches = Command::new("Advent Day 2")
        .about("Computes day 2 2024")
        .arg(Arg::new("file")
            .required(true)
            .short('f')
            .long("file")
            .help("File containing input")
        )
        .get_matches();
    let file_str :&String = matches.get_one("file").expect("Missing file parameter");


    day1::solve_day1(file_str);

}
