use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::str::FromStr;

enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

fn parse_input(input: &str) -> Vec<Command> {
    input
        .lines()
        .filter_map(|line| {
            let mut parts = line.split(' ').collect::<Vec<_>>();

            // let Some(command) = &parts[0] else { println!("command: {:?}", parts); return None; };
            // let Some(num) = &parts[1] else { println!("num: {:?}", parts); return None; };
            // let Ok(num) = u32::from_str(num.trim()) else { println!("num: {:?}", num); return None; };
            //
            // Some(match command {
            //     "forward" => Command::Forward(num),
            //     "down" => Command::Forward(num),
            //     "up" => Command::Forward(num),
            //     _ => {
            //         println!("Not handled: {:?}", line);
            //         return None;
            //     }
            // })

            let num = u32::from_str(parts[1].trim()).unwrap();
            Some(match parts[0] {
                "forward" => Command::Forward(num),
                "down" => Command::Forward(num),
                "up" => Command::Forward(num),
                _ => {
                    println!("Not handled: {:?}", line);
                    return None;
                }
            })
        })
        .collect::<Vec<_>>()
}

// #[aoc_generator(day2)]
// fn parse_input(input: &str) -> Vec<Command> {
//     input
//         .lines()
//         .filter_map(|line| {
//             let mut parts = line.split(' ');
//             let num = u32::from_str(parts.nth(1).unwrap()).unwrap();
//
//             Some(match parts.as_str() {
//                 "forward" => Command::Forward(num),
//                 "down" => Command::Forward(num),
//                 "up" => Command::Forward(num),
//                 _ => {
//                     println!("Not handled: {:?}", line);
//                     return None;
//                 }
//             })
//         })
//         .collect::<Vec<_>>()
// }

#[aoc(day2, part1)]
fn solve_part_1(input: &str) -> u32 {
    let mut horizontal_position = 0;
    let mut depth = 0;

    for line in input.lines() {
        let mut parts = line.split(' ').collect::<Vec<_>>();

        let num = u32::from_str(parts[1].trim()).unwrap();

        match parts[0] {
            "forward" => horizontal_position += num,
            "down" => depth += num,
            "up" => depth -= num,
            _ => {
                println!("Not handled: {:?}", line);
                continue;
            }
        }
    }

    // for command in commands {
    //     match command {
    //         Command::Forward(a) => {
    //             horizontal_position += a;
    //         }
    //         Command::Down(a) => {
    //             depth += a;
    //         }
    //         Command::Up(a) => {
    //             depth -= a;
    //         }
    //     }
    // }

    //
    println!("horizontal_position: {:?}", horizontal_position);
    println!("depth: {:?}", depth);

    depth * horizontal_position
}

#[aoc(day2, part2)]
fn solve_part_2(input: &str) -> u32 {
    let mut horizontal_position = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in input.lines() {
        let mut parts = line.split(' ').collect::<Vec<_>>();

        let num = u32::from_str(parts[1].trim()).unwrap();

        match parts[0] {
            "forward" => {
                horizontal_position += num;
                depth = aim * num;
            }
            "down" => aim += num,
            "up" => aim -= num,
            _ => {
                println!("Not handled: {:?}", line);
                continue;
            }
        }
    }

    depth * horizontal_position
}
