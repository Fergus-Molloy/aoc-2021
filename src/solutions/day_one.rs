use crate::advent_of_code::AdventOfCodeInput;

pub fn solve(aoc_input: AdventOfCodeInput) -> String {
    let depths: Vec<_> = aoc_input
        .inp
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    let pt1 = part_one(&depths);
    let pt2 = part_two(depths);
    format!("Day 1: ({},{})", pt1, pt2)
}

pub fn part_one(depths: &Vec<i64>) -> u64 {
    let mut count = 0;
    let mut last_depth = &depths[0];
    for current_depth in depths {
        if current_depth > last_depth {
            count += 1;
        }
        last_depth = current_depth;
    }
    count
}

pub fn part_two(depths: Vec<i64>) -> u64 {
    let mut count = 0;
    // get first depth
    let mut last_depth = depths.clone().into_iter().take(3).sum::<i64>() as u64;
    let slice = &depths[..];
    // iterate over windows of size 3
    for window in slice.windows(3) {
        let mut total = 0;
        // sum items in window
        for i in window.iter() {
            total += i;
        }
        // count ones where depth increases
        if total as u64 > last_depth {
            count += 1;
        }
        last_depth = total as u64;
    }
    count
}

#[cfg(test)]
mod test {
    use super::{part_one, part_two};
    use crate::advent_of_code::AdventOfCodeInput;
    #[test]
    fn d1a() {
        let aoc_input = AdventOfCodeInput::get_input(1);
        let depths: Vec<_> = aoc_input
            .inp
            .lines()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        assert_eq!(part_one(&depths), 1722);
    }
    #[test]
    fn d1b() {
        let aoc_input = AdventOfCodeInput::get_input(1);
        let depths: Vec<_> = aoc_input
            .inp
            .lines()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        assert_eq!(part_two(depths), 1748);
    }
}
