use crate::advent_of_code::AdventOfCodeInput;
use std::collections::HashMap;
pub fn solve(aoc_input: AdventOfCodeInput) {
    let fish: Vec<u64> = aoc_input
        .inp
        .split(',')
        .map(|x| x.trim().parse::<u64>().unwrap())
        .collect();
    let mut fish_ages = HashMap::new();
    for f in fish {
        let _ = match fish_ages.get(&f) {
            Some(val) => fish_ages.insert(f, val + 1),
            None => fish_ages.insert(f, 1),
        };
    }
    let pt1 = part_one(&fish_ages);
    let pt2 = part_two(&fish_ages);
    println!("Day 6: ({},{})", pt1, pt2);
}

fn execute(fish: &HashMap<u64, u64>, lim: u64) -> u64 {
    let mut curr_fish = fish.clone();
    for i in 0..lim {
        let mut new_map = HashMap::new();

        match curr_fish.get(&0) {
            Some(val) => {
                new_map.insert(8, *val);
                new_map.insert(6, *val);
            }
            None => (),
        }

        for (key, val) in curr_fish {
            if key == 0 {
                continue;
            } else {
                let _ = match new_map.get(&(key - 1)) {
                    Some(v) => new_map.insert(key - 1, v + val),
                    None => new_map.insert(key - 1, val),
                };
            }
        }

        curr_fish = new_map.clone();
    }
    curr_fish.iter().map(|(_, val)| val).sum::<u64>()
}

fn part_one(fish: &HashMap<u64, u64>) -> u64 {
    execute(fish, 80)
}

fn part_two(fish: &HashMap<u64, u64>) -> u64 {
    execute(fish, 256)
}
