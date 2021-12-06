use aoc_runner_derive::{aoc, aoc_generator};
use std::{collections::VecDeque, str::FromStr};

#[aoc_generator(day6)]
fn parse_input(input: &str) -> Vec<u8> {
    input
        .lines()
        .map(|line| line.split(',').filter_map(|num| u8::from_str(num).ok()))
        .flatten()
        .collect::<Vec<_>>()
}

fn input_to_array(input: &[u8]) -> VecDeque<u64> {
    let mut array = VecDeque::from([0u64; 9]);

    for i in input {
        array[(*i) as usize] += 1;
    }

    array
}

fn simulate(input: &[u8], days: u64) -> u64 {
    let mut data = input_to_array(input);

    // Simulate each day
    for _ in 0..days {
        let zero_states = data.pop_front().unwrap_or_default();
        data[6] += zero_states;
        data.push_back(zero_states);
    }

    data.into_iter().sum::<u64>()
}

#[aoc(day6, part1)]
fn solve_part_1(input: &[u8]) -> u64 { simulate(input, 80) }

#[aoc(day6, part2)]
fn solve_part_2(input: &[u8]) -> u64 { simulate(input, 256) }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_to_array() {
        let input = parse_input("3,4,3,1,2");
        let array = input_to_array(&input);

        assert_eq!(array, vec![0, 1, 1, 2, 1, 0, 0, 0, 0]);
    }

    #[test]
    fn test_example() {
        let input = parse_input("3,4,3,1,2");

        let result = solve_part_1(&input);
        assert_eq!(result, 5934);

        let result = solve_part_2(&input);
        assert_eq!(result, 26984457539);
    }

    #[test]
    fn test_real() {
        let input = parse_input(include_str!("../input/2021/day6.txt"));

        let result = solve_part_1(&input);
        assert_eq!(result, 388739);

        let result = solve_part_2(&input);
        assert_eq!(result, 1741362314973);
    }
}
