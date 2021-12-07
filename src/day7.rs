use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn solve(file_input: File) -> Result<(i64, i64), &'static str> {
    let mut reader = BufReader::new(file_input);
    let mut line = String::new();
    reader
        .read_to_string(&mut line)
        .map_err(|_| "Error reading input file")?;

    let mut positions: Vec<i32> = Vec::new();
    for num in line.trim().split(',').map(|x| x.parse::<i32>()) {
        let num = num.map_err(|_| "Error parsing string")?;
        positions.push(num);
    }

    let min = *positions.iter().min().ok_or("Empty input")?;
    let max = *positions.iter().max().ok_or("Empty input")?;
    // println!("{} - {}", min, max);

    // let mut sum = 0f32;
    // for pos in positions.iter() {
    //     sum += *pos as f32;
    // }

    // let center = sum / positions.len() as f32;
    // let center = f32::round(center) as i32;
    // println!("center = {}", center);

    let mut part1 = i64::max_value();
    let mut part2 = i64::max_value();
    for i in min..=max {
        let mut fuel_cost_p1 = 0i64;
        let mut fuel_cost_p2 = 0i64;
        for pos in positions.iter() {
            let distance = (i - *pos).abs() as i64;
            fuel_cost_p1 += distance;
            fuel_cost_p2 += (distance * (distance + 1)) / 2;
        }

        part1 = part1.min(fuel_cost_p1);
        part2 = part2.min(fuel_cost_p2);
    }
    // let mut sum = 0f32;
    // for pos in positions.iter() {
    //     sum += *pos as f32;
    // }

    // let center = sum / positions.len() as f32;
    // let center = f32::round(center) as i32;
    // println!("center = {}", center);

    // let mut num_moves = 0i64;
    // for pos in positions.iter() {
    //     num_moves += (center - *pos).abs() as i64;
    // }

    Ok((part1, part2))
}
