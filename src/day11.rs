use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

struct Octopus {
    energy_level: u32,
    has_flashed: bool,
}

struct OctopusGrid {
    grid: Vec<Octopus>,
    stride: usize,
}

impl OctopusGrid {
    fn new(grid: Vec<Octopus>, stride: usize) -> Self {
        OctopusGrid { grid, stride }
    }

    fn step(&mut self) {
        for octo in self.grid.iter_mut() {
            octo.energy_level += 1;
        }

        for (i, octo) in self.grid.iter_mut().enumerate() {
            if !octo.has_flashed && octo.energy_level > 9 {
                octo.has_flashed = true;
            }
        }
    }

    // fn get_adjacent_locations(i: usize) -> [usize; 9] {}
}

pub fn solve(file_input: File) -> Result<(i64, i64), &'static str> {
    let mut grid = read_input(file_input)?;

    grid.step();

    Ok((0i64, 0i64))
}

fn read_input(file_input: File) -> Result<OctopusGrid, &'static str> {
    let mut reader = BufReader::new(file_input);
    let mut line = String::new();

    let mut stride = 0;
    let mut grid: Vec<Octopus> = Vec::with_capacity(1000);
    while reader
        .read_line(&mut line)
        .map_err(|_| "Error parsing line")?
        != 0
    {
        for character in line.trim().chars() {
            grid.push(Octopus {
                energy_level: character as u32 - '0' as u32,
                has_flashed: false,
            });
            if stride == 0 {
                stride = grid.len();
            }
        }

        line.clear();
    }

    Ok(OctopusGrid::new(grid, stride))
}
