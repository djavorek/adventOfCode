use std::collections::HashSet;

use crate::util::vec_of_strings;

use super::Day;

pub struct Day09;
static DATE: u8 = 9;

const PRINT_LIMIT: i8 = 15;

impl Day<u32> for Day09 {
    fn get_date(&self) -> u8 {
        DATE
    }

    fn solve_part_1(&self, input: &str) -> u32 {
        // let mut print_value = 0;
        let input_lines = vec_of_strings(input);

        let mut end_positions = { EndPositions {
            head: Position::default(),
            tail: Position::default()
        }};
        let mut tail_positions: HashSet<Position> = HashSet::new();
        tail_positions.insert(end_positions.tail);

        // println!("- Start");
        // print_result(&end_positions);

        for instruction in input_lines {
            let (dir, count) = parse_instruction(instruction);
            // if print_value < PRINT_LIMIT {
            //     println!("\n Instruction -- {}", instruction);
            // }

            for _step in 0..count {
                end_positions.head = find_new_head(dir, &end_positions.head);
                end_positions.tail = find_new_tail(&end_positions);

                tail_positions.insert(end_positions.tail);

                // if print_value < PRINT_LIMIT {
                //     print_result(&end_positions);
                //     print_value += 1;
                // }
            }
        }

        tail_positions.len() as u32
    }

    fn solve_part_2(&self, input: &str) -> u32 {
        let mut print_value = 0;
        let input_lines = vec_of_strings(input);

        let mut end_positions = { EndPositions {
            head: Position::default(),
            tail: Position::default()
        }};
        let mut tail_positions: HashSet<Position> = HashSet::new();
        tail_positions.insert(end_positions.tail);

        println!("- Start");
        print_result(&end_positions);

        for instruction in input_lines {
            let (dir, count) = parse_instruction(instruction);
            if print_value < PRINT_LIMIT {
                println!("\n Instruction -- {}", instruction);
            }

            for _step in 0..count {
                end_positions.head = find_new_head(dir, &end_positions.head);
                end_positions.tail = find_new_tail(&end_positions);

                tail_positions.insert(end_positions.tail);

                if print_value < PRINT_LIMIT {
                    print_result(&end_positions);
                    print_value += 1;
                }
            }
        }

        tail_positions.len() as u32
    }

    fn get_expected_result_1(&self) -> u32 {
        6256
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

fn parse_instruction(instruction: &str) -> (&str, i16) {
    let (dir, step_str) = instruction.split_once(" ").expect("Invalid instruction");
    let step = step_str.trim().parse::<i16>().expect("Invalid step count");

    return (dir, step);
}

fn find_new_head(dir: &str, current_position: &Position) -> Position {
    let mut new_x = current_position.x;
    let mut new_y = current_position.y;

    match dir {
        "L" => new_x -= 1,
        "R" => new_x += 1,
        "U" => new_y -= 1,
        "D" => new_y += 1,
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
    } else if y_diff == 1 && x_diff <= 1 {
        positions.tail.y
    } else if positions.head.y > positions.tail.y {
        positions.tail.y + 1
    } else {
        positions.tail.y - 1
    };

    return Position { x: new_x, y: new_y }
}

fn print_result(end_positions: &EndPositions) -> () {
    print!("\n");
    for y in 0..10 {
        for x in 0..10 {
            if end_positions.head.x + 5 == x && end_positions.head.y + 5 == y {
                print!("H");
                continue;
            }
            if end_positions.tail.x + 5 == x && end_positions.tail.y + 5 == y {
                print!("T");
                continue;
            }
            print!(".")
        }
        print!("\n");
    }
}