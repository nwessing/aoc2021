use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

enum MostCommon {
    One,
    Zero,
    Tie,
}

pub fn solve(file_input: File) -> Result<(i64, i64), &'static str> {
    let mut input_count = 0;
    let mut num_bits: Option<u32> = None;
    let mut counter: Vec<i32> = Vec::new();
    let mut numbers = vec![];

    let mut reader = BufReader::new(file_input);
    let mut line = String::new();
    while reader
        .read_line(&mut line)
        .map_err(|_| "Error reading input file")?
        != 0
    {
        let number = line.trim();
        numbers.push(u32::from_str_radix(number, 2).map_err(|_| "Error parsing")?);
        if num_bits.is_none() {
            num_bits = Some(number.len() as u32);
            counter.resize(number.len(), 0)
        }

        for (i, bit) in number.chars().enumerate() {
            if bit == '1' {
                counter[i] += 1;
            }
        }

        input_count += 1;

        line.clear();
    }

    let mut gamma = 0u32;
    let mut epsilon = 0u32;
    for bit_count in counter.iter() {
        gamma <<= 1;
        epsilon <<= 1;

        let (gamma_bit, epsilon_bit) = if *bit_count >= input_count / 2 {
            (1, 0)
        } else {
            (0, 1)
        };
        gamma |= gamma_bit;
        epsilon |= epsilon_bit;
    }
    let part1 = gamma * epsilon;

    let num_bits = num_bits.unwrap();

    let oxygen_generator_rating = find_rating(
        &numbers,
        num_bits,
        RatingConfig {
            use_most_common: true,
            tie_breaker: 1,
        },
    );
    let c02_scrubber_rating = find_rating(
        &numbers,
        num_bits,
        RatingConfig {
            use_most_common: false,
            tie_breaker: 0,
        },
    );
    let part2 = (oxygen_generator_rating * c02_scrubber_rating) as i64;
    return Ok((part1 as i64, part2));
}

struct RatingConfig {
    use_most_common: bool,
    tie_breaker: u32,
}

fn find_rating(numbers: &[u32], num_bits: u32, config: RatingConfig) -> u32 {
    let mut is_candiate = vec![true; numbers.len()];
    let mut candidate_count = is_candiate.len();

    for i_bit in (0..num_bits).rev() {
        let mut one_count = 0;
        for (i_number, number) in numbers.iter().enumerate() {
            if !is_candiate[i_number] {
                continue;
            }

            if (number & (1 << i_bit)) > 0 {
                one_count += 1;
            }
        }

        let most_common = if candidate_count % 2 == 0 && one_count == candidate_count / 2 {
            MostCommon::Tie
        } else if one_count > candidate_count / 2 {
            MostCommon::One
        } else {
            MostCommon::Zero
        };

        let expected_bit_value = match (config.use_most_common, most_common) {
            (true, MostCommon::Zero) => 0,
            (false, MostCommon::Zero) => 1,
            (true, MostCommon::One) => 1,
            (false, MostCommon::One) => 0,
            (_, MostCommon::Tie) => config.tie_breaker,
        };

        for (i_number, number) in numbers.iter().enumerate() {
            if !is_candiate[i_number] {
                continue;
            }

            if (number & (1 << i_bit)) != (expected_bit_value << i_bit) {
                is_candiate[i_number] = false;
                candidate_count -= 1;
            }

            if candidate_count == 1 {
                let mut result_iter = is_candiate.iter().enumerate().filter(|(_, x)| **x);
                return numbers[result_iter.next().unwrap().0];
            }
        }
    }

    panic!("No answer found!");
}
