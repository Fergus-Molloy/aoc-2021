use aoc_2021::advent_of_code::AdventOfCodeInput;
use aoc_2021::solutions::{day_one, day_three, day_two};
use chrono::prelude::*;
fn main() {
    let start = Utc::now();
    for i in 3..4 {
        let input = AdventOfCodeInput::get_input(i);
        match i {
            1 => day_one::solve(input),
            2 => day_two::solve(input),
            3 => day_three::solve(input),
            _ => unimplemented!(),
        }
    }
    let end = Utc::now();
    let dur = end - start;
    println!("{}", dur.num_nanoseconds().unwrap());
}
