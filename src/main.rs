mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::time::{Duration, Instant};

type SolverFunc = fn(File) -> Result<(i64, i64), &'static str>;

fn main() -> Result<(), &'static str> {
    let day = env::args()
        .nth(1)
        .ok_or("What day would you like to run?")?;

    let is_sample = env::args().nth(2).unwrap_or_default() == "sample";

    let solutions: BTreeMap<&str, SolverFunc> = BTreeMap::from([
        ("day1", day1::solve as SolverFunc),
        ("day2", day2::solve),
        ("day3", day3::solve),
        ("day4", day4::solve),
        ("day5", day5::solve),
        ("day6", day6::solve),
        ("day7", day7::solve),
    ]);

    if day.as_str() == "all" {
        let mut total_runtime = Duration::new(0, 0);
        for (day, func) in solutions.iter() {
            let result = run_day(&day, is_sample, func)?;
            println!("{} ({:?}) ", day, result.2);
            println!("  Part 1: {}", result.0);
            println!("  Part 2: {}", result.1);
            total_runtime += result.2;
        }
        println!("Total Elapsed: {:?}", total_runtime);
    } else {
        let func = solutions.get(day.as_str()).ok_or("Unimplemented day")?;

        let result = run_day(&day, is_sample, func)?;

        println!("Part 1: {}", result.0);
        println!("Part 2: {}", result.1);
        println!("Elapsed: {:?}", result.2)
    }

    Ok(())
}

fn run_day(
    day: &str,
    is_sample: bool,
    func: &SolverFunc,
) -> Result<(i64, i64, Duration), &'static str> {
    let filename = if is_sample {
        format!("input/{}_sample", day)
    } else {
        format!("input/{}", day)
    };

    let f = File::open(filename).map_err(|_| "Error opening input file")?;

    let start = Instant::now();
    let (part1, part2) = func(f)?;
    let end = Instant::now();
    let duration = end - start;
    Ok((part1, part2, duration))
}
