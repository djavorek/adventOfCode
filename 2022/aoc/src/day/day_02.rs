use std::{collections::HashMap};

use crate::util::{vec_of_strings};

use super::Day;

static DATE: u8 = 2;
pub struct Day02;

impl Day<u32> for Day02 {
    fn get_date(&self) -> u8 {
        DATE
    }

    fn solve_part_1(&self, input: &str) -> u32 {
        let input_lines = vec_of_strings(input);
        let plays_by_rounds: Vec<(&str, &str)> = input_lines.iter()
            .map(|line| line.split_once(" ").unwrap())
            .collect();

        let score = plays_by_rounds.iter()
            .map(|touple| calculate_round_score(touple.0, touple.1))
            .fold(0, |acc, score| acc + score as u32);

        score
    }

    fn solve_part_2(&self, input: &str) -> u32 {
        2
    }
}

fn calculate_round_score(step: &str, own_step: &str) -> u8 {
    let char_map: HashMap<char, Step> = HashMap::from([
        ('A', Step::Rock),
        ('B', Step::Paper),
        ('C', Step::Scissors),
        ('X', Step::Rock),
        ('Y', Step::Paper),
        ('Z', Step::Scissors),
    ]);

    let step_char = step.chars().next().expect("Empty Elf-step found in input");
    let own_step_char = own_step.chars().next().expect("Empty Own-step found in input");
    
    let step_value = char_map.get(&step_char).expect("Unknown Elf-step found in input");
    let own_step_value = char_map.get(&own_step_char).expect("Unknown Own-step found in input");

    let round_result_score = get_round_result_score(*step_value, *own_step_value);
    

    round_result_score + *own_step_value as u8
}

fn get_round_result_score(step: Step, own_step: Step) -> u8 {
    if step == own_step {
        return 3;
    }

    if (own_step == Step::Rock && step == Step::Scissors) ||
       (own_step == Step::Paper && step == Step::Rock) || 
       (own_step == Step::Scissors && step == Step::Paper) {
        return 6;
    }

    0
}

#[derive(Copy, Clone, PartialEq)]
enum Step {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}
