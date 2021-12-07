use aoc_runner_derive::{aoc, aoc_generator};
use rayon::prelude::*;
use std::str::FromStr;

#[aoc_generator(day7)]
fn parse_input(input: &str) -> Vec<u16> {
    input
        .lines()
        .map(|line| line.split(',').filter_map(|part| u16::from_str(part).ok()))
        .flatten()
        .collect::<Vec<_>>()
}

#[aoc(day7, part1)]
fn solve_part_1(input: &[u16]) -> u16 {
    let min = input.iter().min().unwrap();
    let max = input.iter().max().unwrap();

    (*min..*max)
        .into_par_iter()
        .map(|i| input.iter().map(|input| i.abs_diff(*input)).sum::<u16>())
        .min()
        .unwrap()
}

#[aoc(day7, part2)]
fn solve_part_2(input: &[u16]) -> u16 {
    let min = input.iter().min().unwrap();
    let max = input.iter().max().unwrap();

    (*min..*max)
        .into_par_iter()
        .map(|i| {
            input
                .into_par_iter()
                .map(|input| (1..=input.abs_diff(i)).sum::<u16>())
                .sum::<u16>()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> &'static str { "16,1,2,0,4,2,7,1,2,14" }

    #[test]
    fn test_example() {
        let input = get_input();
        let input = parse_input(input);

        assert_eq!(solve_part_1(&input), 37);
        assert_eq!(solve_part_2(&input), 168);
    }

    #[test]
    fn test_real() {
        let input = parse_input(include_str!("../input/2021/day7.txt"));

        assert_eq!(solve_part_1(&input), 344138);
        assert_eq!(solve_part_2(&input), 94862124);
    }
}
