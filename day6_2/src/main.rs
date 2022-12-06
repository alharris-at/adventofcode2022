use std::fs::read_to_string;
use std::collections::HashSet;

const SIGNAL_LENGTH: usize = 4;
const MESSAGE_LENGTH: usize = 14;

fn main() {
    println!("Signal start at {}", find_signal_start(read_to_string("src/input.txt").unwrap(), SIGNAL_LENGTH));
    println!("Message start at {}", find_signal_start(read_to_string("src/input.txt").unwrap(), MESSAGE_LENGTH));
}

fn find_signal_start(input: String, header_length: usize) -> usize {
    let mut input_chars: Vec<char> = input.chars().collect();

    let mut buffer_index = 0;
    let mut signal_buffer: Vec<char> = vec!['a'; header_length];

    // Load the first 4 values into the signal_buffer, and pop them from the input_chars vector.
    // Then we can just iterate until we find the right set of 4.
    signal_buffer.copy_from_slice(&input_chars[..header_length]);
    input_chars = input_chars[header_length..].to_vec();

    for input_char in input_chars {
        if are_all_unique(&signal_buffer) {
            return buffer_index + header_length;
        }
        signal_buffer[buffer_index % header_length] = input_char;
        buffer_index += 1;
    }

    panic!("Did not find header");
}

fn are_all_unique(elements: &Vec<char>) -> bool {
    let unique_elements: HashSet<char> = HashSet::from_iter(elements.iter().copied());
    return elements.len() == unique_elements.len();
}
