use crate::advent_of_code::AdventOfCodeInput;
use std::collections::HashSet;
use std::fmt::Formatter;
#[derive(Debug, Clone)]
struct HydroFeild {
    feild: Vec<Line>,
}
#[derive(Debug, Clone)]
struct Line {
    start: Point,
    end: Point,
    points: HashSet<Point>,
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
        let mut points = HashSet::new();
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
                points.insert(start);
                points.insert(end);
            } else {
                for i in y_range {
                    points.insert(Point::new(start.x, i));
                }
            }
        } else if start.y == end.y {
            let len = ((start.x as i64 - end.x as i64).pow(2) as f64).sqrt();
            if len < 2.0 {
                points.insert(start);
                points.insert(end);
            } else {
                for i in x_range {
                    points.insert(Point::new(i, start.y));
                }
            }
        } else {
            for (x, y) in x_range.iter().zip(y_range) {
                points.insert(Point::new(*x, y));
            }
        }
        Line { start, end, points }
    }

    fn is_diagonal(&self) -> bool {
        !(self.start.x == self.end.x || self.start.y == self.end.y)
    }
}

pub fn solve(aoc_input: AdventOfCodeInput) {
    let mut feild = HydroFeild {
        feild: aoc_input
            .inp
            .lines()
            .map(|line| {
                let mut parts = line.split(' ');
                let p1 = Point::from_iter(&mut parts.next().unwrap().split(',').clone());
                parts.next(); // get rid of arrow
                let p2 = Point::from_iter(&mut parts.next().unwrap().split(',').clone());
                Line::new(p1, p2)
            })
            .collect(),
    };
    let pt1 = 0; // part_one(&feild);
    let pt2 = part_two(&feild);
    println!("Day 5: ({},{})", pt1, pt2);
}

fn part_one(feild: &HydroFeild) -> u64 {
    let mut count = 0;
    let mut counted: HashSet<Point> = HashSet::new();
    for line in feild.feild.clone().into_iter().filter(|x| !x.is_diagonal()) {
        for point in line.points {
            let other: Vec<Point> = feild
                .feild
                .iter()
                .filter(|x| !x.is_diagonal())
                .map(|line| {
                    line.points
                        .clone()
                        .into_iter()
                        .filter(|other_point| !counted.contains(&other_point))
                        .filter(|&other_point| point == other_point)
                        .collect::<Vec<Point>>()
                })
                .flatten()
                .collect();
            if other.len() > 1 {
                let cloned = other.iter().next().unwrap();
                counted.insert(*cloned);
                count += 1;
            }
        }
    }
    count
}

fn part_two(feild: &HydroFeild) -> u64 {
    let mut count = 0;
    let mut counted: HashSet<Point> = HashSet::new();
    for line in feild.feild.clone().into_iter() {
        for point in line.points {
            let other: Vec<Point> = feild
                .feild
                .iter()
                .map(|line| {
                    line.points
                        .clone()
                        .into_iter()
                        .filter(|other_point| !counted.contains(&other_point))
                        .filter(|&other_point| point == other_point)
                        .collect::<Vec<Point>>()
                })
                .flatten()
                .collect();
            if other.len() > 1 {
                let cloned = other.iter().next().unwrap();
                counted.insert(*cloned);
                count += 1;
            }
        }
    }
    count
}
