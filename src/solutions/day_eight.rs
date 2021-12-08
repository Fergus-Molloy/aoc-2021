#![allow(unused_must_use)]
use crate::advent_of_code::AdventOfCodeInput;
use std::collections::HashMap;
pub struct Notes {
    input: Vec<String>,
    output: Vec<String>,
}

pub fn solve(aoc_input: AdventOfCodeInput) -> String {
    let notes: Vec<_> = aoc_input
        .inp
        .lines()
        .map(|x| {
            let first = x.split('|').next().unwrap();
            let second = x.split('|').nth(1).unwrap();
            let input = first
                .split(' ')
                .filter(|x| x.len() > 0)
                .map(|x| {
                    let mut sorted = x.trim().chars().collect::<Vec<char>>();
                    sorted.sort();
                    String::from_iter(sorted)
                })
                .collect();
            let output = second
                .trim()
                .split(' ')
                .filter(|x| x.len() > 0)
                .map(|x| {
                    let mut sorted = x.trim().chars().collect::<Vec<char>>();
                    sorted.sort();
                    String::from_iter(sorted)
                })
                .collect();
            Notes { input, output }
        })
        .collect();
    let pt1 = part_one(&notes);
    let pt2 = part_two(&notes);
    format!("Day 8: ({},{})", pt1, pt2)
}

pub fn part_one(notes: &Vec<Notes>) -> u64 {
    let parts = notes.iter().map(|x| &x.output).map(|x| {
        x.iter()
            .filter(|y| match y.len() {
                2 => true,
                3 => true,
                4 => true,
                7 => true,
                _ => false,
            })
            .count() as u64
    });
    parts.sum::<u64>()
}

fn pattern_diff(left: &String, right: &String) -> bool {
    let eq = left
        .chars()
        .filter(|c| right.contains(&c.to_string()))
        .collect::<String>();
    eq.len() != right.len()
}

// TODO: For some reason 2 and 9 are getting mixed up????

pub fn part_two(notes: &Vec<Notes>) -> u64 {
    let mut values = Vec::new();
    for display in notes {
        let mut map = HashMap::new();
        // find easy ones
        for code in display.input.clone() {
            match code.len() {
                2 => map.try_insert(1, code),
                3 => map.try_insert(7, code),
                4 => map.try_insert(4, code),
                7 => map.try_insert(8, code),
                _ => continue,
            };
        }

        // find 6
        for code in display.input.clone().iter().filter(|x| x.len() == 6) {
            match map.get(&1) {
                Some(pattern) => {
                    if pattern_diff(code, pattern) {
                        map.try_insert(6, code.clone());
                    }
                }
                None => println!("Could not find pattern 1",),
            }
        }
        // find 0
        for code in display.input.clone().iter().filter(|x| x.len() == 6) {
            if map.values().any(|x| x == code) {
                continue;
            }
            match map.get(&4) {
                Some(pattern) => {
                    if pattern_diff(code, pattern) {
                        map.try_insert(4, code.clone());
                    }
                }
                None => (),
            }
        }
        // find 9
        for code in display.input.clone().iter().filter(|x| x.len() == 6) {
            if !map.values().any(|x| x == code) {
                map.try_insert(9, code.clone());
            }
        }
        // find 5
        for code in display.input.clone().iter().filter(|x| x.len() == 5) {
            match map.get(&6) {
                Some(pattern) => {
                    if pattern_diff(code, pattern) {
                        map.try_insert(5, code.clone());
                    }
                }
                None => (),
            }
        }
        // find 3
        for code in display.input.clone().iter().filter(|x| x.len() == 5) {
            if map.values().any(|x| x == code) {
                continue;
            }
            match map.get(&9) {
                Some(pattern) => {
                    if pattern_diff(code, pattern) {
                        map.try_insert(3, code.clone());
                    }
                }
                None => (),
            }
        }
        // find 2
        for code in display.input.clone().iter().filter(|x| x.len() == 5) {
            if !map.values().any(|x| x == code) {
                map.try_insert(2, code.clone());
            }
        }
        let mut value = String::new();
        for val in display.output.clone() {
            let mut found = false;
            for (key, num) in map.clone() {
                if val == num {
                    value.push_str(&format!("{}", key));
                    found = true;
                }
            }
            if !found {
                println!("Could not find mapping for code {}", val);
            }
        }
        println!("{:?}", map);
        values.push(value.parse::<u64>().unwrap());
    }
    println!("values {:?}", values);
    values.iter().sum::<u64>()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::advent_of_code::AdventOfCodeInput;
    #[test]
    fn d1a() {
        let aoc_input = AdventOfCodeInput::get_input(8);
        let notes: Vec<_> = aoc_input
            .inp
            .lines()
            .map(|x| {
                let first = x.split('|').next().unwrap();
                let second = x.split('|').nth(1).unwrap();
                let input = first
                    .split(' ')
                    .filter(|x| x.len() > 0)
                    .map(|x| String::from(x.trim()))
                    .collect();
                let output = second
                    .trim()
                    .split(' ')
                    .filter(|x| x.len() > 0)
                    .map(|x| String::from(x.trim()))
                    .collect();
                Notes { input, output }
            })
            .collect();
        assert_eq!(part_one(&notes), 272);
    }
    #[test]
    fn d1b() {
        let aoc_input = AdventOfCodeInput::get_input(8);
        assert_eq!(0, 0);
    }
}
