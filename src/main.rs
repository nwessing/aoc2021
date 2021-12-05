mod day1;
mod day2;

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
        _ => {
            return Err("Unimplemented day");
        }
    };

    let result = run_day(&day, is_sample, func)?;

    println!("{}", result);

    Ok(())
}

fn run_day<F>(day: &str, is_sample: bool, func: F) -> Result<i32, &'static str>
where
    F: Fn(File) -> Result<i32, &'static str>,
{
    let filename = if is_sample {
        format!("input/{}_sample", day)
    } else {
        format!("input/{}", day)
    };

    let f = File::open(filename).map_err(|_| "Error opening input file")?;
    func(f)
}
