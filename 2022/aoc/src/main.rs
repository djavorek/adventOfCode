use std::fmt::Display;
use aoc::day::{Day, day_02::Day02};

fn main() {
    println!("Hello, Santa!");
    print_results(Day02 {});
}

fn print_results<R: Display, T: Day<R>>(day: T) { 
    let (day, result_1, result_2) = day.solve();

    println!("******** Results of Day {} ********", day);
    println!("* Part One: {}", result_1);
    println!("* Part Two: {}", result_2);
    println!("*******************************");
}
