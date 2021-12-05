mod day1;

use std::fs::File;

fn main() -> std::io::Result<()> {
    let f = File::open("input/day1")?;
    day1::solve(f).expect("Error solving day 1");

    Ok(())
}
