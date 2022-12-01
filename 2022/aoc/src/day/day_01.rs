use crate::util::{vec_of_strings};

use super::Day;
use itertools::{max, Itertools};

static DATE: u8 = 1;
pub struct Day01;

impl Day<u32> for Day01 {
    fn get_date(&self) -> u8 {
        DATE
    }

    fn solve_part_1(&self, input: &str) -> u32 {
        let lines = vec_of_strings(input);
        return max(calculate_calories_sums(lines)).unwrap();
    }

    fn solve_part_2(&self, input: &str) -> u32 {
        let lines = vec_of_strings(input);
        let sums = calculate_calories_sums(lines);

        return sums.iter().sorted().rev().take(3).sum();
    }
}

fn calculate_calories_sums(lines: Vec<&str>) -> Vec<u32> {
    let mut current = 0;
    let mut sums: Vec<u32> = vec![];
    for line in lines {
        if line.is_empty() {
            sums.push(current);
            current = 0;
        } else {
            current += line.parse::<u32>().unwrap();
        }
    }
    sums.push(current);
    sums
}