use crate::advent_of_code::AdventOfCodeInput;
#[derive(Debug, Clone, Copy)]
pub struct Movement {
    direction: Direction,
    distance: i64,
}
#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Forward,
}

#[derive(Clone, Copy)]
pub struct Position {
    horz: i64,
    vert: i64,
    aim: i64,
}

impl Position {
    fn move_pos(&mut self, movement: &Movement) {
        match movement.direction {
            Direction::Up | Direction::Down => self.aim += movement.distance,
            Direction::Forward => {
                self.horz += movement.distance;
                self.vert += movement.distance * self.aim;
            }
        }
    }
}

pub fn parse(inp: &str) -> Vec<Movement> {
    inp.lines()
        .map(|x| {
            let mut parts = x.split(' ');
            let direction = match parts.next().unwrap() {
                "up" => Direction::Up,
                "down" => Direction::Down,
                "forward" => Direction::Forward,
                _ => unreachable!("direction not recognised"),
            };
            let dist = parts.next().unwrap().parse::<i64>().unwrap();
            Movement {
                direction,
                distance: match direction {
                    Direction::Up => -dist,
                    Direction::Down | Direction::Forward => dist,
                },
            }
        })
        .collect()
}

pub fn solve(aoc_input: AdventOfCodeInput) -> String {
    let directions = parse(&aoc_input.inp);
    let (pt1_x, pt1_y) = part_one(&directions);
    let (pt2_x, pt2_y) = part_two(&directions);
    format!("Day 2: ({},{})", pt1_x * pt1_y, pt2_x * pt2_y)
}

pub fn part_one(directions: &[Movement]) -> (i64, i64) {
    let mut horz_dist = 0;
    let mut vert_dist = 0;
    let mut mov_horz = |x: i64| horz_dist += x;
    let mut mov_vert = |x: i64| vert_dist += x;
    for mov in directions {
        match mov.direction {
            Direction::Up | Direction::Down => mov_vert(mov.distance),
            Direction::Forward => mov_horz(mov.distance),
        }
    }
    (horz_dist, vert_dist)
}

pub fn part_two(directions: &[Movement]) -> (i64, i64) {
    let mut pos = Position {
        horz: 0,
        vert: 0,
        aim: 0,
    };
    for dir in directions {
        pos.move_pos(dir);
    }
    (pos.horz, pos.vert)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::advent_of_code::AdventOfCodeInput;
    #[test]
    fn d2a() {
        let aoc_input = AdventOfCodeInput::get_input(2);
        let inp = parse(&aoc_input.inp);

        let (a, b) = part_one(&inp);
        assert_eq!(a * b, 2_019_945);
    }
    #[test]
    fn d2b() {
        let aoc_input = AdventOfCodeInput::get_input(2);
        let inp = parse(&aoc_input.inp);

        let (a, b) = part_two(&inp);
        assert_eq!(a * b, 1_599_311_480);
    }
}
