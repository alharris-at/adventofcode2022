use std::fs::{File};
use std::io::{BufRead, Lines, BufReader, Result};
use std::path::Path;

fn main() {
    let mut max_val = 0;
    let mut curr_val = 0;

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("src/input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(val) = line {
                if val.is_empty() {
                    curr_val = 0;
                    continue;
                }
                // ADD TO VAL
                let as_int: i32 = val.parse().unwrap();
                curr_val += as_int;
                if curr_val > max_val {
                    max_val = curr_val;
                }
            }
        }
    }

    println!("Max Cals: {}", max_val);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
