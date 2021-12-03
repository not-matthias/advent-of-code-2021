use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[aoc_generator(day3)]
fn parse_input(input: &str) -> Vec<[u8; 12]> {
    input
        .lines()
        .map(|line| {
            let mut bytes: [u8; 12] = [0u8; 12];
            for (i, char) in line.chars().enumerate() {
                bytes[i] += char.to_digit(10).unwrap() as u8;
            }
            bytes
        })
        .collect::<Vec<_>>()
}

fn binary_to_int(number: &[u8]) -> u64 {
    let mut result = 0u64;

    for (i, val) in number.iter().enumerate() {
        result += (*val as u64) << (number.len() - i - 1);
    }

    result
}

#[aoc(day3, part1)]
fn solve_part_1(input: &[[u8; 12]]) -> u64 {
    let mut gamma_rate: [u8; 12] = [0u8; 12];

    let mut zero_bits = 0;
    let mut one_bits = 0;
    for i in 0..12 {
        for bit in input {
            if bit[i] == 0 {
                zero_bits += 1;
            } else {
                one_bits += 1;
            }
        }

        if one_bits > zero_bits {
            gamma_rate[i] = 1;
        } else {
            gamma_rate[i] = 0;
        }
        one_bits = 0;
        zero_bits = 0;
    }

    // Just flip the values
    let epsilon_rate = gamma_rate
        .iter()
        .map(|bit| if *bit == 1 { 0 } else { 1 })
        .collect::<Vec<_>>();

    // Convert to int
    let gamma_int = binary_to_int(&gamma_rate);
    let epsilon_int = binary_to_int(&epsilon_rate);

    gamma_int * epsilon_int
}

fn flip_bits(number: &[u8]) -> [u8; 12] {
    let mut result = [0u8; 12];

    for (i, val) in number.iter().enumerate() {
        result[i] = if *val == 1 { 0 } else { 1 };
    }

    result
}

fn find_oxygen_generator_rating<const T: usize>(input: &[[u8; T]]) -> [u8; T] {
    // Find the correct value
    //
    let mut filtered_bits = input.to_vec();
    for i in 0..T {
        let one_count = filtered_bits
            .iter()
            .map(|bit| bit[i])
            .map(|bit| bit as u64)
            .sum::<u64>();
        let zero_count = filtered_bits
            .iter()
            .map(|bits| flip_bits(bits))
            .map(|bit| bit[i])
            .map(|bit| bit as u64)
            .sum();

        let bit_criteria = if one_count >= zero_count { 1 } else { 0 };
        filtered_bits.retain(|bit| bit[i] == bit_criteria);

        if filtered_bits.len() == 1 {
            break;
        }
    }

    filtered_bits[0]
}

fn find_co2_scrubber_rating<const T: usize>(input: &[[u8; T]]) -> Vec<[u8; T]> {
    // Find the correct value
    //
    let mut filtered_bits = input.to_vec();
    for i in 0..T {
        let one_count: u64 = filtered_bits
            .iter()
            .map(|bit| bit[i])
            .map(|bit| bit as u64)
            .sum();
        let zero_count: u64 = filtered_bits
            .iter()
            .map(|bits| flip_bits(bits))
            .map(|bit| bit[i])
            .map(|bit| bit as u64)
            .sum();

        let bit_criteria = if zero_count <= one_count { 0 } else { 1 };

        filtered_bits.retain(|bit| bit[i] == bit_criteria);

        if filtered_bits.len() == 1 {
            break;
        }
    }

    filtered_bits
}

#[aoc(day3, part2)]
fn solve_part_2<const T: usize>(input: &[[u8; T]]) -> u64 {
    let oxygen_generator_rating = find_oxygen_generator_rating(input);
    let co2_scrubber_rating = find_co2_scrubber_rating(input);

    let oxygen_generator_int = binary_to_int(&oxygen_generator_rating);
    let co2_scrubber_int = binary_to_int(&co2_scrubber_rating[0]);

    oxygen_generator_int * co2_scrubber_int
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> [[u8; 5]; 12] {
        [
            [0, 0, 1, 0, 0],
            [1, 1, 1, 1, 0],
            [1, 0, 1, 1, 0],
            [1, 0, 1, 1, 1],
            [1, 0, 1, 0, 1],
            [0, 1, 1, 1, 1],
            [0, 0, 1, 1, 1],
            [1, 1, 1, 0, 0],
            [1, 0, 0, 0, 0],
            [1, 1, 0, 0, 1],
            [0, 0, 0, 1, 0],
            [0, 1, 0, 1, 0],
        ]
    }

    #[test]
    fn test_binary_to_int() {
        assert_eq!(binary_to_int(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1]), 7);
        assert_eq!(binary_to_int(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0]), 4);
        assert_eq!(binary_to_int(&[0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0]), 8);
        assert_eq!(binary_to_int(&[1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 0, 1]), 3829);
    }

    #[test]
    fn test_solve_part_2() {
        let input = get_input();
        let result = solve_part_2(&input);
        assert_eq!(result, 230);
    }

    #[test]
    fn test_find_oxygen_generator_rating() {
        let input = get_input();

        let result = find_oxygen_generator_rating(&input);
        assert_eq!(result, [1, 0, 1, 1, 1]);
    }

    #[test]
    fn test_find_co2_scrubber_rating() {
        let input = get_input();

        let mut result = find_co2_scrubber_rating(&input);
        while result.len() != 1 {
            println!("Calling it again");
            result = find_co2_scrubber_rating(&result);
        }

        assert_eq!(result[0], [0, 1, 0, 1, 0]);
    }
}
