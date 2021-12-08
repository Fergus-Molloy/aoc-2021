use crate::advent_of_code::AdventOfCodeInput;
#[derive(Clone, Copy)]
pub struct BitArray {
    pub arr: u64,
}

impl BitArray {
    fn index(&self, index: usize) -> u64 {
        (self.arr >> index) & 1
    }
}

impl std::fmt::Debug for BitArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:0>5b}", self.arr)
    }
}

pub fn solve(aoc_input: AdventOfCodeInput) -> String {
    let codes: Vec<BitArray> = aoc_input
        .inp
        .lines()
        .map(|x| BitArray {
            arr: u64::from_str_radix(x, 2).unwrap(),
        })
        .collect();
    let pt1 = part_one(&codes);
    let pt2 = part_two(codes);
    format!("Day 3: ({},{})", pt1, pt2)
}

fn get_ones_count(numbers: &Vec<BitArray>, i: u64) -> (u64, u64) {
    let ones = numbers.iter().map(|x| x.index(i as usize)).sum::<u64>();
    let zeros = numbers.len() as u64 - ones;
    (ones, zeros)
}

pub fn part_one(codes: &Vec<BitArray>) -> u64 {
    let mut g = String::new();
    let mut e = String::new();
    for i in (0..12).rev() {
        let (ones, zeros) = get_ones_count(codes, i);
        if ones > zeros {
            g.push('1');
            e.push('0');
        } else {
            g.push('0');
            e.push('1');
        }
    }
    let ga = u64::from_str_radix(&g, 2).unwrap();
    let ea = u64::from_str_radix(&e, 2).unwrap();
    ga * ea
}

pub fn part_two(_codes: Vec<BitArray>) -> u64 {
    let mut o2_codes = _codes.clone();
    for i in (0..12).rev() {
        let (ones, zeros) = get_ones_count(&o2_codes, i);
        let num_to_match = if ones >= zeros { 1 } else { 0 };
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
        let (ones, zeros) = get_ones_count(&co2_codes, i);
        let num_to_match = if ones < zeros { 1 } else { 0 };
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
    o2.arr * co2.arr
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::advent_of_code::AdventOfCodeInput;
    #[test]
    fn d3a() {
        let aoc_input = AdventOfCodeInput::get_input(3);
        let codes: Vec<BitArray> = aoc_input
            .inp
            .lines()
            .map(|x| BitArray {
                arr: u64::from_str_radix(x, 2).unwrap(),
            })
            .collect();
        assert_eq!(part_one(&codes), 775304);
    }
    #[test]
    fn d3b() {
        let aoc_input = AdventOfCodeInput::get_input(3);
        let codes: Vec<BitArray> = aoc_input
            .inp
            .lines()
            .map(|x| BitArray {
                arr: u64::from_str_radix(x, 2).unwrap(),
            })
            .collect();
        assert_eq!(part_two(codes), 1370737);
    }
}
