use std::collections::HashMap;

use crate::util::vec_of_strings;

use super::Day;

pub struct Day02;
static DATE: u8 = 2;

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
            .map(|input_parts| {
                let step = find_meaning(input_parts.0, &part_1_decryption);
                let own_step = find_meaning(input_parts.1, &part_1_decryption);
                calculate_score_from_steps(step, own_step)
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
            .map(|input_parts| {
                let step = find_meaning(input_parts.0, &part_2_decryption);
                let outcome = find_meaning(input_parts.1, &part_2_decryption);
                let required_step = get_step_for_outcome(step, outcome);
                calculate_score_from_steps(step, required_step)
            } )
            .fold(0, |acc, score| acc + score as u32);
    }

    fn get_expected_result_1(&self) -> u32 {
        12679
    }

    fn get_expected_result_2(&self) -> u32 {
        14470
    }
}

fn calculate_score_from_steps(step: Meaning, own_step: Meaning) -> u8 {
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

fn get_step_for_outcome(step: Meaning, outcome: Meaning) -> Meaning {
    if outcome == Meaning::Draw {
        return step
    }
    if outcome == Meaning::Win {
        return match step {
            Meaning::Paper => Meaning::Scissors,
            Meaning::Scissors => Meaning::Rock,
            Meaning::Rock => Meaning::Paper,
            _ => Meaning::Rock,
        };
    } else {
        return match step {
            Meaning::Paper => Meaning::Rock,
            Meaning::Scissors => Meaning::Paper,
            Meaning::Rock => Meaning::Scissors,
            _ => Meaning::Rock,
        };
    }
}

fn find_meaning(input: &str, meaning_map: &HashMap<char, Meaning>) -> Meaning {
    let character = input.chars().next().expect("Input is lazy, eww.");
    return *meaning_map.get(&character).expect("Input is playing by its own rules, JEEZ");
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
