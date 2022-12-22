use crate::util::vec_of_strings;

use super::Day;

pub struct Day08;
static DATE: u8 = 8;

impl Day<u32> for Day08 {
    fn get_date(&self) -> u8 {
        DATE
    }

    fn solve_part_1(&self, input: &str) -> u32 {
        let input_lines: Vec<&str> = vec_of_strings(input).iter().map(|line| line.trim()).collect();
        1
    }

    fn solve_part_2(&self, input: &str) -> u32 {
        let input_lines: Vec<&str> = vec_of_strings(input).iter().map(|line| line.trim()).collect();
        2
    }

    fn get_expected_result_1(&self) -> u32 {
        todo!();
    }

    fn get_expected_result_2(&self) -> u32 {
        todo!();
    }
}
