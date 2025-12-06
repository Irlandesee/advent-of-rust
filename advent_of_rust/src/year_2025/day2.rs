use std::{fmt, fs};
use std::path::PathBuf;
use regex::Regex;
use lazy_static::lazy_static;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Range{
    start: i64,
    end: i64,
}

impl Range{
    fn new(start: i64, end: i64) -> Range{
        Range{start, end}
    }
}
impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
    }
}
pub fn solve(input_file: PathBuf) -> (i64, i64) {
    let content = fs::read_to_string(input_file)
        .expect("something went wrong reading the file");
    let ranges = extract_ranges(&content);
    ranges.iter().for_each(|range| {
        println!("{}", range)
    });

    (-1, -1)
}

lazy_static! { static ref RANGE_RE: Regex = Regex::new(r"(?P<start>\d+)-(?P<end>\d+)").unwrap(); }

fn extract_ranges(content: &str) -> Vec<Range> {
    let mut result: Vec<Range> = Vec::new();
    RANGE_RE.captures_iter(content).for_each(|caps|{
        let start = caps.name("start").unwrap().as_str().parse::<i64>().unwrap();
        let end = caps.name("end").unwrap().as_str().parse::<i64>().unwrap();
        let range = Range::new(start, end);
        result.push(range);
    });
    result
}