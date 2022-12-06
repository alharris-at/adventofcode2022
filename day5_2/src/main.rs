use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, Lines, BufReader, Result};
use std::path::Path;
use regex::Regex;

#[derive(Debug)]
struct Move {
    source_stack_index: usize,
    destination_stack_index: usize,
    move_count: u32,
}

fn main() {
    let moves = construct_moves();
    println!("Generated moves {:?}", moves);

    let mut initial_stacks = construct_stack_state();
    println!("Generated initial stack state {:?}", initial_stacks);

    let modified_stacks = process_stack_moves(&mut initial_stacks, moves);
    println!("Generated modified stack state {:?}", modified_stacks);

    let top_values = get_top_values(modified_stacks);
    println!("Top Values: {:?}", top_values);

    let response_string = String::from_iter(top_values);
    println!("Result {}", response_string);
}

fn get_top_values(stack_state: &mut Vec<VecDeque<char>>) -> Vec<char> {
    let mut top_values = Vec::new();

    for stack in stack_state {
        top_values.push(stack.pop_front().unwrap());
    }

    return top_values;
}

fn process_stack_moves(stack_state: &mut Vec<VecDeque<char>>, moves: Vec<Move>) -> &mut Vec<VecDeque<char>> {
    for move_config in moves {
        let mut temp_queue: VecDeque<char> = VecDeque::new();
        {
            let source_stack = stack_state.get_mut(move_config.source_stack_index - 1).unwrap();
            for _ in 0..move_config.move_count {
                let moved_char = source_stack.pop_front().unwrap();
                temp_queue.push_front(moved_char);
            }
        }
        {
            let destination_stack = stack_state.get_mut(move_config.destination_stack_index - 1).unwrap();
            for char_to_push in temp_queue {
                destination_stack.push_front(char_to_push)
            }
        }
    }
    return stack_state;
}

fn construct_moves() -> Vec<Move> {
    let mut moves = Vec::new();

    let re = Regex::new(r"^move (\d*) from (\d*) to (\d*)$").unwrap();

    if let Ok(lines) = read_lines("src/moves.txt") {
        for line in lines {
            if let Ok(val) = line {
                let cap = re.captures(&val).unwrap();
                moves.push(Move {
                    source_stack_index: cap[2].parse::<usize>().unwrap(),
                    destination_stack_index: cap[3].parse::<usize>().unwrap(),
                    move_count: cap[1].parse::<u32>().unwrap(),
                });
            }
        }
    }

    return moves;
}

fn construct_stack_state() -> Vec<VecDeque<char>> {
    let mut stacks: Option<Vec<VecDeque<char>>> = None;
    let mut stack_count: Option<usize> = None;

    if let Ok(lines) = read_lines("src/state.txt") {
        for line in lines {
            if let Ok(val) = line {
                if let None = stacks {
                    stack_count = Some((val.chars().count() + 1) / 4);
                    let mut local_stacks: Vec<VecDeque<char>> = Vec::new();
                    for _ in 0..stack_count.unwrap() {
                        local_stacks.push(VecDeque::new());
                    }
                    stacks = Some(local_stacks);
                }
                
                if let Some(stacks) = &mut stacks {
                    let row_chars: Vec<char> = val.chars().collect();
                    for stack_index in 0..stack_count.unwrap() {
                        let stack_index_char = row_chars.get(stack_index * 4 + 1).unwrap().clone();
                        if stack_index_char != ' ' {
                            stacks.get_mut(stack_index).unwrap().push_back(stack_index_char);
                        }
                    }
                }
            }
        }
    }

    return stacks.unwrap();
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
