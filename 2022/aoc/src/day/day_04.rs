use crate::util::{vec_of_strings};
use super::Day;

static DATE: u8 = 4;

pub struct Day04;

impl Day<u32> for Day04 {
    fn get_date(&self) -> u8 {
        DATE
    }

    fn solve_part_1(&self, input: &str) -> u32 {
        let input_lines = vec_of_strings(input).iter().map(|line| line.trim()).collect();
        let overlap_count = get_overlap_count(input_lines, &check_complete_overlap);
        overlap_count.try_into().unwrap()
    }

    fn solve_part_2(&self, input: &str) -> u32 {
        let input_lines = vec_of_strings(input).iter().map(|line| line.trim()).collect();
        let overlap_count = get_overlap_count(input_lines, &check_overlap);
        overlap_count.try_into().unwrap()
    }

    fn get_expected_result_1(&self) -> u32 {
        550
    }

    fn get_expected_result_2(&self) -> u32 {
        931
    }
}

fn get_overlap_count(input_lines: Vec<&str>, is_overlap: &dyn Fn((u32, u32), (u32, u32)) -> bool) -> usize {
    let overlap_count = input_lines.iter()
        .map(|line| line.split_once(",").expect("What should the other Elf do?"))
        .map(|parts| {
            let range_1 = parse_range(parts.0);
            let range_2 = parse_range(parts.1);

            (range_1, range_2)
        })
        .filter(|ranges| is_overlap(ranges.0, ranges.1))
        .count();
    overlap_count
}

fn parse_range(range_str: &str) -> (u32, u32) {
    let parts = range_str.split_once("-").expect("I am pretty sure you were talking about ranges");

    (parts.0.parse().unwrap(), parts.1.parse().unwrap())       
}

fn check_overlap(range_1: (u32, u32), range_2: (u32, u32)) -> bool {
    if range_1.0 <= range_2.1 && range_1.1 >= range_2.0 {
        return true;
    }
    return false;
}

fn check_complete_overlap(range_1: (u32, u32), range_2: (u32, u32)) -> bool {
    if (range_1.0 <= range_2.0 && range_1.1 >= range_2.1) ||
            (range_2.0 <= range_1.0 && range_2.1 >= range_1.1) {
        return true;
    }

    return false;
}