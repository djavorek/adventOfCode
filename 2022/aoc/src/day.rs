use crate::util::read_lines;

use std::fmt::Display;

pub trait Day<T: Display> {
    fn get_date(&self) -> u8;
    fn solve_part_1(&self, input: &str) -> T;
    fn solve_part_2(&self, input: &str) -> T;

    fn get_inputs(&self) -> String {
        return read_lines(self.get_date());
    }

    fn solve(&self) -> (u8, T, T) {
        let date = self.get_date();

        let input = self.get_inputs();
        let result_1 = self.solve_part_1(&input);
        let result_2 = self.solve_part_2(&input);

        (date, result_1, result_2)
    }
}

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
