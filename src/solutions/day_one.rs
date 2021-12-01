use crate::advent_of_code::AdventOfCodeInput;
pub fn solve(aoc_input: AdventOfCodeInput) {
    let depths: Vec<i64> = aoc_input
        .inp
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    let pt1 = part_one(&depths);
    let pt2 = part_two(depths);
    println!("Day 1: ({},{})", pt1, pt2);
}

fn part_one(depths: &Vec<i64>) -> u64 {
    let mut count = 0;
    let mut last_depth = depths[0];
    for current_depth in depths {
        if *current_depth > last_depth {
            count += 1;
        }
        last_depth = *current_depth;
    }
    count
}

fn part_two(depths: Vec<i64>) -> u64 {
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
