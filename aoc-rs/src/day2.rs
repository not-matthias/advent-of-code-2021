use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;

#[derive(Debug)]
enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Vec<Command> {
    input
        .lines()
        .filter_map(|line| {
            // WARNING: When testing things, you must not call `nth` or `next` since it
            //          consumes the iterator and will thus result in completely different
            //          outputs. Either don't print anything, or just use
            // `collect::<Vec<_>>()`.
            //
            //          In this case, nth(1) and nth(0) also caused some problems for me, so
            //          it's probably best to just use next() or next_back() instead.
            //
            let mut parts = line.split(' ');
            let num = u32::from_str(parts.next_back().unwrap()).unwrap();
            Some(match parts.next() {
                Some("forward") => Command::Forward(num),
                Some("down") => Command::Down(num),
                Some("up") => Command::Up(num),
                _ => {
                    return None;
                }
            })
        })
        .collect::<Vec<_>>()
}

#[aoc(day2, part1)]
fn solve_part_1(input: &[Command]) -> u32 {
    let (mut horizontal_position, mut depth) = (0, 0);
    for command in input {
        match command {
            Command::Forward(a) => {
                horizontal_position += a;
            }
            Command::Down(a) => {
                depth += a;
            }
            Command::Up(a) => {
                depth -= a;
            }
        }
    }

    depth * horizontal_position
}

#[aoc(day2, part2)]
fn solve_part_2(input: &[Command]) -> u32 {
    let (mut horizontal_position, mut depth, mut aim) = (0, 0, 0);

    for command in input {
        match command {
            Command::Forward(a) => {
                horizontal_position += a;
                depth += aim * a;
            }
            Command::Down(a) => {
                aim += a;
            }
            Command::Up(a) => {
                aim -= a;
            }
        }
    }

    depth * horizontal_position
}
