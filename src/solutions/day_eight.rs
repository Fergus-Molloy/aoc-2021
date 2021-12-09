#![allow(unused_must_use)]
use crate::advent_of_code::AdventOfCodeInput;
use rustc_hash::FxHashMap;
#[derive(Debug)]
pub struct Notes {
    pub input: Vec<String>,
    pub output: Vec<String>,
}

pub fn solve(aoc_input: AdventOfCodeInput) -> String {
    let mut notes: Vec<_> = aoc_input
        .inp
        .lines()
        .map(|x| {
            let first = x.split('|').next().unwrap();
            let second = x.split('|').nth(1).unwrap();
            let input = first
                .split(' ')
                .filter(|x| !x.is_empty())
                .map(|x| {
                    let mut sorted = x.trim().chars().collect::<Vec<char>>();
                    sorted.sort_unstable();
                    String::from_iter(sorted)
                })
                .collect();
            let output = second
                .trim()
                .split(' ')
                .filter(|x| !x.is_empty())
                .map(|x| {
                    let mut sorted = x.trim().chars().collect::<Vec<char>>();
                    sorted.sort_unstable();
                    String::from_iter(sorted)
                })
                .collect();
            Notes { input, output }
        })
        .collect();
    let pt1 = part_one(&notes);
    let pt2 = part_two(&mut notes);
    format!("Day 8: ({},{})", pt1, pt2)
}

pub fn part_one(notes: &[Notes]) -> u64 {
    let parts = notes.iter().map(|x| &x.output).map(|x| {
        x.iter()
            .filter(|y| matches!(y.len(), 2 | 3 | 4 | 7))
            .count() as u64
    });
    parts.sum::<u64>()
}

fn pattern_diff(left: &str, right: &str) -> bool {
    let mut res = false;
    for c in right.chars() {
        if !left.contains(&c.to_string()) {
            res = true;
        }
    }
    res
}

fn reverse_diff(left: &str, right: &str) -> bool {
    let mut res = true;
    for c in left.chars() {
        if !right.contains(&c.to_string()) {
            res = false;
        }
    }
    res
}

pub fn part_two(notes: &mut Vec<Notes>) -> u64 {
    let mut values = Vec::new();
    for display in notes {
        display.input[..].sort_by(|a, b| a.len().cmp(&b.len()));
        let mut map = FxHashMap::default();
        map.try_insert(1, display.input[0].clone());
        map.try_insert(7, display.input[1].clone());
        map.try_insert(4, display.input[2].clone());
        map.try_insert(8, display.input[display.input.len() - 1].clone());

        let five_long = display.input[3..6].iter();
        let six_long = display.input[6..display.input.len() - 1].iter();
        // find 6
        for code in six_long.clone() {
            if let Some(pattern) = map.get(&1) {
                if pattern_diff(code, pattern) {
                    map.try_insert(6, code.clone());
                    break;
                }
            }
        }
        // find 0
        for code in six_long.clone() {
            if map.values().any(|x| x == code) {
                continue;
            }
            if let Some(pattern) = map.get(&4) {
                if pattern_diff(code, pattern) {
                    map.try_insert(0, code.clone());
                    break;
                }
            }
        }
        // find 9
        for code in six_long {
            if !map.values().any(|x| x == code) {
                map.try_insert(9, code.clone());
                break;
            }
        }
        // find 5
        for code in five_long.clone() {
            if let Some(pattern) = map.get(&6) {
                if reverse_diff(code, pattern) {
                    map.try_insert(5, code.clone());
                    break;
                }
            }
        }
        // find 3
        for code in five_long.clone() {
            if map.values().any(|x| x == code) {
                continue;
            }
            if let Some(pattern) = map.get(&9) {
                if reverse_diff(code, pattern) {
                    map.try_insert(3, code.clone());
                    break;
                }
            }
        }
        // find 2
        for code in five_long {
            if !map.values().any(|x| x == code) {
                map.try_insert(2, code.clone());
            }
        }
        let mut value = String::new();
        let mut final_map = FxHashMap::default();
        for (key, val) in map.clone() {
            final_map.insert(val, key);
        }
        for val in display.output.clone() {
            value.push_str(&format!("{}", final_map.get(&val).unwrap()));
        }
        values.push(value.parse::<u64>().unwrap());
    }
    values.iter().sum::<u64>()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::advent_of_code::AdventOfCodeInput;
    #[test]
    fn d8a() {
        let aoc_input = AdventOfCodeInput::get_input(8);
        let notes: Vec<_> = aoc_input
            .inp
            .lines()
            .map(|x| {
                let first = x.split('|').next().unwrap();
                let second = x.split('|').nth(1).unwrap();
                let input = first
                    .split(' ')
                    .filter(|x| !x.is_empty())
                    .map(|x| String::from(x.trim()))
                    .collect();
                let output = second
                    .trim()
                    .split(' ')
                    .filter(|x| !x.is_empty())
                    .map(|x| String::from(x.trim()))
                    .collect();
                Notes { input, output }
            })
            .collect();
        assert_eq!(part_one(&notes), 272);
    }
    #[test]
    fn d8b() {
        let aoc_input = AdventOfCodeInput::get_input(8);
        let mut notes: Vec<_> = aoc_input
            .inp
            .lines()
            .map(|x| {
                let first = x.split('|').next().unwrap();
                let second = x.split('|').nth(1).unwrap();
                let input = first
                    .split(' ')
                    .filter(|x| !x.is_empty())
                    .map(|x| {
                        let mut sorted = x.trim().chars().collect::<Vec<char>>();
                        sorted.sort_unstable();
                        String::from_iter(sorted)
                    })
                    .collect();
                let output = second
                    .trim()
                    .split(' ')
                    .filter(|x| !x.is_empty())
                    .map(|x| {
                        let mut sorted = x.trim().chars().collect::<Vec<char>>();
                        sorted.sort_unstable();
                        String::from_iter(sorted)
                    })
                    .collect();
                Notes { input, output }
            })
            .collect();
        assert_eq!(part_two(&mut notes), 1007675);
    }
}
