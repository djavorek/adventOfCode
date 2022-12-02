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
        let part_1_decryption: HashMap<char, Meaning> = HashMap::from([
            ('A', Meaning::Rock),
            ('B', Meaning::Paper),
            ('C', Meaning::Scissors),
            ('X', Meaning::Rock),
            ('Y', Meaning::Paper),
            ('Z', Meaning::Scissors),
        ]);

        let input_lines = vec_of_strings(input);

        return input_lines.iter()
            .map(|line| line.split_once(" ").unwrap())
            .map(|touple| {
                let step = find_step_according_to_map(touple.0, part_1_decryption.to_owned());
                let own_step = find_step_according_to_map(touple.1, part_1_decryption.to_owned());
                calculate_total_round_score(step, own_step)
            } )
            .fold(0, |acc, score| acc + score as u32);
    }

    fn solve_part_2(&self, input: &str) -> u32 {
        let part_2_decryption: HashMap<char, Meaning> = HashMap::from([
            ('A', Meaning::Rock),
            ('B', Meaning::Paper),
            ('C', Meaning::Scissors),
            ('X', Meaning::Lose),
            ('Y', Meaning::Draw),
            ('Z', Meaning::Win),
        ]);

        let input_lines = vec_of_strings(input);

        return input_lines.iter()
            .map(|line| line.split_once(" ").unwrap())
            .map(|touple| {
                let step = find_step_according_to_map(touple.0, part_2_decryption.to_owned());
                let own_step = find_step_according_to_map(touple.1, part_2_decryption.to_owned());
                calculate_total_round_score(step, own_step)
            } )
            .fold(0, |acc, score| acc + score as u32);
    }
}

fn calculate_total_round_score(step: Meaning, own_step: Meaning) -> u8 {
    let round_result_score = calculate_round_result_score(step, own_step);
    round_result_score + own_step as u8
}

fn calculate_round_result_score(step: Meaning, own_step: Meaning) -> u8 {
    if step == own_step {
        return 3;
    }

    if (own_step == Meaning::Rock && step == Meaning::Scissors) ||
       (own_step == Meaning::Paper && step == Meaning::Rock) || 
       (own_step == Meaning::Scissors && step == Meaning::Paper) {
        return 6;
    }

    0
}

fn find_step_according_to_map(input: &str, meaning_map: HashMap<char, Meaning>) -> Meaning {
    let step_char = input.chars().next().expect("Input is lazy, eww.");
    return *meaning_map.get(&step_char).expect("Input is playing by its own rules, JEEZ");
}

#[derive(Copy, Clone, PartialEq)]
enum Meaning {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
    Win,
    Lose,
    Draw,
}
