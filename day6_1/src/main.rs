use std::fs::read_to_string;
use std::collections::HashSet;

const SIGNAL_LENGTH: usize = 4;

fn main() {
    println!("Signal start at {}", find_signal_start(read_to_string("src/input.txt").unwrap()));
}

fn find_signal_start(input: String) -> usize {
    let mut input_chars: Vec<char> = input.chars().collect();

    let mut buffer_index = 0;
    let mut signal_buffer: Vec<char> = vec!['a'; SIGNAL_LENGTH];

    // Load the first 4 values into the signal_buffer, and pop them from the input_chars vector.
    // Then we can just iterate until we find the right set of 4.
    signal_buffer.copy_from_slice(&input_chars[..SIGNAL_LENGTH]);
    input_chars = input_chars[SIGNAL_LENGTH..].to_vec();

    for input_char in input_chars {
        if are_all_unique(&signal_buffer) {
            return buffer_index + SIGNAL_LENGTH;
        }
        signal_buffer[buffer_index % SIGNAL_LENGTH] = input_char;
        buffer_index += 1;
    }

    panic!("Did not find index start.");
}

fn are_all_unique(elements: &Vec<char>) -> bool {
    let unique_elements: HashSet<char> = HashSet::from_iter(elements.iter().copied());
    return elements.len() == unique_elements.len();
}
