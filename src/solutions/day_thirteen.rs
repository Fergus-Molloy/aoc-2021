use crate::advent_of_code::AdventOfCodeInput;

pub struct Paper {
    dots: Vec<Dot>,
    folds: Vec<Fold>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dot {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy)]
pub struct Fold {
    horizontal: bool,
    position: i64,
}

impl std::fmt::Debug for Paper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_x = self.dots.iter().map(|x| x.x).max().unwrap();
        let max_y = self.dots.iter().map(|y| y.y).max().unwrap();
        let mut string = String::new();
        for y in 0..=max_y {
            for x in 0..=max_x {
                if self.dots.iter().any(|&d| d == (Dot::new(x, y))) {
                    string.push('#');
                } else {
                    string.push('.');
                }
            }
            string.push('\n');
        }
        string.push('\n');
        for f in &self.folds {
            string.push_str(&format!(
                "fold along {}={}\n",
                if f.horizontal { "x" } else { "y" },
                f.position
            ));
        }
        write!(f, "{}", string)
    }
}

impl Fold {
    pub fn new(horizontal: bool, position: i64) -> Self {
        Self {
            horizontal,
            position,
        }
    }
}

impl Dot {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

pub fn parse(inp: String) -> Paper {
    let mut paper = Vec::new();
    let mut instructions = Vec::new();
    let mut inst = false;
    for line in inp.lines() {
        if line.is_empty() {
            inst = true;
            continue;
        }
        if inst {
            let mut split = line.split(' ');
            let mut instruction = split.nth(2).unwrap().split('=');
            let horz = instruction.next().unwrap() == "x";
            let position = instruction.next().unwrap();
            instructions.push(Fold::new(horz, position.parse::<i64>().unwrap()));
        } else {
            let mut split = line.split(',');
            let x = split.next().unwrap();
            let y = split.next().unwrap();
            paper.push(Dot::new(
                x.parse::<i64>().unwrap(),
                y.parse::<i64>().unwrap(),
            ));
        }
    }
    Paper {
        dots: paper,
        folds: instructions,
    }
}

pub fn solve(aoc_input: AdventOfCodeInput) -> String {
    let paper = parse(aoc_input.inp);
    println!("{:?}", paper);
    let pt1 = part_one(&paper);
    let pt2 = part_two(&paper);
    format!("Day 1: ({},{})", pt1, pt2)
}

pub fn part_one(depths: &Paper) -> u32 {
    0
}

pub fn part_two(depths: &Paper) -> u32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::advent_of_code::AdventOfCodeInput;
    #[test]
    fn d13a() {
        let aoc_input = AdventOfCodeInput::get_input(13);
        let paper = parse(aoc_input.inp);
        assert_eq!(part_one(&paper), 1722);
    }
    #[test]
    fn d13b() {
        let aoc_input = AdventOfCodeInput::get_input(13);
        let paper = parse(aoc_input.inp);
        assert_eq!(part_two(&paper), 1748);
    }
}
