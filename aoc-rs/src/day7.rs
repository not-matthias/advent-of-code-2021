use aoc_runner_derive::{aoc, aoc_generator};
use std::{collections::VecDeque, str::FromStr};

#[aoc_generator(day7)]
fn parse_input(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| line.split(',').filter_map(|part| u32::from_str(part).ok()))
        .flatten()
        .collect::<Vec<_>>()
}

fn input_to_array(input: &[u32]) -> VecDeque<u64> {
    let mut array = VecDeque::from([0u64; 9]);

    for i in input {
        array[(*i) as usize] += 1;
    }

    array
}

fn median(input: &[u32]) -> u32 { input.iter().sum::<u32>() / input.len() as u32 }

fn bruteforce(input: &[u32]) -> u32 {
    let max = input.iter().max().unwrap();

    let mut min_costs = u32::MAX;
    for i in 0..*max {
        let costs = input.iter().map(|input| i.abs_diff(*input)).sum::<u32>();
        if costs < min_costs {
            min_costs = costs;
        }
    }

    min_costs
}

#[aoc(day7, part1)]
fn solve_part_1(input: &[u32]) -> u32 {
    // let median = median(input);
    // let costs = input.iter().map(|i| i.abs_diff(median)).sum::<u32>();
    //
    // // Check n+-5 elements around the median
    // let mut min_median = median;
    // let mut min_costs = costs;
    // for i in -50..=50_i32 {
    //     let costs = input.iter().map(|i| i.abs_diff(median + i)).sum::<u32>();
    //     if costs < min_costs {
    //         min_median = (median as i32 + i) as u32;
    //         min_costs = costs;
    //     }
    // }
    //
    // // Option 1: Bruteforce
    // // Option 2: Calculate median
    //
    // min_costs
    bruteforce(input)
}

#[aoc(day7, part2)]
fn solve_part_2(input: &[u32]) -> u32 {
    //
    todo!()
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
        // assert_eq!(solve_part_2(&input), 0);
    }

    #[test]
    fn test_real() {
        let input = parse_input(include_str!("../input/2021/day7.txt"));

        assert_eq!(solve_part_1(&input), 344138);
    }
}
