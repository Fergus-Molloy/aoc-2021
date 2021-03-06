use crate::advent_of_code::AdventOfCodeInput;
use std::cmp::Ordering;

const MAX_X: usize = 9;
const MAX_Y: usize = 9;

#[derive(Clone)]
pub struct Octopi {
    octopi: Vec<Octopus>,
    h_count: u32,
}

#[derive(Clone, Copy, Debug)]
pub struct Octopus {
    pos: Position,
    energy: u8,
    highlighted: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    #[inline(always)]
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    fn get_adjacent(&self) -> Vec<Position> {
        let mut adj = Vec::new();
        if self.x > 0 {
            //left
            adj.push(Position::new(self.x - 1, self.y));
        }
        if self.x < MAX_X {
            //right
            adj.push(Position::new(self.x + 1, self.y));
        }
        if self.y > 0 {
            //top
            adj.push(Position::new(self.x, self.y - 1));
        }
        if self.y < MAX_Y {
            //bottom
            adj.push(Position::new(self.x, self.y + 1));
        }
        if self.x > 0 && self.y > 0 {
            //top left
            adj.push(Position::new(self.x - 1, self.y - 1));
        }
        if self.x < MAX_X && self.y > 0 {
            // top right
            adj.push(Position::new(self.x + 1, self.y - 1));
        }
        if self.x < MAX_X && self.y < MAX_Y {
            //bottom right
            adj.push(Position::new(self.x + 1, self.y + 1));
        }
        if self.x > 0 && self.y < MAX_Y {
            //bottom left
            adj.push(Position::new(self.x - 1, self.y + 1));
        }
        adj
    }
}

#[allow(clippy::if_same_then_else)]
impl std::cmp::Ord for Position {
    fn cmp(&self, other: &Position) -> Ordering {
        if self.x == other.x && self.y == other.y {
            Ordering::Equal
        } else if self.x < other.x && self.y == other.y {
            Ordering::Less
        } else if self.x == other.x && self.y < other.y {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}
impl std::cmp::PartialOrd for Position {
    fn partial_cmp(&self, other: &Position) -> Option<Ordering> {
        if self.x == other.x && self.y == other.y {
            Some(Ordering::Equal)
        } else if self.x < other.x && self.y == other.y {
            Some(Ordering::Less)
        } else if self.x == other.x && self.y < other.y {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl Octopi {
    fn get(&mut self, pos: &Position) -> &mut Octopus {
        &mut self.octopi[pos.x + (pos.y * (MAX_X + 1))]
    }
}

impl Octopus {
    #[inline(always)]
    pub fn new(energy: u8, x: usize, y: usize) -> Self {
        Self {
            energy,
            highlighted: false,
            pos: Position::new(x, y),
        }
    }
}

pub fn parse(inp: String) -> Octopi {
    Octopi {
        h_count: 0,
        octopi: inp
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, o)| Octopus::new(o.to_string().parse::<u8>().unwrap(), x, y))
                    .collect::<Vec<Octopus>>()
            })
            .collect::<Vec<Octopus>>(),
    }
}

pub fn solve(aoc_input: AdventOfCodeInput) -> String {
    let mut octopi = parse(aoc_input.inp);
    octopi.octopi.sort_by(|a, b| a.pos.cmp(&b.pos));
    let pt1 = part_one(&mut octopi.clone());
    let pt2 = part_two(&mut octopi);
    format!("Day 1: ({},{})", pt1, pt2)
}

fn flash(s_octopi: &mut Octopi) {
    let count = s_octopi.h_count;
    let to_flash = s_octopi
        .octopi
        .iter_mut()
        .filter(|x| x.energy > 9 && !x.highlighted);
    let mut adj = Vec::new();
    for o in to_flash {
        o.highlighted = true;
        s_octopi.h_count += 1;
        adj.append(&mut o.pos.get_adjacent());
    }
    for pos in adj {
        s_octopi.get(&pos).energy += 1;
    }
    if s_octopi.h_count != count {
        flash(s_octopi);
    }
}

fn step(octopi: &mut Octopi) -> u32 {
    for octopus in &mut octopi.octopi {
        octopus.highlighted = false;
        octopus.energy += 1;
    }
    octopi.h_count = 0;
    flash(octopi);
    for o in octopi.octopi.iter_mut().filter(|x| x.highlighted) {
        o.energy = 0;
    }
    octopi.h_count
}

pub fn part_one(octopi: &mut Octopi) -> u32 {
    let mut total = 0;
    for _ in 0..100 {
        total += step(octopi);
    }
    total
}

pub fn part_two(octopi: &mut Octopi) -> u32 {
    let mut steps = 0;
    let mut all_flash = false;
    let length = octopi.octopi.len() as u32;
    while !all_flash {
        let highlighted = step(octopi);

        all_flash = highlighted == length;
        steps += 1;
    }
    steps
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::advent_of_code::AdventOfCodeInput;
    #[test]
    fn d11a() {
        let aoc_input = AdventOfCodeInput::get_input(11);
        let mut octopi = parse(aoc_input.inp);
        assert_eq!(part_one(&mut octopi), 1652);
    }

    #[test]
    fn d11b() {
        let aoc_input = AdventOfCodeInput::get_input(11);
        let mut octopi = parse(aoc_input.inp);
        assert_eq!(part_two(&mut octopi), 220);
    }
}
