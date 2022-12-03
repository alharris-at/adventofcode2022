use std::fs::{File};
use std::io::{BufRead, Lines, BufReader, Result};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let play_scores = HashMap::from([
        (String::from("A X"), 4),
        (String::from("A Y"), 8),
        (String::from("A Z"), 3),
        (String::from("B X"), 1),
        (String::from("B Y"), 5),
        (String::from("B Z"), 9),
        (String::from("C X"), 7),
        (String::from("C Y"), 2),
        (String::from("C Z"), 6),
    ]);

    let mut score = 0;

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("src/input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(val) = line {
                let curr_score = play_scores.get(&val).unwrap();
                score += curr_score;
            }
        }
    }

    println!("Score: {}", score);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
