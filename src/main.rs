use aoc_2021::advent_of_code::AdventOfCodeInput;
use aoc_2021::solutions::{day_one, day_two};
fn main() {
    for i in 1..3 {
        let input = AdventOfCodeInput::get_input(i);
        match i {
            1 => day_one::solve(input),
            2 => day_two::solve(input),
            _ => unimplemented!(),
        }
    }
}
