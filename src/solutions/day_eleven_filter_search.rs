use crate::advent_of_code::AdventOfCodeInput;

const MAX_X: usize = 9;
const MAX_Y: usize = 9;

#[derive(Clone)]
pub struct Octopi {
    octopi: Vec<Octopus>,
}

#[derive(Clone, Copy)]
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
    let pt1 = part_one(&mut octopi.clone());
    let pt2 = part_two(&mut octopi);
    format!("Day 1: ({},{})", pt1, pt2)
}

fn flash(octopi: &mut [Octopus]) {
    let count = octopi.iter().filter(|x| x.highlighted).count();
    let to_flash = octopi.iter_mut().filter(|x| x.energy > 9 && !x.highlighted);
    let mut adj = Vec::new();
    for o in to_flash {
        o.highlighted = true;
        for pos in o.pos.get_adjacent() {
            adj.push(pos);
        }
    }
    for pos in adj {
        for o in octopi.iter_mut().filter(|x| x.pos == pos) {
            o.energy += 1;
        }
    }
    if octopi.iter().filter(|x| x.highlighted).count() != count {
        flash(octopi);
    }
}

fn step(octopi: &mut [Octopus]) -> u32 {
    for octopus in octopi.iter_mut() {
        octopus.highlighted = false;
        octopus.energy += 1;
    }
    flash(octopi);
    let highlighted_count = octopi.iter().filter(|x| x.highlighted).count();
    for o in octopi.iter_mut().filter(|x| x.highlighted) {
        o.energy = 0;
    }
    highlighted_count as u32
}

pub fn part_one(octopi: &mut Octopi) -> u32 {
    let mut total = 0;
    for _ in 0..100 {
        total += step(&mut octopi.octopi);
    }
    total
}

pub fn part_two(octopi: &mut Octopi) -> u32 {
    let mut steps = 0;
    let mut all_flash = false;
    let length = octopi.octopi.len();
    while !all_flash {
        let highlighted = step(&mut octopi.octopi);

        all_flash = highlighted == length as u32;
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
        let mut octopi = Octopi {
            octopi: aoc_input
                .inp
                .lines()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .map(|(x, o)| Octopus::new(o.to_string().parse::<u8>().unwrap(), x, y))
                        .collect::<Vec<Octopus>>()
                })
                .collect::<Vec<Octopus>>(),
        };
        assert_eq!(part_one(&mut octopi), 1652);
    }

    #[test]
    fn d11b() {
        let aoc_input = AdventOfCodeInput::get_input(11);
        let mut octopi = Octopi {
            octopi: aoc_input
                .inp
                .lines()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .map(|(x, o)| Octopus::new(o.to_string().parse::<u8>().unwrap(), x, y))
                        .collect::<Vec<Octopus>>()
                })
                .collect::<Vec<Octopus>>(),
        };
        assert_eq!(part_two(&mut octopi), 220);
    }
}
