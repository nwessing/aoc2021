mod day1;
mod day2;
mod day3;

use std::env;
use std::fs::File;

fn main() -> Result<(), &'static str> {
    let day = env::args()
        .nth(1)
        .ok_or("What day would you like to run?")?;

    let is_sample = env::args().nth(2).unwrap_or_default() == "sample";

    let func = match day.as_str() {
        "day1" => day1::solve,
        "day2" => day2::solve,
        "day3" => day3::solve,
        _ => {
            return Err("Unimplemented day");
        }
    };

    let result = run_day(&day, is_sample, func)?;

    println!("Part 1: {}", result.0);
    println!("Part 2: {}", result.1);

    Ok(())
}

fn run_day<F>(day: &str, is_sample: bool, func: F) -> Result<(i32, i32), &'static str>
where
    F: Fn(File) -> Result<(i32, i32), &'static str>,
{
    let filename = if is_sample {
        format!("input/{}_sample", day)
    } else {
        format!("input/{}", day)
    };

    let f = File::open(filename).map_err(|_| "Error opening input file")?;
    func(f)
}
