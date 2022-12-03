use std::fs::{File};
use std::io::{BufRead, Lines, BufReader, Result};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let play_scores = HashMap::from([
        (String::from("A X"), 3), // Loss, we play Scissors, Score is TK
        (String::from("A Y"), 4), // Draw, we play Rock, Score is TK
        (String::from("A Z"), 8), // Win, we play Paper, Score is TK
        (String::from("B X"), 1), // Loss, we play Rock, Score is TK
        (String::from("B Y"), 5), // Draw, we play Paper, Score is TK
        (String::from("B Z"), 9), // Win, we play Scissors, Score is TK
        (String::from("C X"), 2), // Loss, we play Paper, Score is TK
        (String::from("C Y"), 6), // Draw, we play Scissors, Score is TK
        (String::from("C Z"), 7), // Win, we play Rock, Score is TK
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
