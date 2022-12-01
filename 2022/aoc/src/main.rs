use std::fmt::Display;
use aoc::day::{days::Day01, Day};

fn main() {
    println!("Hello, Santa!");
    print_results(Day01 {});

}

fn print_results<R: Display, T: Day<R>>(day: T) { 
    let (day_name, result_1, result_2) = day.solve();

    println!("******** Results of {} ********", day_name);
    println!("* Part One: {}", result_1);
    println!("* Part Two: {}", result_2);
    println!("*******************************");
}
