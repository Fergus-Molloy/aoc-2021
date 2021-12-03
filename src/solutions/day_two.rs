use crate::advent_of_code::AdventOfCodeInput;
#[derive(Debug, Clone, Copy)]
struct Movement {
    direction: Direction,
    distance: i64,
}
#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Forward,
    Backward,
}

#[derive(Clone, Copy)]
struct Position {
    horz: i64,
    vert: i64,
    aim: i64,
}

impl Position {
    fn move_pos(&mut self, movement: &Movement) {
        match movement.direction {
            Direction::Up => self.aim += movement.distance,
            Direction::Down => self.aim += movement.distance,
            Direction::Forward => {
                self.horz += movement.distance;
                self.vert += movement.distance * self.aim;
            }
            Direction::Backward => {
                self.horz += movement.distance;
                self.vert += movement.distance * self.aim;
            }
        }
    }
}

fn parse(inp: String) -> Vec<Movement> {
    inp.lines()
        .map(|x| {
            let mut parts = x.split(' ');
            let direction = match parts.next().unwrap() {
                "up" => Direction::Up,
                "down" => Direction::Down,
                "forward" => Direction::Forward,
                "backward" => Direction::Backward,
                _ => panic!("direction not recognised"),
            };
            let dist = parts.next().unwrap().parse::<i64>().unwrap();
            Movement {
                direction: direction,
                distance: match direction {
                    Direction::Up => -dist,
                    Direction::Down => dist,
                    Direction::Forward => dist,
                    Direction::Backward => -dist,
                },
            }
        })
        .collect()
}

pub fn solve(aoc_input: AdventOfCodeInput) {
    let directions = parse(aoc_input.inp);
    let (pt1_x, pt1_y) = part_one(&directions);
    let (pt2_x, pt2_y) = part_two(&directions);
    println!("Day 2: ({},{})", pt1_x * pt1_y, pt2_x * pt2_y);
}

fn part_one(directions: &Vec<Movement>) -> (i64, i64) {
    let mut horz_dist = 0;
    let mut vert_dist = 0;
    let mut mov_horz = |x: i64| horz_dist += x;
    let mut mov_vert = |x: i64| vert_dist += x;
    for mov in directions {
        match mov.direction {
            Direction::Up => mov_vert(mov.distance),
            Direction::Down => mov_vert(mov.distance),
            Direction::Forward => mov_horz(mov.distance),
            Direction::Backward => mov_horz(mov.distance),
        }
    }
    (horz_dist, vert_dist)
}

fn part_two(directions: &Vec<Movement>) -> (i64, i64) {
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
