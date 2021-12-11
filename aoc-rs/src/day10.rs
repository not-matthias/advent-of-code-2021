use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::VecDeque;

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Vec<VecDeque<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<VecDeque<_>>())
        .collect::<Vec<_>>()
}

#[aoc(day10, part1)]
fn solve_part_1(input: &[VecDeque<char>]) -> u32 {
    let mut stack = VecDeque::new();
    let mut points = 0;

    for line in input {
        let result = line.iter().fold(0, |mut acc, item| {
            match item {
                '<' | '(' | '[' | '{' => stack.push_back(item),
                '}' | ']' | ')' | '>' => {
                    let last_item = *stack.pop_back().unwrap();

                    acc += match item {
                        '}' if last_item != '{' => 1197,
                        ']' if last_item != '[' => 57,
                        ')' if last_item != '(' => 3,
                        '>' if last_item != '<' => 25137,
                        _ => 0,
                    };
                }
                _ => unreachable!("Invalid character: {}", item),
            }

            acc
        });

        points += result;
    }

    points
}

#[aoc(day10, part2)]
fn solve_part_2(input: &[VecDeque<char>]) -> u64 {
    let mut stack = VecDeque::new();
    let mut scores = Vec::new();

    for line in input {
        let result = line.iter().fold(0, |mut acc, item| {
            match item {
                '<' | '(' | '[' | '{' => stack.push_back(item),
                '}' | ']' | ')' | '>' => {
                    let last_item = *stack.pop_back().unwrap();

                    acc += match item {
                        '}' if last_item != '{' => 1197,
                        ']' if last_item != '[' => 57,
                        ')' if last_item != '(' => 3,
                        '>' if last_item != '<' => 25137,
                        _ => 0,
                    };
                }
                _ => unreachable!("Invalid character"),
            }

            acc
        });

        if result == 0 {
            let mut points = 0;
            for item in stack.iter().rev() {
                points *= 5;
                points += match item {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => unreachable!(),
                }
            }

            println!(
                "Missing elements: {:?} - {:?} - {:?}",
                line.iter().join(""),
                stack.iter().join(""),
                points
            );

            scores.push(points);
        }

        stack.clear();
    }

    scores.sort_unstable();

    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    fn get_input() -> &'static str {
        "[({(<(())[]>[[{[]{<()<>>\n[(()[<>])]({[<{<<[]>>(\n{([(<{}[<>[]}>{[]{[(<()>\n(((({<>}<{<{<>}{[]{[]{}\n[[<[([]))<([[{}[[()]]]\n[{[{({}]{}}([{[{{{}}([]\n{<[[]]>}<{[{[{[]{()[[[]\n[<(<(<(<{}))><([]([]()\n<{([([[(<>()){}]>(<<{{\n<{([{{}}[<[[[<>{}]]]>[]]"
    }

    #[test]
    fn test_example() {
        let input = get_input();

        let input = parse_input(input);

        assert_eq!(solve_part_1(&input), 26397);
        assert_eq!(solve_part_2(&input), 288957);
    }
}
