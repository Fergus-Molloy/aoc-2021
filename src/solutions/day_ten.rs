use crate::advent_of_code::AdventOfCodeInput;

pub fn solve(aoc_input: AdventOfCodeInput) -> String {
    let lines: Vec<_> = aoc_input.inp.lines().map(str::to_owned).collect();
    let pt1 = part_one(&lines);
    let pt2 = part_two(&lines);
    format!("Day 10: ({},{})", pt1, pt2)
}

#[inline(always)]
fn get_matching(c: char) -> char {
    match c {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => unreachable!("no chars will match this"),
    }
}

fn is_corrupted(line: &str) -> u64 {
    let mut stack = Vec::with_capacity(line.len());
    for c in line.chars() {
        match c {
            ')' | '}' | ']' | '>' => {
                let top = stack.pop().unwrap();
                let matching = get_matching(top);
                if matching != c {
                    match c {
                        ')' => return 3,
                        ']' => return 57,
                        '}' => return 1197,
                        '>' => return 25137,
                        _ => unreachable!("no char matches this"),
                    }
                }
            }
            _ => stack.push(c),
        }
    }
    0
}

pub fn part_one(all_lines: &[String]) -> u64 {
    all_lines
        .iter()
        .filter_map(|x| {
            let score = is_corrupted(x);
            if score > 0 {
                Some(score)
            } else {
                None
            }
        })
        .sum::<u64>()
}

#[inline(always)]
fn get_score(c: char) -> u64 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => unreachable!("no char will match this"),
    }
}

pub fn part_two(all_lines: &[String]) -> u64 {
    let incomplete = all_lines
        .iter()
        .filter(|x| is_corrupted(x) == 0)
        .collect::<Vec<_>>();
    let mut totals = Vec::with_capacity(incomplete.len());
    for line in incomplete {
        let mut total = 0;
        let mut stack = Vec::new();
        for c in line.chars() {
            match c {
                ')' | ']' | '}' | '>' => {
                    stack.pop();
                }
                _ => stack.push(c),
            }
        }
        while let Some(top) = stack.pop() {
            let matching = get_matching(top);
            total *= 5;
            total += get_score(matching);
        }
        totals.push(total);
    }
    totals.sort_unstable();
    *totals.get(totals.len() / 2).unwrap()
}

#[cfg(test)]
mod test {
    use super::{part_one, part_two};
    use crate::advent_of_code::AdventOfCodeInput;
    #[test]
    fn d10a() {
        let aoc_input = AdventOfCodeInput::get_input(10);
        let depths: Vec<_> = aoc_input.inp.lines().map(str::to_owned).collect();
        assert_eq!(part_one(&depths), 469755);
    }
    #[test]
    fn d10b() {
        let aoc_input = AdventOfCodeInput::get_input(10);
        let depths: Vec<_> = aoc_input.inp.lines().map(str::to_owned).collect();
        assert_eq!(part_two(&depths), 2762335572);
    }
}
