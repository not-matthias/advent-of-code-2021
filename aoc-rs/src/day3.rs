use aoc_runner_derive::{aoc, aoc_generator};

fn binary_to_int(number: &[u8]) -> u64 {
    let mut result = 0u64;

    for (i, val) in number.iter().enumerate() {
        result += (*val as u64) << (number.len() - i - 1);
    }

    result
}

fn find_rating<F: Fn(usize, usize) -> u8, const T: usize>(input: &[[u8; T]], bit_criteria: F) -> [u8; T] {
    let mut filtered_bits = input.to_vec();
    for i in 0..T {
        let one_count = filtered_bits.iter().filter(|bit| bit[i] == 1).count();
        let zero_count = filtered_bits.iter().filter(|bit| bit[i] == 0).count();

        let bit_criteria = bit_criteria(one_count, zero_count);
        filtered_bits.retain(|bit| bit[i] == bit_criteria);

        if filtered_bits.len() == 1 {
            break;
        }
    }

    assert_eq!(filtered_bits.len(), 1);

    filtered_bits[0]
}

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

#[aoc(day3, part1)]
fn solve_part_1<const T: usize>(input: &[[u8; T]]) -> u64 {
    let mut gamma_rate: [u8; T] = [0u8; T];

    for i in 0..T {
        let zero_bits = input.iter().filter(|x| x[i] == 0).count();
        let one_bits = input.iter().filter(|x| x[i] == 1).count();

        if one_bits > zero_bits {
            gamma_rate[i] = 1;
        } else {
            gamma_rate[i] = 0;
        }
    }

    // Just flip the value
    //
    let epsilon_rate = gamma_rate
        .iter()
        .map(|bit| if *bit == 1 { 0 } else { 1 })
        .collect::<Vec<_>>();

    // Convert to int
    //
    let gamma_int = binary_to_int(&gamma_rate);
    let epsilon_int = binary_to_int(&epsilon_rate);

    gamma_int * epsilon_int
}

#[aoc(day3, part2)]
fn solve_part_2<const T: usize>(input: &[[u8; T]]) -> u64 {
    let oxygen_generator_rating = find_rating(
        input,
        |one_count, zero_count| if one_count >= zero_count { 1 } else { 0 },
    );
    let co2_scrubber_rating = find_rating(
        input,
        |one_count, zero_count| if zero_count <= one_count { 0 } else { 1 },
    );

    let oxygen_generator_int = binary_to_int(&oxygen_generator_rating);
    let co2_scrubber_int = binary_to_int(&co2_scrubber_rating);

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

        let result = find_rating(
            &input,
            |one_count, zero_count| if one_count >= zero_count { 1 } else { 0 },
        );
        assert_eq!(result, [1, 0, 1, 1, 1]);
    }

    #[test]
    fn test_find_co2_scrubber_rating() {
        let input = get_input();

        let result = find_rating(
            &input,
            |one_count, zero_count| if zero_count <= one_count { 0 } else { 1 },
        );
        assert_eq!(result, [0, 1, 0, 1, 0]);
    }
}
