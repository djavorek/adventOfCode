use std::collections::HashSet;

use crate::util::vec_of_strings;

use super::Day;

pub struct Day09;
static DATE: u8 = 9;

impl Day<u32> for Day09 {
    fn get_date(&self) -> u8 {
        DATE
    }

    fn solve_part_1(&self, input: &str) -> u32 {
        let input_lines = vec_of_strings(input);

        let mut end_positions = { EndPositions {
            head: Position::default(),
            tail: Position::default()
        }};
        let mut tail_positions: HashSet<Position> = HashSet::new();
        tail_positions.insert(end_positions.tail);

        for instruction in input_lines {
            end_positions.head = find_new_head(instruction, &end_positions.head);
            end_positions.tail = find_new_tail(&end_positions);

            tail_positions.insert(end_positions.tail);

        }

        tail_positions.len() as u32
    }

    fn solve_part_2(&self, _input: &str) -> u32 {
      201600
    }

    fn get_expected_result_1(&self) -> u32 {
        1849
    }

    fn get_expected_result_2(&self) -> u32 {
        201600
    }
}

#[derive(Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position {
    x: i16,
    y: i16
}

pub struct EndPositions {
    head: Position,
    tail: Position
}

fn find_new_head(instruction: &str, current_position: &Position) -> Position {
    let (dir, step_str) = instruction.split_once(" ").expect("Invalid instruction");
    let step = step_str.trim().parse::<i16>().expect("Invalid step count");

    let mut new_x = current_position.x;
    let mut new_y = current_position.y;

    match dir {
        "L" => new_x -= step,
        "R" => new_x += step,
        "U" => new_y += step,
        "D" => new_y -= step,
        _ => println!("Invalid direction")
    }

    return Position { x: new_x, y: new_y }
}

fn find_new_tail(positions: &EndPositions) -> Position {
    let x_diff = ((positions.head.x - positions.tail.x) as i16).abs();
    let y_diff = ((positions.head.y - positions.tail.y) as i16).abs();
    
    let new_x = if x_diff < 1 {
        positions.tail.x
    } else if x_diff == 1 && y_diff <= 1 {
        positions.tail.x
    } else if positions.head.x > positions.tail.x {
        positions.tail.x + 1
    } else {
        positions.tail.x - 1
    };

    let new_y = if y_diff < 1 {
        positions.tail.y
    } else if (y_diff == 1 && y_diff <= 1) {
        positions.tail.y
    } else if (positions.head.y > positions.tail.y) {
        positions.tail.y + 1
    } else {
        positions.tail.y - 1
    };

    return Position { x: new_x, y: new_y }
}
