use crate::util::vec_of_strings;

use super::Day;

pub struct Day07;
static DATE: u8 = 7;

impl Day<u32> for Day07 {
    fn get_date(&self) -> u8 {
        DATE
    }

    fn solve_part_1(&self, input: &str) -> u32 {
        let input_lines: Vec<&str> = vec_of_strings(input).iter().map(|line| line.trim()).collect();
        let mut directory_archive: Vec<Directory> = vec![];
        let mut directory_stack: Vec<Directory> = vec![];

        for line in input_lines {
            let mut split_line = line.split_whitespace();
            let first_part = split_line.next().unwrap();
            let is_command = first_part == "$";

            if is_command {
                let command = split_line.next().unwrap();
                match command {
                    "cd" => {
                        let argument = split_line.next().expect("No argument provided for cd, how is that?");
                        match argument {
                            ".." => {
                                pop_from_directory_stack(&mut directory_stack, &mut directory_archive);
                            }
                            _ => {
                                push_to_directory_stack(argument, &mut directory_stack);
                            }
                        }
                    }
                    "ls" => {
                        continue;
                    }
                    _ => {
                        panic!("Unknown command {}", command)
                    }
                }
            }
            else {
                match first_part {
                    "dir" => {
                        continue;
                    }
                    _ => {
                        let file_size: usize = first_part.parse().unwrap();
                        
                        for mut dir in directory_stack.iter_mut() {
                            dir.size += file_size;
                        }
                    }
                }
            }
        }

        while !directory_stack.is_empty() {
            pop_from_directory_stack(&mut directory_stack, &mut directory_archive)
        }

        directory_archive.iter()
            .filter(|dir| dir.size <= 100000)
            .fold(0, | acc, curr | acc + curr.size) as u32
    }

    fn solve_part_2(&self, input: &str) -> u32 {
        let input_lines: Vec<&str> = vec_of_strings(input).iter().map(|line| line.trim()).collect();
        2
    }

    fn get_expected_result_1(&self) -> u32 {
        1334506
    }

    fn get_expected_result_2(&self) -> u32 {
        todo!()
    }
}

fn pop_from_directory_stack(directory_stack: &mut Vec<Directory>, all_directory: &mut Vec<Directory>) {
    let popped = directory_stack.pop().unwrap();
    let from_directory_archive = all_directory.iter_mut()
        .find(|dir| dir.path == popped.path && dir.name == popped.name);
    match from_directory_archive {
        Some(directory) => {
            directory.size = directory.size + popped.size
        }
        None => {
            all_directory.push(popped);
        }
    }
}

fn push_to_directory_stack(argument: &str, directory_stack: &mut Vec<Directory>) {
    let mut path = String::from("/");
    if argument != "/" {
        let separator = if directory_stack.last().unwrap().name.ends_with("/") {
            ""
        } else {
            "/"
        };
        path = directory_stack.last().unwrap().path.to_owned() + separator + argument
    }
    let new_dir = Directory { name: argument.to_string(), path: path, size: 0};
    directory_stack.push(new_dir);
}

struct Directory {
    name: String,
    path: String,
    size: usize
}