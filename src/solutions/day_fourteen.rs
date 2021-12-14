use crate::advent_of_code::AdventOfCodeInput;
use rustc_hash::FxHashMap;

pub fn parse<'a>(inp: &'a str) -> (Vec<char>, FxHashMap<(char, char), char>) {
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
            let mut to = split.next().unwrap().chars();
            let a = to.next().unwrap();
            let b = to.next().unwrap();
            map.insert((a, b), split.next().unwrap().chars().next().unwrap());
        }
    }
    (template, map)
}

pub fn solve(aoc_input: AdventOfCodeInput) -> String {
    let (mut template, map) = parse(&aoc_input.inp);
    let pt1 = part_one(&mut template.clone(), &map);
    let pt2 = part_two(&mut template, &map);
    format!("Day 1: ({},{})", pt1, pt2)
}

fn step(template: &mut Vec<char>, map: &FxHashMap<(char, char), char>) {
    let mut to_add = Vec::new();
    for idx in 0..(template.len() - 1) {
        let lookup = (template[idx], template[idx + 1]);
        to_add.push(map.get(&lookup).unwrap());
    }
    let mut count = 1;
    for a in to_add {
        template.insert(count, *a);
        count += 2;
    }
}

fn execute(template: &mut Vec<char>, map: &FxHashMap<(char, char), char>, lim: usize) -> u64 {
    for _ in 0..lim {
        step(template, map);
    }
    let (max, min) = most_least_freq(template);
    (max - min) as u64
}

fn fast_execute(
    template: &mut Vec<char>,
    formulea: &FxHashMap<(char, char), char>,
    lim: usize,
) -> u64 {
    let mut counts: FxHashMap<(char, char), u64> = FxHashMap::default();
    let mut elements: FxHashMap<char, u64> = FxHashMap::default();
    for idx in 0..(template.len() - 1) {
        let lookup = (template[idx], template[idx + 1]);
        *counts.entry(lookup).or_default() += 1;
        *elements.entry(template[idx]).or_default() += 1;
    }
    *elements.entry(template[template.len() - 1]).or_default() += 1; // don't forget the last element

    for _ in 0..lim {
        let to_polymerize = counts.clone();
        for (key, val) in &to_polymerize {
            let pt1 = (key.0, *formulea.get(key).unwrap());
            let pt2 = (*formulea.get(key).unwrap(), key.1);
            *counts.entry(*key).or_default() -= val; //remove old pairing
            *counts.entry(pt1).or_default() += val;
            *counts.entry(pt2).or_default() += val;
            *elements.entry(pt1.1).or_default() += val; //update element counts
        }
    }
    let mut v = counts.into_iter().collect::<Vec<((char, char), u64)>>();
    v.sort_by(|a, b| a.1.cmp(&b.1));
    elements.values().max().unwrap() - elements.values().min().unwrap()
}

fn most_least_freq(array: &[char]) -> (usize, usize) {
    let mut map = FxHashMap::default();
    for x in array {
        *map.entry(x).or_default() += 1;
    }
    (*map.values().max().unwrap(), *map.values().min().unwrap())
}

pub fn part_one(template: &mut Vec<char>, map: &FxHashMap<(char, char), char>) -> u64 {
    fast_execute(template, map, 10)
}

pub fn part_two(template: &mut Vec<char>, map: &FxHashMap<(char, char), char>) -> u64 {
    fast_execute(template, map, 40)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::advent_of_code::AdventOfCodeInput;
    #[test]
    fn d14a() {
        let aoc_input = AdventOfCodeInput::get_input(14);
        let (mut template, map) = parse(&aoc_input.inp);
        assert_eq!(part_one(&mut template, &map), 2657);
    }
    #[test]
    fn d14b() {
        let aoc_input = AdventOfCodeInput::get_input(14);
        let (mut template, map) = parse(&aoc_input.inp);
        assert_eq!(part_two(&mut template, &map), 2911561572630);
    }
}
