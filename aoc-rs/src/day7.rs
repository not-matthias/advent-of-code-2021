use aoc_runner_derive::{aoc, aoc_generator};
use rayon::prelude::*;
use std::str::FromStr;

#[aoc_generator(day7)]
fn parse_input(input: &str) -> Vec<u32> { input.trim().split(',').flat_map(u32::from_str).collect::<Vec<_>>() }

#[aoc(day7, part1)]
fn solve_part_1(input: &[u32]) -> u32 {
    let min = input.iter().min().unwrap();
    let max = input.iter().max().unwrap();

    (*min..*max)
        .into_par_iter()
        .map(|i| input.iter().map(|input| i.abs_diff(*input)).sum::<u32>())
        .min()
        .unwrap()
}

#[aoc(day7, part2)]
fn solve_part_2(input: &[u32]) -> u32 {
    let min = input.iter().min().unwrap();
    let max = input.iter().max().unwrap();

    (*min..*max)
        .into_par_iter()
        .map(|i| {
            input
                .into_iter()
                .map(|input| {
                    let n = input.abs_diff(i);

                    (n * (n + 1)) / 2
                })
                .sum::<u32>()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> &'static str { "16,1,2,0,4,2,7,1,2,14" }

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input(get_input()), vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]);
        assert_eq!(parse_input(include_str!("../input/2021/day7.txt")).len(), 1000);
    }

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
