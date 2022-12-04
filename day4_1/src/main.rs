use std::fs::File;
use std::io::{BufRead, Lines, BufReader, Result};
use std::path::Path;
use regex::Regex;

#[derive(Debug)]
struct LineData {
    first_min: i32,
    first_max: i32,
    second_min: i32,
    second_max: i32,
}

fn main() {
    let mut score: u32 = 0;

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("src/input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(val) = line {
                let line_data = line_to_data(&val);
                let first_contains_second = line_data.first_min <= line_data.second_min && line_data.first_max >= line_data.second_max;
                let second_contains_first = line_data.second_min <= line_data.first_min && line_data.second_max >= line_data.first_max;
                if first_contains_second || second_contains_first {
                    score += 1;
                }
            }
        }
    }

    println!("Got score {}", score);
}

fn line_to_data(line: &String) -> LineData {
    let re = Regex::new(r"^(\d*)-(\d*),(\d*)-(\d*)$").unwrap();
    let cap = re.captures(line).unwrap();
    return LineData {
        first_min: cap[1].parse::<i32>().unwrap(),
        first_max: cap[2].parse::<i32>().unwrap(),
        second_min: cap[3].parse::<i32>().unwrap(),
        second_max: cap[4].parse::<i32>().unwrap(),
    };
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
