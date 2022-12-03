use std::fs::{File};
use std::io::{BufRead, Lines, BufReader, Result};
use std::path::Path;

fn main() {
    let mut curr_val = 0;
    let mut elf_vals: Vec<i32> = Vec::new();

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("src/input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(val) = line {
                if val.is_empty() {
                    elf_vals.push(curr_val);
                    curr_val = 0;
                    continue;
                }
                curr_val += val.parse::<i32>().unwrap();
            }
        }
    }

    elf_vals.sort();
    elf_vals.reverse();

    println!("Elf Vals: {}, {}, {} - Sum: {}", elf_vals[0], elf_vals[1], elf_vals[2], elf_vals[0] + elf_vals[1] + elf_vals[2]);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
