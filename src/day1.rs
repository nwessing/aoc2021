use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn solve(file_input: File) -> Result<(), ()> {
    let mut reader = BufReader::new(file_input);

    let mut result = 0;

    let mut window = [0; 3];
    let mut window_size = 0;
    let mut current_window_value = 0;

    let mut line = String::new();
    while reader.read_line(&mut line).map_err(|_| ())? != 0 {
        let number = i32::from_str_radix(&line.trim(), 10).map_err(|_| ())?;
        if window_size == 3 && number + window[2] + window[1] > current_window_value {
            result += 1;
        }

        if window_size < 3 {
            window_size += 1;
        }

        current_window_value -= window[0];
        current_window_value += number;

        window[0] = window[1];
        window[1] = window[2];
        window[2] = number;

        line.clear();
    }

    println!("{}", result);
    Ok(())
}
