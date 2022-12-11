use crate::util::{vec_of_strings};

use super::Day;

static DATE: u8 = 1;
pub struct Day01;

impl Day<u32> for Day01 {
    fn get_date(&self) -> u8 {
        DATE
    }

    fn solve_part_1(&self, input: &str) -> u32 {
        let input_lines = vec_of_strings(input).iter().map(|line| line.trim()).collect();
        let calories_by_elves = calculate_calories_by_elves(input_lines);
        return *calories_by_elves.iter().max().unwrap();
    }

    fn solve_part_2(&self, input: &str) -> u32 {
        let lines = vec_of_strings(input).iter().map(|line| line.trim()).collect();
        let mut calories_by_elves = calculate_calories_by_elves(lines);
        calories_by_elves.sort();

        return calories_by_elves.iter().rev().take(3).sum();
    }

    fn get_expected_result_1(&self) -> u32 {
        75622
    }

    fn get_expected_result_2(&self) -> u32 {
        213159
    }
}

fn calculate_calories_by_elves(lines: Vec<&str>) -> Vec<u32> {
    let mut current_calorie_sum = 0;
    let mut calories_by_elves: Vec<u32> = vec![];
    for line in lines {

        if line.is_empty() {
            calories_by_elves.push(current_calorie_sum);
            current_calorie_sum = 0;
        } else {
            current_calorie_sum += line.parse::<u32>().unwrap();
        }
    }
    calories_by_elves.push(current_calorie_sum);
    calories_by_elves
}
