use aoc_2021::advent_of_code::AdventOfCodeInput;
use aoc_2021::solutions::day_one;
fn main() {
    for i in 1..2 {
        let input = AdventOfCodeInput::get_input(i);
        match i {
            1 => day_one::solve(input),
            _ => unimplemented!(),
        }
    }
}
