use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

#[derive(Copy, Clone, Default)]
pub struct BingoBoard([[u8; 5]; 5]);

/// The value that will be used to mark cells of numbers on the bingo board that
/// have been drawn.
pub const MARKER: u8 = u8::MAX;

impl BingoBoard {
    pub fn new(lines: &[&str]) -> Self {
        let mut board = [[0; 5]; 5];

        assert_eq!(lines.len(), 5);
        for (i, line) in lines.iter().enumerate() {
            for (j, c) in line.split_whitespace().enumerate() {
                board[i][j] = u8::from_str(c).unwrap();
            }
        }

        BingoBoard(board)
    }

    pub fn get_column(&self, i: usize) -> Vec<u8> { self.0.iter().map(|row| row[i]).collect() }

    /// Marks the cell that contains the specified number. If the number is not
    /// defined, nothing will happen.
    pub fn assign_number(&mut self, number: u8) {
        if let Some((position, _)) = self.0.iter().flatten().find_position(|&&n| n == number) {
            let row = position / 5;
            let col = position % 5;

            self.0[row][col] = MARKER;
        }
    }

    pub fn has_won(&self) -> bool {
        let column = (0..5).any(|i| self.get_column(i).iter().all(|&n| n == MARKER));
        let row = self.0.iter().any(|row| row.iter().all(|&c| c == MARKER));

        column || row
    }

    /// Sum of all cells that have not been marked.
    pub fn unmarked_number_sum(&self) -> usize {
        self.0
            .iter()
            .flatten()
            .map(|&n| n as usize)
            .filter(|&n| n != MARKER.into())
            .sum()
    }

    /// The maximum number of cells that have already been marked (either per
    /// row or column)
    pub fn max_cells_marked(&self) -> usize {
        let col_marks = (0..5)
            .map(|i| self.get_column(i).iter().filter(|&&n| n == MARKER).count())
            .max()
            .unwrap_or_default();

        let row_marks = self
            .0
            .iter()
            .map(|row| row.iter().filter(|&&c| c == MARKER).count())
            .max()
            .unwrap_or_default();

        if col_marks > row_marks {
            col_marks
        } else {
            row_marks
        }
    }
}

impl Display for BingoBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.0 {
            writeln!(f, "{:?}", row)?;
        }

        Ok(())
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
        .map(BingoBoard::new)
        .collect::<Vec<_>>();

    (drawn_numbers, boards)
}

fn find_winner(drawn_numbers: &[u8], boards: &mut [BingoBoard]) -> Option<(u8, BingoBoard)> {
    for number in drawn_numbers {
        for board in boards.iter_mut() {
            board.assign_number(*number);

            if board.has_won() {
                return Some((*number, *board));
            }
        }
    }

    None
}

fn find_last_board(drawn_numbers: Vec<u8>, mut boards: Vec<BingoBoard>) -> Option<(u8, BingoBoard)> {
    for number in drawn_numbers {
        // If we found the last board, return it.
        //
        if let [mut board] = *boards {
            board.assign_number(number);

            if board.has_won() {
                return Some((number, board));
            } else {
                continue;
            }
        }

        // Otherwise, assign the number and filter winning boards.
        //
        boards = boards
            .into_iter()
            .map(|mut board| {
                board.assign_number(number);
                board
            })
            .filter(|b| !b.has_won())
            .collect();
    }

    None
}

#[aoc(day4, part1)]
fn solve_part_1(input: &(Vec<u8>, Vec<BingoBoard>)) -> u32 {
    let (drawn_numbers, mut boards) = input.clone();

    let (last_num, winner) = find_winner(drawn_numbers.as_slice(), boards.as_mut_slice()).unwrap();

    (last_num as usize * winner.unmarked_number_sum()) as u32
}

#[aoc(day4, part2)]
fn solve_part_2(input: &(Vec<u8>, Vec<BingoBoard>)) -> u32 {
    let (drawn_numbers, boards) = input.clone();

    let (last_num, last_board) = find_last_board(drawn_numbers, boards).unwrap();

    (last_num as usize * last_board.unmarked_number_sum()) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board() {
        let board = "22 13 17 11  0\n 8  2 23  4 24\n21  9 14 16  7\n 6 10  3 18  5\n 1 12 20 15 19";
        let lines = board.lines().collect::<Vec<_>>();
        let mut board = BingoBoard::new(&lines);

        board.assign_number(16);

        assert_eq!(board.get_column(0), vec![22, 8, 21, 6, 1]);
        assert_eq!(board.0[2][3], MARKER);
    }

    #[test]
    fn test_max_unmarked_cells() {
        let (drawn_numbers, mut boards) = parse_input(include_str!("../input/2021/day4_example.txt"));
        let _ = find_winner(drawn_numbers.as_slice(), boards.as_mut_slice());

        assert_eq!(boards[0].max_cells_marked(), 4);
        assert_eq!(boards[1].max_cells_marked(), 3);
        assert_eq!(boards[2].max_cells_marked(), 5);
    }

    #[test]
    fn test_find_winner() {
        let (drawn_numbers, mut boards) = parse_input(include_str!("../input/2021/day4_example.txt"));
        let (last_num, winner) = find_winner(drawn_numbers.as_slice(), boards.as_mut_slice()).unwrap();

        assert_eq!(last_num, 24);
        assert_eq!(winner.unmarked_number_sum(), 188);
    }

    #[test]
    fn test_example() {
        let (drawn_numbers, boards) = parse_input(include_str!("../input/2021/day4_example.txt"));
        let part_1 = solve_part_1(&(drawn_numbers, boards));

        let (drawn_numbers, boards) = parse_input(include_str!("../input/2021/day4_example.txt"));
        let part_2 = solve_part_2(&(drawn_numbers, boards));

        assert_eq!(part_1, 4512);
        assert_eq!(part_2, 1924);
    }

    #[test]
    fn test_real() {
        let (drawn_numbers, boards) = parse_input(include_str!("../input/2021/day4.txt"));
        let part_1 = solve_part_1(&(drawn_numbers, boards));

        let (drawn_numbers, boards) = parse_input(include_str!("../input/2021/day4.txt"));
        let part_2 = solve_part_2(&(drawn_numbers, boards));

        assert_eq!(part_1, 27027);
        assert_eq!(part_2, 36975);
    }
}
