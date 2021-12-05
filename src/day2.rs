use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn solve(file_input: File) -> Result<i32, &'static str> {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    let mut reader = BufReader::new(file_input);
    let mut line = String::new();
    while reader
        .read_line(&mut line)
        .map_err(|_| "Error reading input file")?
        != 0
    {
        let mut iter = line.trim().split_whitespace();

        let direction = iter.next().unwrap();
        let amount = i32::from_str_radix(iter.next().unwrap(), 10)
            .map_err(|_| "Error parsing string to int")?;

        match direction {
            "forward" => {
                horizontal += amount;
                depth += aim * amount;
            }
            "down" => {
                aim += amount;
            }
            "up" => {
                aim -= amount;
            }
            _ => {
                panic!("Invalid direction {}", direction);
            }
        }

        line.clear();
    }

    Ok(horizontal * depth)
}
