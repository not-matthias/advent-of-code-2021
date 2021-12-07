use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;

#[aoc_generator(day7)]
fn parse_input(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|line| line.split(',').filter_map(|part| u64::from_str(part).ok()))
        .flatten()
        .collect::<Vec<_>>()
}

#[aoc(day7, part1)]
fn solve_part_1(input: &[u64]) -> u64 {
    let max = input.iter().max().unwrap();

    let mut min_costs = u64::MAX;
    for i in 0..*max {
        let costs = input.iter().map(|input| i.abs_diff(*input)).sum::<u64>();
        if costs < min_costs {
            min_costs = costs;
        }
    }

    min_costs
}

#[aoc(day7, part2)]
fn solve_part_2(input: &[u64]) -> u64 {
    let max = input.iter().max().unwrap();

    let mut min_costs = u64::MAX;
    for i in 0..*max {
        let costs = input
            .iter()
            .map(|input| (1..=input.abs_diff(i)).sum::<u64>())
            .sum::<u64>();
        if costs < min_costs {
            min_costs = costs;
        }
    }

    min_costs
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
