use crate::advent_of_code::AdventOfCodeInput;
#[derive(Clone, Copy)]
struct BitArray {
    arr: u64,
}

impl BitArray {
    fn index(&self, index: usize) -> u64 {
        (self.arr >> index) & 1
    }
    fn set(&mut self, index: u64, value: bool) {
        if index == 0 {
            if value {
                self.arr += 1;
            }
            return;
        }
        if value {
            self.arr = self.arr | (2u64.pow((index - 1) as u32) << 1)
        } else {
            let mask: u64 = 0xFFFFFFFFFFFFFFFF - (index << 1);
            self.arr &= mask;
        }
    }
}

impl std::fmt::Debug for BitArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:0>5b}", self.arr)
    }
}

pub fn solve(aoc_input: AdventOfCodeInput) {
    let codes: Vec<BitArray> = aoc_input
        .inp
        .lines()
        .map(|x| BitArray {
            arr: u64::from_str_radix(x, 2).unwrap(),
        })
        .collect();
    let pt1 = part_one(&codes);
    let pt2 = part_two(codes);
    println!("Day 3: ({},{})", pt1, pt2);
}

fn most_common(numbers: &Vec<BitArray>, i: u64) -> bool {
    let common_min = (numbers.len() / 2) as u64;
    numbers.iter().map(|x| x.index(i as usize)).sum::<u64>() > common_min
}
fn get_ones_count(numbers: &Vec<BitArray>, i: u64) -> (u64, u64) {
    let ones = numbers.iter().map(|x| x.index(i as usize)).sum::<u64>();
    let zeros = numbers.len() as u64 - ones;
    (ones, zeros)
}

fn part_one(codes: &Vec<BitArray>) -> u64 {
    let mut gamma = BitArray { arr: 0 };
    let mut epsilon = BitArray { arr: 0 };
    let common_min = (codes.len() / 2) as u64;
    for i in (0..12).rev() {
        if most_common(codes, i) {
            gamma.set(i as u64, true);
            epsilon.set(i as u64, false);
        } else {
            gamma.set(i as u64, false);
            epsilon.set(i as u64, true);
        }
    }
    gamma.arr * epsilon.arr
}

fn part_two(_codes: Vec<BitArray>) -> u64 {
    let mut o2_codes = _codes.clone();
    for i in (0..12).rev() {
        let (ones, zeros) = get_ones_count(&o2_codes, i);
        let num_to_match = if ones > zeros || ones == zeros { 1 } else { 0 };
        o2_codes = o2_codes
            .into_iter()
            .filter(|x| x.index(i as usize) == num_to_match)
            .collect();
        if o2_codes.len() == 1 {
            break;
        }
    }
    let mut co2_codes = _codes.clone();
    for i in (0..12).rev() {
        let (ones, zeros) = get_ones_count(&o2_codes, i);
        let num_to_match = if ones > zeros || ones == zeros { 0 } else { 1 };
        co2_codes = co2_codes
            .into_iter()
            .filter(|x| x.index(i as usize) == num_to_match)
            .collect();
        if co2_codes.len() == 1 {
            break;
        }
    }
    let o2 = o2_codes.iter().next().unwrap();
    let co2 = co2_codes.iter().next().unwrap();
    println!("{}, {}", o2.arr, co2.arr);
    o2.arr * co2.arr
}
