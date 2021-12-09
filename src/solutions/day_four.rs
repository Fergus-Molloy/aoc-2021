use crate::advent_of_code::AdventOfCodeInput;
#[derive(Debug, Clone)]
pub struct BingoBoard {
    board: Vec<BingoNum>,
    size: (u64, u64),
    won: bool,
}
#[derive(Debug, Clone, Copy)]
pub struct BingoNum {
    num: u64,
    pos: Position,
    called: bool,
}
#[derive(Debug, Clone, Copy)]
pub struct Position {
    x: u64,
    y: u64,
}

impl BingoBoard {
    fn new() -> Self {
        BingoBoard {
            board: Vec::new(),
            size: (5, 5),
            won: false,
        }
    }

    fn call_num(&mut self, num: u64) {
        self.board
            .iter_mut()
            .filter(|bingo_num| bingo_num.num == num)
            .for_each(|bingo_num| {
                bingo_num.called = true;
            });
    }

    fn col_complete(&self, col: u64) -> bool {
        self.board
            .iter()
            .filter(|x| x.pos.x == col)
            .all(|x| x.called)
    }

    fn row_complete(&self, row: u64) -> bool {
        self.board
            .iter()
            .filter(|x| x.pos.y == row)
            .all(|x| x.called)
    }

    fn won(&self) -> bool {
        for i in 0..self.size.0 {
            if self.col_complete(i) {
                return true;
            }
        }
        for i in 0..self.size.1 {
            if self.row_complete(i) {
                return true;
            }
        }
        return false;
    }

    fn sum_uncalled(&self) -> u64 {
        self.board
            .iter()
            .filter(|x| !x.called)
            .map(|x| x.num)
            .sum::<u64>()
    }
}

pub fn parse(aoc_input: AdventOfCodeInput) -> (Vec<BingoBoard>, Vec<u64>) {
    let mut lines = aoc_input.inp.lines();
    let numbers: Vec<u64> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u64>().expect("could not parse input"))
        .collect();
    lines.next(); // remove empty line
    let mut boards = Vec::new();
    let mut curr_board = BingoBoard::new();
    let mut row = 0;
    for line in lines {
        if line.trim().is_empty() {
            boards.push(curr_board.clone());
            curr_board = BingoBoard::new();
            row = 0;
        } else {
            let mut rows: Vec<BingoNum> = line
                .split(' ')
                .filter(|x| !x.trim().is_empty())
                .enumerate()
                .map(|(col, x)| BingoNum {
                    num: x.parse::<u64>().unwrap(),
                    called: false,
                    pos: Position {
                        x: col as u64,
                        y: row,
                    },
                })
                .collect();
            curr_board.board.append(&mut rows);
            row += 1;
        }
    }
    boards.push(curr_board.clone());
    (boards, numbers)
}

pub fn solve(aoc_input: AdventOfCodeInput) -> String {
    let (mut boards, numbers) = parse(aoc_input);

    let pt1 = part_one(&numbers, &mut boards.clone());
    let pt2 = part_two(&numbers, &mut boards);
    format!("Day 4: ({},{})", pt1, pt2)
}

pub fn part_one(numbers: &Vec<u64>, boards: &mut Vec<BingoBoard>) -> u64 {
    for num in numbers {
        for board in boards.iter_mut() {
            board.call_num(*num);
        }
        if boards.iter().any(|x| x.won()) {
            return boards
                .iter()
                .filter(|x| x.won())
                .map(|x| x.sum_uncalled())
                .max()
                .unwrap()
                * num;
        }
    }
    panic!("No boards one even after all numbers were called");
}

pub fn part_two(numbers: &Vec<u64>, boards: &mut Vec<BingoBoard>) -> u64 {
    // flags for last board
    let mut last_board = false;
    let mut num_for_final = 0;
    for (i, num) in numbers.iter().enumerate() {
        for board in boards.iter_mut() {
            board.call_num(*num);
            board.won = board.won();
        }
        if boards.iter().filter(|x| !x.won).count() == 1 {
            last_board = true;
            num_for_final = i + 1;
            break;
        }
    }
    if !last_board {
        panic!("No boards one even after all numbers were called");
    }
    let mut last_board = boards.iter_mut().filter(|x| !x.won).next().unwrap();
    while !last_board.won {
        last_board.call_num(numbers[num_for_final]);
        last_board.won = last_board.won();
        num_for_final += 1;
    }
    last_board.sum_uncalled() * numbers[num_for_final - 1]
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::advent_of_code::AdventOfCodeInput;
    fn parse(aoc_input: AdventOfCodeInput) -> (Vec<BingoBoard>, Vec<u64>) {
        let mut lines = aoc_input.inp.lines();
        let numbers: Vec<u64> = lines
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<u64>().expect("could not parse input"))
            .collect();
        lines.next(); // remove empty line
        let mut boards = Vec::new();
        let mut curr_board = BingoBoard::new();
        let mut row = 0;
        for line in lines {
            if line.trim().is_empty() {
                boards.push(curr_board.clone());
                curr_board = BingoBoard::new();
                row = 0;
            } else {
                let mut rows: Vec<BingoNum> = line
                    .split(' ')
                    .filter(|x| !x.trim().is_empty())
                    .enumerate()
                    .map(|(col, x)| BingoNum {
                        num: x.parse::<u64>().unwrap(),
                        called: false,
                        pos: Position {
                            x: col as u64,
                            y: row,
                        },
                    })
                    .collect();
                curr_board.board.append(&mut rows);
                row += 1;
            }
        }
        boards.push(curr_board.clone());
        (boards, numbers)
    }
    #[test]
    fn d4a() {
        let aoc_input = AdventOfCodeInput::get_input(4);
        let (mut boards, numbers) = parse(aoc_input);
        assert_eq!(part_one(&numbers, &mut boards), 60368);
    }
    #[test]
    fn d4b() {
        let aoc_input = AdventOfCodeInput::get_input(4);
        let (mut boards, numbers) = parse(aoc_input);
        assert_eq!(part_two(&numbers, &mut boards), 17435);
    }
}
