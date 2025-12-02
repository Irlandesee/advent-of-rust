use clap::{Arg, Command};
use std::path::PathBuf;

pub fn parse_args() -> (String, String, PathBuf) {
    let matches = Command::new("Advent of Rust")
        .arg(Arg::new("file")
            .required(true)
            .short('f')
            .long("file")
            .help("File containing input to parse")
        )
        .arg(Arg::new("year").
            required(true).
            short('y')
            .long("year")
            .help("Which year to solve (year_2025, year_2025"))
        .arg(Arg::new("day")
            .required(true)
            .short('d')
            .long("day")
            .help("Which day to solve (1, 2 etc..)")
        )
        .get_matches();
    let year: String = matches.get_one::<String>("year").
        expect("Year is required").
        clone();
    let day: String = matches.get_one::<String>("day")
        .expect("Day is required")
        .clone();
    let file: PathBuf = PathBuf::from(matches
        .get_one::<String>("file")
        .expect("Missing file parameter"));

    (year, day, file)
}