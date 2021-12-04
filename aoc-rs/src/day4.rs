use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use itertools::Itertools;
use log_derive::logfn;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub struct BingoBoard([[u8; 5]; 5]);

impl BingoBoard {
    pub fn new(lines: &[&str]) -> Self {
        let mut board = [[0; 5]; 5];
        for (i, line) in lines.iter().enumerate() {
            for (j, c) in line
                .split(' ')
                .filter(|c| !c.contains(" ") && !c.is_empty())
                .enumerate()
            {
                board[i][j] = u8::from_str(c).unwrap();
            }
        }
        BingoBoard(board)
    }

    pub fn get_column(&self, i: usize) -> Vec<u8> {
        self.0.iter().map(|row| row[i]).collect()
    }

    #[logfn(Trace)]
    pub fn assign_number(&mut self, number: u8) {
        if let Some((position, _)) = self.0.iter().flatten().find_position(|&&n| n == number) {
            let row = position / 5;
            let col = position % 5;

            self.0[row][col] = u8::MAX;
        }
    }

    #[logfn(Trace)]
    pub fn has_won(&self) -> bool {
        // // Check for bingo in column
        // for i in 0..5 {
        //     if self.0[0][i] == u8::MAX
        //         && self.0[1][i] == u8::MAX
        //         && self.0[2][i] == u8::MAX
        //         && self.0[3][i] == u8::MAX
        //         && self.0[4][i] == u8::MAX
        //     {
        //         return true;
        //     }
        // }

        // Columns
        for i in 0..5 {
            if self.get_column(i).iter().all(|&n| n == u8::MAX) {
                return true;
            }
        }

        // Row
        if self.0.iter().any(|row| row.iter().all(|&c| c == u8::MAX)) {
            return true;
        }

        false
    }

    pub fn unmarked_number_sum(&self) -> usize {
        self.0
            .iter()
            .flatten()
            .map(|&n| n as usize)
            .filter(|&n| n != u8::MAX.into())
            .sum()
    }
}

#[aoc_generator(day4)]
fn parse_input(input: &str) -> (Vec<u8>, Vec<BingoBoard>) {
    let drawn_numbers = input
        .lines()
        .next()
        .map(|line| line.split(',').map(|s| u8::from_str(s).unwrap()).collect())
        .unwrap();

    let boards = input
        .lines()
        .skip(1)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .chunks(5)
        .map(|chunk| BingoBoard::new(chunk))
        .collect::<Vec<_>>();

    (drawn_numbers, boards)
}

fn find_winner(drawn_numbers: Vec<u8>, mut boards: Vec<BingoBoard>) -> Option<(u8, BingoBoard)> {
    for number in drawn_numbers {
        for board in boards.iter_mut() {
            board.assign_number(number);

            if board.has_won() {
                return Some((number, *board));
            }
        }
    }

    None
}

#[aoc(day4, part1)]
fn solve_part_1(input: &(Vec<u8>, Vec<BingoBoard>)) -> u32 {
    let (drawn_numbers, mut boards) = input.clone();

    // Go through all the drawn numbers
    //
    let (last_num, winner) = find_winner(drawn_numbers.clone(), boards.clone()).unwrap();

    (last_num as usize * winner.unmarked_number_sum()) as u32
}

#[aoc(day4, part2)]
fn solve_part_2(input: &(Vec<u8>, Vec<BingoBoard>)) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::LevelFilter;
    use simple_logger::SimpleLogger;

    #[test]
    fn test_example() {
        SimpleLogger::new()
            .with_level(LevelFilter::Trace)
            .init()
            .unwrap();

        let (drawn_numbers, boards) = parse_input(include_str!("../input/2021/day4_example.txt"));
        let (last_num, winner) = find_winner(drawn_numbers, boards).unwrap();

        assert_eq!(last_num, 24);
        assert_eq!(winner.unmarked_number_sum(), 188);
    }

    #[test]
    fn test_board() {
        let board =
            "22 13 17 11  0\n 8  2 23  4 24\n21  9 14 16  7\n 6 10  3 18  5\n 1 12 20 15 19";
        let lines = board.lines().collect::<Vec<_>>();
        let mut board = BingoBoard::new(&lines);

        board.assign_number(16);

        assert_eq!(board.get_column(0), vec![22, 8, 21, 6, 1]);
        assert_eq!(board.0[2][3], u8::MAX);
    }
}
