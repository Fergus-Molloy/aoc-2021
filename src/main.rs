use aoc_2021::advent_of_code::AdventOfCodeInput;
use aoc_2021::solutions::{day_five, day_four, day_one, day_six, day_three, day_two};
use chrono::prelude::*;
fn main() {
    let start = Utc::now();
    for i in 6..7 {
        let input = AdventOfCodeInput::get_input(i);
        match i {
            1 => day_one::solve(input),
            2 => day_two::solve(input),
            3 => day_three::solve(input),
            4 => day_four::solve(input),
            5 => day_five::solve(input),
            6 => day_six::solve(input),
            _ => unimplemented!(),
        }
    }
    let end = Utc::now();
    let dur = end - start;
    println!("{}s", dur.num_nanoseconds().unwrap() as f64 / 1e9);
}
