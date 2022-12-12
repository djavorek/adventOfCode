use super::Day;

use std::collections::HashSet;

const SIZE_OF_4: usize = 4;
const SIZE_OF_14: usize = 14;

pub struct Day06;
static DATE: u8 = 6;

impl Day<u32> for Day06 {
    fn get_date(&self) -> u8 {
        DATE
    }

    fn solve_part_1(&self, input: &str) -> u32 {
        let mut char_window: [char; SIZE_OF_4] = [' '; SIZE_OF_4];
        let mut char_index: usize = 0;

        for char in input.chars() {
            let window_index = char_index % SIZE_OF_4;
            char_window[window_index] = char;
            
            if char_index > SIZE_OF_4 - 1 && check_4_distinct(char_window) {
                return (char_index + 1).try_into().unwrap();
            }
            char_index += 1;
        }

        return 0;
    }

    fn solve_part_2(&self, input: &str) -> u32 {
        let mut char_window: [char; SIZE_OF_14] = [' '; SIZE_OF_14];
        let mut char_index: usize = 0;

        for char in input.chars() {
            let window_index = char_index % SIZE_OF_14;
            char_window[window_index] = char;
            
            if char_index > SIZE_OF_14 - 1 && check_14_distinct(char_window) {
                return (char_index + 1).try_into().unwrap();
            }
            char_index += 1;
        }

        return 0;
    }

    fn get_expected_result_1(&self) -> u32 {
        1287
    }

    fn get_expected_result_2(&self) -> u32 {
        3716
    }
}

fn check_4_distinct(char_array: [char; SIZE_OF_4]) -> bool {
    let mut char_set = HashSet::new();

    for i in 0..SIZE_OF_4 {
        char_set.insert(char_array[i]);
    }

    char_set.len() == SIZE_OF_4
}

fn check_14_distinct(char_array: [char; SIZE_OF_14]) -> bool {
    let mut char_set = HashSet::new();

    for i in 0..SIZE_OF_14 {
        char_set.insert(char_array[i]);
    }

    char_set.len() == SIZE_OF_14
}
