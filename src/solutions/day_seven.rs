use crate::advent_of_code::AdventOfCodeInput;
const MAX: u64 = 1887;

pub fn solve(aoc_input: AdventOfCodeInput) -> String {
    let crabs: Vec<u64> = aoc_input
        .inp
        .split(',')
        .map(|x| x.trim().parse::<u64>().unwrap())
        .collect();
    let pt1 = part_one(&crabs);
    let pt2 = part_two(&crabs);
    format!("Day 7: ({},{})", pt1, pt2)
}

fn to_fuel(crab: u64, pos: u64) -> u64 {
    (crab as i64 - pos as i64).abs() as u64
}

fn to_fuel_exp(crab: u64, pos: u64) -> u64 {
    let dist = (crab as i64 - pos as i64).abs() as u64;
    (dist * (dist + 1) / 2) as u64
}

pub fn part_one(crabs: &[u64]) -> u64 {
    let mut min = u64::MAX;
    for i in 0..MAX {
        let fuel = crabs.to_owned().iter().map(|x| to_fuel(*x, i)).sum::<u64>();
        min = if fuel < min { fuel } else { min };
    }
    min
}

pub fn part_two(crabs: &[u64]) -> u64 {
    let mut min = u64::MAX;
    for i in 0..MAX {
        let fuel = crabs
            .to_owned()
            .iter()
            .map(|x| to_fuel_exp(*x, i))
            .sum::<u64>();
        min = if fuel < min { fuel } else { min };
    }
    min
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn d7a() {
        let aoc_input = AdventOfCodeInput::get_input(7);
        let crabs: Vec<u64> = aoc_input
            .inp
            .split(',')
            .map(|x| x.trim().parse::<u64>().unwrap())
            .collect();
        assert_eq!(part_one(&crabs), 359648);
    }
    #[test]
    fn d7b() {
        let aoc_input = AdventOfCodeInput::get_input(7);
        let crabs: Vec<u64> = aoc_input
            .inp
            .split(',')
            .map(|x| x.trim().parse::<u64>().unwrap())
            .collect();
        assert_eq!(part_two(&crabs), 100727924);
    }
}
