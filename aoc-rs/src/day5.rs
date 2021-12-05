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
    pub points: Vec<(u32, u32)>,
}

impl Diagram {
    pub fn new() -> Self { Default::default() }

    pub fn draw_line(&mut self, line: &Line) {
        // Diagonal line
        if line.from.0 != line.to.0 && line.from.1 != line.to.1 {
            let mut x_range: Vec<_> = (line.from.0..=line.to.0).collect();
            if line.from.0 > line.to.0 {
                x_range = (line.to.0..=line.from.0).rev().collect();
            }

            let mut y_range: Vec<_> = (line.from.1..=line.to.1).collect();
            if line.from.1 > line.to.1 {
                y_range = (line.to.1..=line.from.1).rev().collect();
            }

            let points = x_range
                .into_iter()
                .enumerate()
                .zip(y_range.into_iter().enumerate())
                .filter_map(|((xi, x), (yi, y))| {
                    //
                    if xi == yi {
                        Some((x, y))
                    } else {
                        None
                    }
                });
            self.points.extend(points);
            // for x in  {
            //     for y in  {
            //         // self.lines[x as usize][y as usize] += 1;
            //         self.points.push((x, y));
            //     }
            // }

            return;
        }

        // Vertical line
        if line.from.0 == line.to.0 {
            let start = line.from.1.min(line.to.1);
            let end = line.from.1.max(line.to.1);

            for p in start..=end {
                self.points.push((line.from.0, p));
            }
            return;
        }

        // Horizontal line
        if line.from.1 == line.to.1 {
            let start = line.from.0.min(line.to.0);
            let end = line.from.0.max(line.to.0);

            for p in start..=end {
                self.points.push((p, line.to.1));
            }
            return;
        }
    }

    pub fn duplicate_point_count(&self) -> u32 { self.points.iter().duplicates().count() as u32 }
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
    let mut diagram = Diagram::new();
    for line in input {
        diagram.draw_line(line);
    }

    diagram.duplicate_point_count()
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
        // Horizontal line
        let mut diagram = Diagram::new();
        diagram.draw_line(&Line {
            from: (9, 7),
            to: (7, 7),
        });
        assert_eq!(diagram.points, vec![(7, 7), (8, 7), (9, 7)]);

        // Vertical line
        let mut diagram = Diagram::new();
        diagram.draw_line(&Line {
            from: (1, 1),
            to: (1, 3),
        });
        assert_eq!(diagram.points, vec![(1, 1), (1, 2), (1, 3)]);

        // Diagonal line
        let mut diagram = Diagram::new();
        diagram.draw_line(&Line {
            from: (1, 1),
            to: (3, 3),
        });
        assert_eq!(diagram.points, vec![(1, 1), (2, 2), (3, 3)]);

        let mut diagram = Diagram::new();
        diagram.draw_line(&Line {
            from: (9, 7),
            to: (7, 9),
        });
        assert_eq!(diagram.points, vec![(9, 7), (8, 8), (7, 9)]);
    }

    #[test]
    fn test_example() {
        let lines = parse_input(get_input());
        let mut diagram = Diagram::new();

        let line = Line {
            from: (9, 7),
            to: (7, 7),
        };
        diagram.draw_line(&line);

        // for line in lines {
        //     diagram.draw_line(line);
        // }

        assert_eq!(diagram.duplicate_point_count(), 5);
    }
}
