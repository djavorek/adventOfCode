use crate::util::{vec_of_strings};
use super::Day;

static DATE: u8 = 5;

const CRATE_DEF_START: &str = "[";
const MOVE_INSTRUCTION: &str = "move";

pub struct Day05;

impl Day<String> for Day05 {
    fn get_date(&self) -> u8 {
        DATE
    }

    fn solve_part_1(&self, input: &str) -> String {
        let input_lines = vec_of_strings(input);
        let mut stacks = parse_crate_stacks(&input_lines);
        
        input_lines.iter()
            .skip_while(|line| !line.contains(MOVE_INSTRUCTION))
            .map(|instruction_line| Instuction::from_line(instruction_line))
            .for_each(|instuction|follow_with_cratemover_9000(instuction, &mut stacks));

        let mut result = String::with_capacity(stacks.len());
        stacks.iter().for_each(|stack| {
            result.push(*stack.last().unwrap());
        });
        
        result
    }

    fn solve_part_2(&self, input: &str) -> String {
        let input_lines = vec_of_strings(input);
        let mut stacks = parse_crate_stacks(&input_lines);
        
        input_lines.iter()
            .skip_while(|line| !line.contains(MOVE_INSTRUCTION))
            .map(|instruction_line| Instuction::from_line(instruction_line))
            .for_each(|instuction|follow_with_cratemover_9001(instuction, &mut stacks));

        let mut result = String::with_capacity(stacks.len());
        stacks.iter().for_each(|stack| {
            result.push(*stack.last().unwrap());
        });
        
        result
    }

    fn get_expected_result_1(&self) -> String {
        "ZRLJGSCTR".to_string()
    }

    fn get_expected_result_2(&self) -> String {
        "PRTTGRFPB".to_string()
    }
}

fn follow_with_cratemover_9000(instruction: Instuction, stacks: &mut Vec<Vec<char>>) -> () {
    (0..instruction.count).for_each(|_| {
        let goodie = stacks[instruction.from-1].pop().expect("Something did end up in the ocean");
        stacks[instruction.to-1].push(goodie)
    });
}

fn follow_with_cratemover_9001(instruction: Instuction, stacks: &mut Vec<Vec<char>>) -> () {
    let mut tmp_stack: Vec<char> = Vec::new();

    (0..instruction.count).for_each(|_| {
        let goodie = stacks[instruction.from-1].pop().expect("Something did end up in the ocean");
        tmp_stack.push(goodie)        
    });

    (0..instruction.count).for_each(|_| {
        stacks[instruction.to-1].push(tmp_stack.pop().unwrap())
    });
}

fn parse_crate_stacks(input_lines: &Vec<&str>) -> Vec<Vec<char>> {
    let crates_input: Vec<&str> = input_lines.iter()
        .take_while(|line| line.contains(CRATE_DEF_START))
        .cloned()
        .collect();
    let mut stacks: Vec<Vec<char>> = Vec::new();
    crates_input.iter().rev().for_each(|row_line| {
        let mut stack_index = 0;
        row_line.chars().skip(1).step_by(4).for_each(|crate_id| {
            if stacks.len() < (stack_index + 1) {
                stacks.push(Vec::new());
            }

            if !crate_id.is_whitespace() {
                stacks[stack_index].push(crate_id)
            }
            stack_index += 1;
        });
    });
    stacks
}

struct Instuction {
    count: usize,
    from: usize,
    to: usize
}

impl Instuction {
    fn from_line(line: &str) -> Self {
        let instruction_parts: Vec<&str> = line.split_whitespace().collect();
        let count = instruction_parts.get(1).unwrap().parse::<usize>().expect("Well, that are many creates");
        let from = instruction_parts.get(3).unwrap().parse::<usize>().expect("I do not know that stack");
        let to = instruction_parts.get(5).unwrap().parse::<usize>().expect("I do not know that stack");
        
        Self {count, from, to}
    }
}
