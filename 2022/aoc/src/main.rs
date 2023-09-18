use std::fmt::Display;
use aoc::day::{Day, day_09::Day09};

fn main() {
    println!("Hello, Santa!");
    print_results(Day09);
}

fn print_results<R: Display + std::cmp::PartialEq + std::fmt::Debug , T: Day<R>>(day: T) { 
    let (day, result_1, result_2) = day.solve();

    println!("******** Results of Day {} ********", day);
    println!("* Part One: {}", result_1);
    println!("* Part Two: {}", result_2);
    println!("*******************************");
}
