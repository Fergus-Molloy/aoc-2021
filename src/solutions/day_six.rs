use crate::advent_of_code::AdventOfCodeInput;
use rustc_hash::FxHashMap;
pub fn solve(aoc_input: AdventOfCodeInput) -> String {
    let fish: Vec<u64> = aoc_input
        .inp
        .split(',')
        .map(|x| x.trim().parse::<u64>().unwrap())
        .collect();
    let mut fish_ages = FxHashMap::default();
    for f in fish {
        let _ = match fish_ages.get(&f) {
            Some(val) => {
                let new_val = val + 1;
                fish_ages.insert(f, new_val)
            }
            None => fish_ages.insert(f, 1),
        };
    }
    let pt1 = part_one(&fish_ages);
    let pt2 = part_two(&fish_ages);
    format!("Day 6: ({},{})", pt1, pt2)
}

fn execute(fish: &FxHashMap<u64, u64>, lim: u64) -> u64 {
    let mut curr_fish = fish.clone();
    for _ in 0..lim {
        let mut new_map = FxHashMap::default();

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
                    Some(v) => {
                        let new_val = v + val;
                        new_map.insert(key - 1, new_val)
                    }
                    None => new_map.insert(key - 1, val),
                };
            }
        }

        curr_fish = new_map.clone();
    }
    curr_fish.iter().map(|(_, val)| val).sum::<u64>()
}

pub fn part_one(fish: &FxHashMap<u64, u64>) -> u64 {
    execute(fish, 80)
}

pub fn part_two(fish: &FxHashMap<u64, u64>) -> u64 {
    execute(fish, 256)
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn d6a() {
        let aoc_input = AdventOfCodeInput::get_input(6);
        let fish: Vec<u64> = aoc_input
            .inp
            .split(',')
            .map(|x| x.trim().parse::<u64>().unwrap())
            .collect();
        let mut fish_ages = FxHashMap::default();
        for f in fish {
            let _ = match fish_ages.get(&f) {
                Some(val) => {
                    let new_val = val + 1;
                    fish_ages.insert(f, new_val)
                }
                None => fish_ages.insert(f, 1),
            };
        }
        assert_eq!(part_one(&fish_ages), 371379);
    }
    #[test]
    fn d6b() {
        let aoc_input = AdventOfCodeInput::get_input(6);
        let fish: Vec<u64> = aoc_input
            .inp
            .split(',')
            .map(|x| x.trim().parse::<u64>().unwrap())
            .collect();
        let mut fish_ages = FxHashMap::default();
        for f in fish {
            match fish_ages.get(&f) {
                Some(val) => {
                    let new_val = val + 1;
                    fish_ages.insert(f, new_val);
                }
                None => {
                    fish_ages.insert(f, 1);
                }
            };
        }
        assert_eq!(part_two(&fish_ages), 1674303997472);
    }
}
