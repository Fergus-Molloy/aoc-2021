use crate::advent_of_code::AdventOfCodeInput;
use std::collections::{HashMap, HashSet};
use std::fmt::Formatter;
#[derive(Debug, Clone)]
struct Line {
    start: Point,
    end: Point,
}
#[derive(Clone, Copy, Hash)]
struct Point {
    x: u64,
    y: u64,
}
impl std::cmp::Eq for Point {}
impl std::cmp::PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Point {
    fn new(x: u64, y: u64) -> Self {
        Point { x, y }
    }
    fn from_iter<'a, T: Iterator<Item = &'a str>>(iter: &mut T) -> Self {
        let x = iter.next().unwrap().parse::<u64>().unwrap();
        let y = iter.next().unwrap().parse::<u64>().unwrap();
        Point { x, y }
    }
}

impl Line {
    fn new(start: Point, end: Point) -> Self {
        Line { start, end }
    }

    fn generate_points(&self) -> Vec<Point> {
        let start = self.start;
        let end = self.end;
        let mut points = Vec::new();
        let x_range: Vec<u64> = if start.x < end.x {
            (start.x..(end.x + 1)).collect()
        } else {
            (end.x..(start.x + 1)).rev().collect()
        };
        let y_range: Vec<u64> = if start.y < end.y {
            (start.y..(end.y + 1)).collect()
        } else {
            (end.y..(start.y + 1)).rev().collect()
        };
        if start.x == end.x {
            let len = ((start.y as i64 - end.y as i64).pow(2) as f64).sqrt();
            if len < 2.0 {
                points.push(start);
                points.push(end);
            } else {
                for i in y_range {
                    points.push(Point::new(start.x, i));
                }
            }
        } else if start.y == end.y {
            let len = ((start.x as i64 - end.x as i64).pow(2) as f64).sqrt();
            if len < 2.0 {
                points.push(start);
                points.push(end);
            } else {
                for i in x_range {
                    points.push(Point::new(i, start.y));
                }
            }
        } else {
            for (x, y) in x_range.iter().zip(y_range) {
                points.push(Point::new(*x, y));
            }
        }
        points
    }

    fn is_diagonal(&self) -> bool {
        !(self.start.x == self.end.x || self.start.y == self.end.y)
    }
}

pub fn solve(aoc_input: AdventOfCodeInput) {
    let pt1 = part_one(&aoc_input.inp);
    let pt2 = part_two(&aoc_input.inp);
    println!("Day 5: ({},{})", pt1, pt2);
}

fn part_one(inp: &String) -> u64 {
    let mut map = HashMap::new();
    let lines = inp.lines();
    for line in lines {
        let mut parts = line.split(' ');
        let p1 = Point::from_iter(&mut parts.next().unwrap().split(',').clone());
        parts.next(); // get rid of arrow
        let p2 = Point::from_iter(&mut parts.next().unwrap().split(',').clone());
        let line = Line::new(p1, p2);
        if !line.is_diagonal() {
            for point in line.generate_points() {
                let _ = match map.get(&point) {
                    Some(val) => map.insert(point, val + 1),
                    None => map.insert(point, 1),
                };
            }
        }
    }
    map.iter().filter(|(_, val)| **val > 1).count() as u64
}

fn part_two(inp: &String) -> u64 {
    let mut map = HashMap::new();
    let lines = inp.lines();
    for line in lines {
        let mut parts = line.split(' ');
        let p1 = Point::from_iter(&mut parts.next().unwrap().split(',').clone());
        parts.next(); // get rid of arrow
        let p2 = Point::from_iter(&mut parts.next().unwrap().split(',').clone());
        let line = Line::new(p1, p2);
        for point in line.generate_points() {
            let _ = match map.get(&point) {
                Some(val) => map.insert(point, val + 1),
                None => map.insert(point, 1),
            };
        }
    }
    map.iter().filter(|(_, val)| **val > 1).count() as u64
}
