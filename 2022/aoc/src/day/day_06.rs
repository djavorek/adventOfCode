use crate::util::{vec_of_strings};
use super::Day;

static DATE: u8 = 6;


pub struct Day06;

impl Day<String> for Day06 {
    fn get_date(&self) -> u8 {
        DATE
    }

    fn solve_part_1(&self, input: &str) -> String {
        let input_lines: Vec<&str> = vec_of_strings(input).iter().map(|line| line.trim()).collect();

        todo!()
    }

    fn solve_part_2(&self, input: &str) -> String {
        let input_lines: Vec<&str> = vec_of_strings(input).iter().map(|line| line.trim()).collect();
        
        todo!()
    }

    fn get_expected_result_1(&self) -> String {
        todo!()
    }

    fn get_expected_result_2(&self) -> String {
        todo!()
    }
}
