use std::fs;
#[derive(Debug)]
pub struct AdventOfCodeInput {
    inp: String,
}

impl AdventOfCodeInput {
    pub fn get_input(day: u32) -> AdventOfCodeInput {
        let path = format!("inputs/day{}", day);
        let inp =
            fs::read_to_string(path).expect(&format!("Could not read input for day {}", day)[..]);
        AdventOfCodeInput { inp }
    }
}
