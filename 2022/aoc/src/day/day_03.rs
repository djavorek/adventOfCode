use crate::util::{vec_of_strings};

use super::Day;

static DATE: u8 = 3;

const ASCII_LOWER_TO_PRIORITY: u8 = 96;
const ASCII_UPPER_TO_PRIORITY: u8 = 38;

pub struct Day03;

impl Day<u32> for Day03 {
    fn get_date(&self) -> u8 {
        DATE
    }

    fn solve_part_1(&self, input: &str) -> u32 {
        let input_lines = vec_of_strings(input);

        input_lines.iter()
            .map(|line| line.split_at(line.len() / 2))
            .map(|line_parts| find_intersect(line_parts))
            .map(|shared_char| get_priority(shared_char))
            .fold(0, |acc, sum| acc + sum as u32)
    }

    fn solve_part_2(&self, input: &str) -> u32 {
        let input_lines = vec_of_strings(input);
        let mut priority_sum: u32 = 0;

        for group in input_lines.chunks(3) {
            let badge_identifier = group[0].chars()
                .filter(move |c| group[1].contains(*c))
                .filter(move |c| group[2].contains(*c))
                .next()
                .unwrap();
            priority_sum += get_priority(badge_identifier) as u32;
        }
        priority_sum
    }

    fn get_expected_result_1(&self) -> u32 {
        7848
    }

    fn get_expected_result_2(&self) -> u32 {
        2616
    }
}

fn find_intersect(set: (&str, &str)) -> char {
    let (first, second) = set;

    return first.chars()
        .filter(move |c| second.contains(*c))
        .next()
        .unwrap();
}

fn get_priority(character: char) -> u8 {
    let character_code = character as u8;

    match character {
        'a'..='z' => {
            character_code - ASCII_LOWER_TO_PRIORITY
        }
        'A'..='Z' => {
            character_code - ASCII_UPPER_TO_PRIORITY
        }
        _ => {
            0
        }
    }
}
