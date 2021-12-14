use crate::advent_of_code::AdventOfCodeInput;
use rustc_hash::FxHashMap;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct Node {
    name: char,
    is_small: bool,
    visited: u32,
    neighbors: Vec<char>,
}

impl Node {
    fn new(from: char, to: &str) -> Self {
        if to.len() > 1 {
            Self {
                name: from,
                is_small: from.is_lowercase(),
                visited: 0,
                neighbors: vec!['!'],
            }
        } else if to.is_empty() {
            Self {
                name: from,
                is_small: from.is_lowercase(),
                visited: 0,
                neighbors: Vec::new(),
            }
        } else {
            Self {
                name: from,
                is_small: from.is_lowercase(),
                visited: 0,
                neighbors: vec![to.chars().next().unwrap()],
            }
        }
    }

    fn add_neighbor(&mut self, to: &str) {
        let n = if to.len() > 1 {
            '!'
        } else {
            to.chars().next().unwrap()
        };
        self.neighbors.push(n);
    }
}

pub fn parse(inp: String) -> FxHashMap<char, Node> {
    let mut map: FxHashMap<char, Node> = FxHashMap::default();
    for line in inp.lines() {
        let mut s = line.split('-');
        let from = s.next().unwrap();
        let to = s.next().unwrap();
        let from_c = from.chars().next().unwrap();
        // if from == start
        if from.len() > 1 {
            // if start node exists
            match map.get_mut(&'*') {
                Some(node) => node.add_neighbor(to), // add new neighbors
                None => {
                    // create start node
                    map.insert('*', Node::new('*', to));
                }
            }
        } else {
            match map.get_mut(&from_c) {
                Some(node) => node.add_neighbor(to),
                None => {
                    map.insert(from_c, Node::new(from_c, to));
                }
            }
        }
    }
    // add end node
    map.insert('!', Node::new('!', ""));
    map
}

pub fn solve(aoc_input: AdventOfCodeInput) -> String {
    let mut map = parse(aoc_input.inp);
    for (_, val) in &map {
        println!("{:?}", val);
    }
    let pt1 = part_one(&mut map.clone());
    let pt2 = part_two(&map);
    format!("Day 1: ({},{})", pt1, pt2)
}

//fn search(queue: &mut Vec<char>, map: &mut FxHashMap<char, Node>) -> u32 {
//    let top = match queue.iter().next() {
//        Some(val) => val,
//        None => return 0,
//    };
//    println!("Searching {}", top);
//    let node = match map.get_mut(top) {
//        Some(v) => v,
//        None => return 0,
//    };
//    node.visited += 1;
//    let mut add: Vec<char> = node
//        .neighbors
//        .clone()
//        .into_iter()
//        .filter_map(|c| {
//            let node = map.get(&c).unwrap();
//            if (node.is_small && node.visited < 1) || !node.is_small {
//                Some(c)
//            } else {
//                None
//            }
//        })
//        .collect();
//    if node.name == '!' {
//        0
//    } else {
//        queue.append(&mut node.neighbors);
//        node.neighbors.len() as u32 + search(queue, map)
//    }
//}

pub fn part_one(map: &mut FxHashMap<char, Node>) -> u32 {
    let mut queue = Vec::new();
    let start = map.get(&'*').unwrap();
    for n in start.neighbors.clone() {
        queue.push(n);
    }
    //search(&mut queue, map)
    0
}

pub fn part_two(depths: &FxHashMap<char, Node>) -> u32 {
    0
}

#[cfg(test)]
mod test {
    use super::{part_one, part_two};
    use crate::advent_of_code::AdventOfCodeInput;
    #[test]
    fn d12a() {
        let aoc_input = AdventOfCodeInput::get_input(1);
        let depths: Vec<_> = aoc_input
            .inp
            .lines()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        assert_eq!(part_one(&depths), 1722);
    }
    #[test]
    fn d12b() {
        let aoc_input = AdventOfCodeInput::get_input(1);
        let depths: Vec<_> = aoc_input
            .inp
            .lines()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        assert_eq!(part_two(&depths), 1748);
    }
}
