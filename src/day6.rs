use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn tick(school: &mut [u64; 9]) {
    let mut original = [0u64; 9];
    original.clone_from(school);

    for day in (0..original.len()).rev() {
        if day + 1 < original.len() {
            school[day] = original[day + 1];
        } else {
            school[day] = 0;
        }

        if day == 0 {
            school[6] += original[0];
            school[8] += original[0];
        }
    }
}

pub fn solve(file_input: File) -> Result<(i64, i64), &'static str> {
    let mut reader = BufReader::new(file_input);
    let mut line = String::new();
    reader
        .read_to_string(&mut line)
        .map_err(|_| "Error reading input file")?;

    let mut school = [0u64; 9];
    for num in line.trim().split(',').map(|x| x.parse::<u32>()) {
        let num = num.map_err(|_| "Error parsing string")?;
        school[num as usize] += 1;
    }

    let mut part1 = 0u64;
    for i in 1..=256 {
        if i == 80 {
            part1 = school.iter().sum();
        }
        tick(&mut school);
    }

    let part2: u64 = school.iter().sum();

    Ok((part1 as i64, part2 as i64))
}
