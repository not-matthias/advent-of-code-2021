use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashMap;

#[aoc_generator(day9)]
fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .into_iter()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

#[aoc(day9, part1)]
fn solve_part_1(input: &[Vec<u32>]) -> u32 {
    let mut min_num = Vec::new();
    for row in 0..input.len() {
        for col in 0..input[0].len() {
            let current = input[row][col];

            let up = if row.checked_sub(1).is_none() {
                true
            } else {
                current < input[row - 1][col]
            };
            let down = if row + 1 == input.len() {
                true
            } else {
                current < input[row + 1][col]
            };
            let left = if col.checked_sub(1).is_none() {
                true
            } else {
                current < input[row][col - 1]
            };
            let right = if col + 1 >= input[0].len() {
                true
            } else {
                current < input[row][col + 1]
            };

            if up && down && left && right {
                min_num.push(current);
            }
        }
    }

    min_num.into_iter().map(|n| n + 1).sum()
}

fn flood_fill(
    checked: &mut HashMap<(usize, usize), bool>, input: &[Vec<u32>],
    result: &mut Vec<((usize, usize), u32)>, current: (usize, usize),
) {
    let (x, y) = current;

    // Return if we are at a 9
    //
    if input[x][y] == 9 {
        return;
    }

    // Return if already checked
    //
    if checked.contains_key(&current) {
        return;
    }

    checked.insert(current, true);
    result.push(((x, y), input[x][y]));

    if x.checked_sub(1).is_some() {
        flood_fill(checked, input, result, (x - 1, y));
    }
    if x + 1 < input.len() {
        flood_fill(checked, input, result, (x + 1, y));
    }

    if y.checked_sub(1).is_some() {
        flood_fill(checked, input, result, (x, y - 1));
    }
    if y + 1 < input[0].len() {
        flood_fill(checked, input, result, (x, y + 1));
    }
}

#[aoc(day9, part2)]
fn solve_part_2(input: &[Vec<u32>]) -> u32 {
    let mut all_results = Vec::new();
    let mut checked = HashMap::new();

    for row in 0..input.len() {
        for col in 0..input[0].len() {
            let mut result = Vec::new();
            flood_fill(&mut checked, input, &mut result, (row, col));
            all_results.push(result);
        }
    }

    // Find the 3 largest basins and return them.
    //
    all_results
        .iter()
        .sorted_by_key(|r| r.len())
        .rev()
        .take(3)
        .map(|n| n.len())
        .reduce(|acc, val| acc * val)
        .unwrap() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> &'static str {
        "2199943210\n3987894921\n9856789892\n8767896789\n9899965678"
    }

    #[test]
    fn test_example() {
        let input = get_input();
        let input = parse_input(input);

        assert_eq!(solve_part_1(&input), 15);
        assert_eq!(solve_part_2(&input), 1134);
    }

    #[test]
    fn test_real() {
        let input = include_str!("../input/2021/day9.txt");
        let input = parse_input(input);

        assert_eq!(solve_part_1(&input), 594);
        assert_eq!(solve_part_2(&input), 858494);
    }
}
