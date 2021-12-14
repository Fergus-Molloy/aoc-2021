use crate::advent_of_code::AdventOfCodeInput;
use rustc_hash::FxHashMap;
use std::time::Instant;

pub fn parse<'a>(inp: &'a str) -> (Vec<char>, FxHashMap<&'a str, char>) {
    let mut template = Vec::new();
    let mut map = FxHashMap::default();
    for line in inp.lines() {
        if line.is_empty() {
            continue;
        }
        let mut split = line.split(" -> ");
        if split.clone().count() == 1 {
            template = line.chars().collect();
        } else {
            map.insert(
                split.next().unwrap(),
                split.next().unwrap().chars().next().unwrap(),
            );
        }
    }
    (template, map)
}

pub fn solve(aoc_input: AdventOfCodeInput) -> String {
    let (mut template, map) = parse(&aoc_input.inp);
    println!("template: {:?}", template);
    let pt1 = part_one(&mut template.clone(), &map);
    let pt2 = 0; //part_two(&mut template, &map);
    format!("Day 1: ({},{})", pt1, pt2)
}

fn step(template: &mut Vec<char>, map: &FxHashMap<&str, char>) {
    let mut to_add = Vec::new();
    for idx in 0..(template.len() - 1) {
        let lookup = format!("{}{}", template[idx], template[idx + 1]);
        to_add.push(map.get(&lookup[..]).unwrap());
    }
    let mut count = 1;
    for a in to_add {
        template.insert(count, *a);
        count += 2;
    }
}

fn most_least_freq(array: &[char]) -> (usize, usize) {
    let mut map = FxHashMap::default();
    for x in array {
        *map.entry(x).or_default() += 1;
    }
    (*map.values().max().unwrap(), *map.values().min().unwrap())
}

pub fn part_one(template: &mut Vec<char>, map: &FxHashMap<&str, char>) -> u64 {
    for i in 0..15 {
        let start = Instant::now();
        step(template, map);
        println!("completed step {} in {:?}", i, Instant::now() - start);
        println!("Size: {}\n", template.len());
    }
    let start = Instant::now();
    let (max, min) = most_least_freq(template);
    println!("completed search in {:?}", Instant::now() - start);
    (max - min) as u64
}

pub fn part_two(template: &mut Vec<char>, map: &FxHashMap<&str, char>) -> u64 {
    for _ in 0..40 {
        step(template, map);
    }
    let (max, min) = most_least_freq(template);
    (max - min) as u64
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::advent_of_code::AdventOfCodeInput;
    #[test]
    fn d1a() {
        let aoc_input = AdventOfCodeInput::get_input(1);
        let (mut template, map) = parse(&aoc_input.inp);
        assert_eq!(part_one(&mut template, &map), 1722);
    }
    #[test]
    fn d1b() {
        let aoc_input = AdventOfCodeInput::get_input(1);
        let (mut template, map) = parse(&aoc_input.inp);
        assert_eq!(part_two(&mut template, &map), 1748);
    }
}
