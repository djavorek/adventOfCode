use std::collections::BinaryHeap;
use crate::util::vec_of_strings;

use super::Day;

pub struct Day08;
static DATE: u8 = 8;

const DIMENSION: usize = 99;

impl Day<u32> for Day08 {
    fn get_date(&self) -> u8 {
        DATE
    }

    fn solve_part_1(&self, input: &str) -> u32 {
        let input_lines: Vec<&str> = vec_of_strings(input).iter().map(|line| line.trim()).collect();
        let mut trees = [[0u8; DIMENSION]; DIMENSION];
        let mut count_of_visible: u32 = (DIMENSION * 2 + (DIMENSION - 2) * 2).try_into().unwrap(); // Trees on the edges

        for (row_i, line) in input_lines.iter().enumerate() {
            for (column_i, tree) in line.chars().enumerate() {
                trees[row_i][column_i] = tree.to_digit(10).unwrap_or(0) as u8;
            }
        }

        for row_i in 1..DIMENSION - 1 {
            for column_i in 1..DIMENSION - 1 {
                let item = trees[row_i][column_i];
                let row_left: BinaryHeap<&u8> = trees[row_i].iter()
                                                            .take(column_i)
                                                            .collect();
                let row_right: BinaryHeap<&u8> = trees[row_i].iter()
                                                            .skip(column_i + 1)
                                                            .take(DIMENSION - 1 - column_i)
                                                            .collect();
                let column_top: BinaryHeap<&u8> = trees.iter()
                                                            .take(row_i)
                                                            .map(|row| row.get(column_i).unwrap())
                                                            .collect();
                let column_bottom: BinaryHeap<&u8> = trees.iter()
                                                                .skip(row_i + 1)
                                                                .take(DIMENSION - 1 - row_i)
                                                                .map(|row| row.get(column_i).unwrap())
                                                                .collect();

                let left =  row_left.peek().expect("No number on left");
                let right =  row_right.peek().expect("No number on right");
                let top =  column_top.peek().expect("No number on top");
                let bottom =  column_bottom.peek().expect("No number on bottom");
                
                let is_visible = (left < &&item) || (right < &&item) || (top < &&item) || (bottom < &&item);

                if is_visible {
                    count_of_visible += 1
                }
            }
        }

        count_of_visible
    }

    fn solve_part_2(&self, input: &str) -> u32 {
        let input_lines: Vec<&str> = vec_of_strings(input).iter().map(|line| line.trim()).collect();
        let mut trees = [[0u8; DIMENSION]; DIMENSION];
        let mut max_scenic_score: u32 = 0;

        for (row_i, line) in input_lines.iter().enumerate() {
            for (column_i, tree) in line.chars().enumerate() {
                trees[row_i][column_i] = tree.to_digit(10).unwrap_or(0) as u8;
            }
        }

        for row_i in 1..DIMENSION - 1 {
            for column_i in 1..DIMENSION - 1 {
                let item = trees[row_i][column_i];
                let row_left: Vec<&u8> = trees[row_i].iter()
                                                            .take(column_i)
                                                            .collect();
                let row_right: Vec<&u8> = trees[row_i].iter()
                                                            .skip(column_i + 1)
                                                            .take(DIMENSION - 1 - column_i)
                                                            .collect();
                let column_top: Vec<&u8> = trees.iter()
                                                            .take(row_i)
                                                            .map(|row| row.get(column_i).unwrap())
                                                            .collect();
                let column_bottom: Vec<&u8> = trees.iter()
                                                                .skip(row_i + 1)
                                                                .take(DIMENSION - 1 - row_i)
                                                                .map(|row| row.get(column_i).unwrap())
                                                                .collect();
                
                if row_left.is_empty() || row_right.is_empty() || column_top.is_empty() || column_bottom.is_empty() {
                    continue; // It would result 0 scenic value
                }

                let mut visible_left = row_left.iter().rev().take_while(is_not_blocking_view(item)).count() + 1;
                let mut visible_right = row_right.iter().take_while(is_not_blocking_view(item)).count() + 1;
                let mut visible_top = column_top.iter().rev().take_while(is_not_blocking_view(item)).count() + 1;
                let mut visible_bottom = column_bottom.iter().take_while(is_not_blocking_view(item)).count() + 1;

                if visible_left > row_left.len() {
                    visible_left -= 1;
                }
                if visible_right > row_right.len() {
                    visible_right -= 1;
                }
                if visible_top > column_top.len() {
                    visible_top -= 1;
                }
                if visible_bottom > column_bottom.len() {
                    visible_bottom -= 1;
                }

                let scenic_score: u32 = (visible_left * visible_right * visible_top * visible_bottom).try_into().unwrap();

                if scenic_score > max_scenic_score {
                    max_scenic_score = scenic_score;
                }
            }
        }


        max_scenic_score
    }

    fn get_expected_result_1(&self) -> u32 {
        1849
    }

    fn get_expected_result_2(&self) -> u32 {
        201600
    }
}

fn is_not_blocking_view(current_tree: u8) -> impl Fn(&&&u8) -> bool {
    move | test_subject: &&&u8 | current_tree > ***test_subject
}
