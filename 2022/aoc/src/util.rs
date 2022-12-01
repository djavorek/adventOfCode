use std::env;
use std::fs;

pub fn read_lines<P>(folder: &str, day: u8) -> String {
    let cwd = env::current_dir().unwrap();

    let filepath = cwd.join("inputs").join(format!("{:02}.txt", day));

    let f = fs::read_to_string(filepath);
    f.expect("could not open input file")
}

pub fn vec_of_strings(input: &str) -> Vec<&str> {
    let lines: Vec<&str> = input.split('\n').map(|str| {
        str.trim()
    }).collect();

    lines
}

pub fn vec_of_numbers(input: &str) -> Vec<u32> {
    let strings = vec_of_strings(input);

    return strings.iter().map(|str| {
        str.parse().unwrap()
    }).collect();
}
