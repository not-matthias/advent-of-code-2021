use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

#[derive(Debug, Copy, Clone)]
pub struct Line {
    pub from: (u32, u32),
    pub to: (u32, u32),
}

#[derive(Default)]
pub struct Diagram {
    pub lines: [[u32; 9]; 9],
    pub points: Vec<(u32, u32)>,
}

impl Diagram {
    pub fn new() -> Self { Default::default() }

    pub fn draw_line(&mut self, line: &Line) {
        // Only consider horizontal and vertical lines
        if line.from.0 != line.to.0 && line.from.1 != line.to.1 {
            println!("Skipping");
            return;
        }

        if line.from.0 == line.to.0 {
            let start = line.from.1.min(line.to.1);
            let end = line.from.1.max(line.to.1);

            for p in start..=end {
                // self.lines[line.from.0 as usize][p as usize] += 1;
                self.points.push((line.from.0, p));
            }
        }
        if line.from.1 == line.to.1 {
            let start = line.from.0.min(line.to.0);
            let end = line.from.0.max(line.to.0);

            for p in start..=end {
                // self.lines[p as usize][line.to.1 as usize] += 1;
                self.points.push((p, line.to.1));
            }
        }
    }

    pub fn duplicate_point_count(&self) -> u32 { self.points.iter().duplicates().count() as u32 }
}

impl Display for Diagram {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for line in self.lines {
            for item in line {
                writeln!(f, "{:?}", item)?;
            }
        }

        Ok(())
    }
}

#[aoc_generator(day5)]
fn parse_input(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(" -> ").collect::<Vec<_>>();

            let from = parts[0]
                .split(",")
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            let to = parts[1]
                .split(",")
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<_>>();

            Line {
                from: (from[0], from[1]),
                to: (to[0], to[1]),
            }
        })
        .collect::<Vec<_>>()
}

#[aoc(day5, part1)]
fn solve_part_1(input: &[Line]) -> u32 {
    let mut diagram = Diagram::new();
    for line in input {
        diagram.draw_line(line);
    }

    diagram.duplicate_point_count()
}

#[aoc(day5, part2)]
fn solve_part_2(input: &[Line]) -> u32 {
    let (mut horizontal_position, mut depth, mut aim) = (0, 0, 0);

    depth * horizontal_position
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> &'static str {
        "0,9 -> 5,9\n8,0 -> 0,8\n9,4 -> 3,4\n2,2 -> 2,1\n7,0 -> 7,4\n6,4 -> 2,0\n0,9 -> 2,9\n3,4 -> 1,4\n0,0 -> \
         8,8\n5,5 -> 8,2"
    }

    #[test]
    fn test_draw_line() {
        let mut diagram = Diagram::new();
        diagram.draw_line(Line {
            from: (9, 7),
            to: (7, 7),
        });
        assert_eq!(diagram.points, vec![(7, 7), (8, 7), (9, 7)]);

        let mut diagram = Diagram::new();
        diagram.draw_line(Line {
            from: (1, 1),
            to: (1, 3),
        });
        assert_eq!(diagram.points, vec![(1, 1), (1, 2), (1, 3)]);
    }

    #[test]
    fn test_example() {
        let lines = parse_input(get_input());
        let mut diagram = Diagram::new();

        let line = Line {
            from: (9, 7),
            to: (7, 7),
        };
        diagram.draw_line(line);

        // for line in lines {
        //     diagram.draw_line(line);
        // }

        assert_eq!(diagram.duplicate_point_count(), 5);
    }
}
