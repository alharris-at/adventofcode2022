use std::fs::File;
use std::io::{BufRead, Lines, BufReader, Result};
use std::path::Path;
use std::collections::{HashSet, HashMap};

fn main() {
    let char_priorities: HashMap<char, u32> = HashMap::from([
        ('a', 1),
        ('b', 2),
        ('c', 3),
        ('d', 4),
        ('e', 5),
        ('f', 6),
        ('g', 7),
        ('h', 8),
        ('i', 9),
        ('j', 10),
        ('k', 11),
        ('l', 12),
        ('m', 13),
        ('n', 14),
        ('o', 15),
        ('p', 16),
        ('q', 17),
        ('r', 18),
        ('s', 19),
        ('t', 20),
        ('u', 21),
        ('v', 22),
        ('w', 23),
        ('x', 24),
        ('y', 25),
        ('z', 26),
        ('A', 27),
        ('B', 28),
        ('C', 29),
        ('D', 30),
        ('E', 31),
        ('F', 32),
        ('G', 33),
        ('H', 34),
        ('I', 35),
        ('J', 36),
        ('K', 37),
        ('L', 38),
        ('M', 39),
        ('N', 40),
        ('O', 41),
        ('P', 42),
        ('Q', 43),
        ('R', 44),
        ('S', 45),
        ('T', 46),
        ('U', 47),
        ('V', 48),
        ('W', 49),
        ('X', 50),
        ('Y', 51),
        ('Z', 52),
    ]);

    let mut score: u32 = 0;

    let mut chars_1: HashSet<char> = HashSet::new();
    let mut chars_2: HashSet<char> = HashSet::new();
    let mut index = 0;


    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("src/input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(val) = line {
                match index {
                    i if i == 0 => {
                        chars_1 = HashSet::from_iter((&val).chars());
                    }
                    i if i == 1 => {
                        let char_set: HashSet<char> = HashSet::from_iter((&val).chars());
                        let intersection = chars_1.intersection(&char_set);
                        chars_2 = HashSet::from_iter(intersection.into_iter().copied());
                    }
                    i if i == 2 => {
                        let char_set: HashSet<char> = HashSet::from_iter((&val).chars());
                        let intersection = chars_2.intersection(&char_set);
                        let chars_3: HashSet<char> = HashSet::from_iter(intersection.into_iter().copied());
                        for mych in chars_3 {
                            score += char_priorities.get(&mych).unwrap();
                        }
                    }
                    _ => unreachable!("should not be here")
                }
                index = (index + 1) % 3;
            }
        }
    }

    println!("Got score {}", score);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
