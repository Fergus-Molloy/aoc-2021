use aoc_2021::advent_of_code::AdventOfCodeInput;
use aoc_2021::solutions::{day_five, day_four, day_one, day_seven, day_six, day_three, day_two};
use std::time::Instant;
fn main() {
    let start = Instant::now();
    for i in 1..8 {
        let input = AdventOfCodeInput::get_input(i);
        match i {
            1 => println!("{}", day_one::solve(input)),
            2 => println!("{}", day_two::solve(input)),
            3 => println!("{}", day_three::solve(input)),
            4 => println!("{}", day_four::solve(input)),
            5 => println!("{}", day_five::solve(input)),
            6 => println!("{}", day_six::solve(input)),
            7 => println!("{}", day_seven::solve(input)),
            _ => unimplemented!(),
        }
    }
    let end = Instant::now();
    println!("Total runtime {:?}", end - start);
}
