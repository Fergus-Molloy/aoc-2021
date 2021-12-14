use crate::advent_of_code::AdventOfCodeInput;
use rustc_hash::FxHashSet;
#[derive(Clone)]
pub struct HeightMap {
    map: Vec<Vec<usize>>,
    size: Point,
}

impl HeightMap {
    const fn max_y(&self) -> usize {
        self.size.y
    }
    const fn max_x(&self) -> usize {
        self.size.x
    }

    /// # Panics
    #[must_use]
    pub fn new(inp: &str) -> Self {
        let map: Vec<Vec<usize>> = inp
            .lines()
            .map(|x| {
                x.chars()
                    .map(|c| c.to_string().parse::<usize>().unwrap())
                    .collect::<Vec<usize>>()
            })
            .collect();
        HeightMap {
            map: map.clone(),
            size: Point {
                x: map[0].len(),
                y: map.len(),
            },
        }
    }

    fn get_adjacent_points(&self, point: Point) -> Vec<Point> {
        let mut points = Vec::with_capacity(4);
        if point.y > 0 {
            // check above
            points.push(Point::new(point.x, point.y - 1));
        }
        if point.y < self.map.len() - 1 {
            //check below
            points.push(Point::new(point.x, point.y + 1));
        }
        if point.x > 0 {
            //check left
            points.push(Point::new(point.x - 1, point.y));
        }
        if point.x < self.map[0].len() - 1 {
            //check right
            points.push(Point::new(point.x + 1, point.y));
        }
        points
    }

    fn is_lowest(&self, point: Point) -> bool {
        let h = self.get(&point);
        let values_to_check = self.get_adjacent_points(point);
        values_to_check.iter().all(|x| self.get(x) > h)
    }

    fn get(&self, point: &Point) -> usize {
        self.map[point.y][point.x]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}

pub fn solve(aoc_input: AdventOfCodeInput) -> String {
    let hmap = HeightMap::new(&aoc_input.inp);
    let pt1 = part_one(&hmap);
    let pt2 = part_two(&hmap);
    format!("Day 9: ({},{})", pt1, pt2)
}

fn get_lowest(hmap: &HeightMap) -> FxHashSet<Point> {
    let mut lowest = FxHashSet::default();
    for y in 0..hmap.max_y() {
        for x in 0..hmap.max_x() {
            let p = Point::new(x, y);
            if hmap.is_lowest(p) {
                lowest.insert(p);
            }
        }
    }
    lowest
}

pub fn part_one(hmap: &HeightMap) -> u64 {
    let lowest = get_lowest(hmap);
    lowest.iter().map(|x| (hmap.get(x) + 1) as u64).sum::<u64>()
}

pub fn part_two(hmap: &HeightMap) -> u64 {
    let lowest = get_lowest(hmap);
    let mut sizes: Vec<u64> = Vec::with_capacity(lowest.len());
    for point in lowest {
        let mut points = FxHashSet::default();
        points.insert(point);
        let mut added = 1;
        while added != 0 {
            added = 0;
            let mut new_points = points.clone();
            for p in &points {
                let adj = hmap.get_adjacent_points(*p);
                let to_add = adj.into_iter().filter(|x| hmap.get(x) != 9);
                for new_p in to_add {
                    if new_points.insert(new_p) {
                        added += 1;
                    }
                }
            }
            points = new_points;
        }
        let size = points.len() as u64;
        sizes.push(size);
    }
    sizes.sort_unstable();
    let top_3 = sizes.iter().rev().take(3);
    let mut total = 1;
    for item in top_3 {
        total *= item;
    }
    total
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::advent_of_code::AdventOfCodeInput;
    #[test]
    fn d9a() {
        let aoc_input = AdventOfCodeInput::get_input(9);
        let hmap = HeightMap::new(&aoc_input.inp);
        assert_eq!(part_one(&hmap), 444);
    }

    #[test]
    fn d9b() {
        let aoc_input = AdventOfCodeInput::get_input(9);
        let hmap = HeightMap::new(&aoc_input.inp);
        assert_eq!(part_two(&hmap), 1168440);
    }
}
