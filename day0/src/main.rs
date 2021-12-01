use std::str::FromStr;

fn solve_part_1(input: &str) -> u32 {
    input
        .lines()
        .collect::<Vec<_>>()
        .windows(2)
        .filter_map(|lines| {
            let previous = lines[0];
            let current = lines[1];

            if u32::from_str(previous).unwrap() < u32::from_str(current).unwrap() {
                Some(1)
            } else {
                None
            }
        })
        .sum::<u32>()
}

fn solve_part_2(input: &str) -> u32 {
    input
        .lines()
        .collect::<Vec<_>>()
        .windows(3)
        .map(|lines| {
            Some(
                u32::from_str(lines[0]).unwrap()
                    + u32::from_str(lines[1]).unwrap()
                    + u32::from_str(lines[2]).unwrap(),
            )
        })
        .collect::<Vec<_>>()
        .windows(2)
        .filter_map(|sum| if sum[0] < sum[1] { Some(1) } else { None })
        .sum::<u32>()
}

fn main() {
    println!(
        "[Part 1] {}",
        solve_part_1(include_str!("../data/input_1.txt"))
    );

    println!(
        "[Part 1] {} ",
        solve_part_2(include_str!("../data/input_2.txt"))
    );
}
