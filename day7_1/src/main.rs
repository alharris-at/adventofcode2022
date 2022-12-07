use std::fs::read_to_string;
use core::str::Lines;

#[derive(Debug, Clone)]
struct File {
    name: String,
    size: usize
}

#[derive(Debug, Clone)]
struct Directory {
    name: String,
    files: Vec<File>,
    directories: Vec<Directory>,
}

const MAX_DIR_SIZE: u32 = 100000;

fn main() {
    let input_text = read_to_string("src/input.txt").unwrap();
    let root = build_structure_from_lines(input_text.lines());
    println!("Build fs {:?}", root);

    let structure = build_directory_structure();
    println!("Structure {:?}", structure);

    print_all_directory_sizes(structure.clone());
}

fn build_structure_from_lines(lines: Lines) -> Directory {
    for line in lines {
        println!("Line {}", line);
    }
    Directory { name: String::from(""), files: Vec::from([]), directories: Vec::from([]) }
}

fn print_all_directory_sizes(root: Directory) {
    let directory_size = get_directory_size(root.clone());
    println!("Directory {} - {}", root.name, directory_size);
    for directory in root.directories {
        print_all_directory_sizes(directory);
    }
}

fn get_directory_size(root: Directory) -> usize {
    let mut size: usize = 0;
    for file in root.files {
        size += file.size;
    }
    for directory in root.directories {
        size += get_directory_size(directory);
    }
    return size;
}

fn build_directory_structure() -> Directory {
    Directory {
        name: String::from("/"),
        files: Vec::from([
            File {
                name: String::from("input.txt"),
                size: 500,
            },
            File {
                name: String::from("output.txt"),
                size: 300,
            },
        ]),
        directories: Vec::from([
            Directory {
                name: String::from("target"),
                files: Vec::from([
                    File {
                        name: String::from("myFile.dat"),
                        size: 1000,
                    }
                ]),
                directories: Vec::from([
                    Directory {
                        name: String::from("debug"),
                        files: Vec::from([
                            File {
                                name: String::from("out.exe"),
                                size: 40000,
                            }
                        ]),
                        directories: Vec::from([]),
                    },
                    Directory {
                        name: String::from("release"),
                        files: Vec::from([
                            File {
                                name: String::from("out.exe"),
                                size: 40000,
                            }
                        ]),
                        directories: Vec::from([]),
                    }
                ]),
            }
        ]),
    }
}
