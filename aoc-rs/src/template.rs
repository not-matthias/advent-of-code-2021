use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;

#[aoc_generator(dayN)]
fn parse_input(input: &str) -> Vec<u8> {
    input
        .lines()
        .filter_map(|line| {
            let parts = line.split(' ');

            todo!()
        })
        .collect::<Vec<_>>()
}

#[aoc(dayN, part1)]
fn solve_part_1(input: &[u8]) -> u32 {
    //
    todo!()
}

#[aoc(dayN, part2)]
fn solve_part_2(input: &[u8]) -> u32 {
    //
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> &'static str { "" }

    #[test]
    fn test_example() {
        let input = get_input();
        let input = parse_input(input);

        assert_eq!(solve_part_1(&input), 0);
        assert_eq!(solve_part_2(&input), 0);
    }
}
