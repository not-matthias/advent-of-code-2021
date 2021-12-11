use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::{collections::HashMap, str::FromStr};

pub const THRESHOLD: u8 = 9;

#[derive(Copy, Clone, Debug)]
pub struct Octopus {
    energy: u8,
    flashed: bool,
}

impl Octopus {
    pub const fn new(energy: u8) -> Self {
        Self {
            energy,
            flashed: false,
        }
    }

    pub fn charge(&mut self) { self.energy += 1; }
}

pub struct Board {
    fields: Vec<Vec<Octopus>>,
}

impl Board {
    pub fn new(fields: Vec<Vec<Octopus>>) -> Self { Board { fields } }

    pub fn is_out(&self, pos: (usize, usize)) -> bool {
        let (x, y) = pos;

        // x < 0 || x >= self.fields.len() || y < 0 || y >= self.fields[0].len()
        x >= self.fields.len() || y >= self.fields[0].len()
    }

    pub fn charge(&mut self, pos: (usize, usize)) {
        if self.is_out(pos) {
            println!("Is out: {:?}", pos);
            return;
        }

        let (x, y) = pos;
        self.fields[x][y].charge();
    }

    fn neighbours(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let (x, y) = pos;

        // vec![
        //     (x - 1, y - 1),
        //     (x - 1, y),
        //     (x - 1, y + 1),
        //     (x, y - 1),
        //     (x, y + 1),
        //     (x + 1, y - 1),
        //     (x + 1, y),
        //     (x + 1, y + 1),
        // ]

        (-1..=1)
            .flat_map(|dx| (-1..=1).map(move |dy| (dx, dy)))
            .filter_map(|(dx, dy)| {
                let x = x as i32 + dx;
                let y = y as i32 + dy;
                if x >= 0
                    && x < self.fields.len() as i32
                    && y >= 0
                    && y < self.fields[0].len() as i32
                {
                    Some((x as usize, y as usize))
                } else {
                    None
                }
            })
            .filter(|&(x, y)| !(x == pos.0 && y == pos.1))
            .collect_vec()
    }

    pub fn flash(&mut self, pos: (usize, usize)) {
        if self.is_out(pos) {
            return;
        }

        let (x, y) = pos;
        let mut octopus = &mut self.fields[x][y];
        if octopus.energy <= 9 || octopus.flashed {
            return;
        }
        octopus.flashed = true;

        for pos in self.neighbours(pos) {
            self.charge(pos);
            self.flash(pos);
        }
    }

    pub fn print(&self) {
        println!("\n[Board]");
        for x in 0..self.fields.len() {
            for y in 0..self.fields[0].len() {
                let octopus = &self.fields[x][y];
                print!("{}", octopus.energy);
            }
            println!();
        }
    }
}

fn step(board: &mut Board) -> u32 {
    let mut flashes = 0;

    for octopus in board.fields.iter_mut().flatten() {
        octopus.charge();
    }

    for x in 0..board.fields.len() {
        for y in 0..board.fields[0].len() {
            board.flash((x, y));
        }
    }

    for octopus in board.fields.iter_mut().flatten() {
        if octopus.flashed {
            octopus.energy = 0;
            flashes += 1;
        }
        octopus.flashed = false;
    }

    flashes
}

#[aoc_generator(day11)]
fn parse_input(input: &str) -> Vec<Vec<Octopus>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10).map(|c| Octopus::new(c as u8)))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

#[aoc(day11, part1)]
fn solve_part_1(input: &[Vec<Octopus>]) -> u32 {
    let mut input = input.to_vec();
    let mut board = Board::new(input.clone());

    let mut flashes = 0;
    for _ in 0..100 {
        board.print();

        flashes += step(&mut board);
    }

    flashes
}

#[aoc(day11, part2)]
fn solve_part_2(input: &[Vec<Octopus>]) -> u32 {
    //
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neighbours() {
        let input = "11111\n19991\n19191\n19991\n11111";
        let input = parse_input(input);
        let board = Board::new(input.clone());

        assert_eq!(board.neighbours((0, 0)), [(0, 1), (1, 0), (1, 1)]);
        assert_eq!(
            board.neighbours((2, 2)),
            [
                (1, 1),
                (1, 2),
                (1, 3),
                (2, 1),
                (2, 3),
                (3, 1),
                (3, 2),
                (3, 3)
            ]
        );
    }

    #[test]
    fn test_simple() {
        let input = "11111\n19991\n19191\n19991\n11111";
        let input = parse_input(input);

        assert_eq!(solve_part_1(&input), 0);
        assert_eq!(solve_part_2(&input), 0);
    }

    fn get_input() -> &'static str {
        "5483143223\n2745854711\n5264556173\n6141336146\n6357385478\\
         n4167524645\n2176841721\n6882881134\n4846848554\n5283751526"
    }

    #[test]
    fn test_example() {
        let input = get_input();
        let input = parse_input(input);

        assert_eq!(solve_part_1(&input), 1656);
        assert_eq!(solve_part_2(&input), 0);
    }
}
