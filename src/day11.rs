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
    height: usize,
}

impl OctopusGrid {
    fn new(grid: Vec<Octopus>, stride: usize) -> Self {
        let height = grid.len() / stride;
        OctopusGrid {
            grid,
            stride,
            height,
        }
    }

    fn step(&mut self) -> i64 {
        let mut result = 0;

        for octo in self.grid.iter_mut() {
            octo.energy_level += 1;
        }

        loop {
            let mut flashes_this_iteration = 0;
            for i in 0..self.grid.len() {
                let octo = &mut self.grid[i];
                if octo.has_flashed || octo.energy_level <= 9 {
                    continue;
                }

                octo.has_flashed = true;
                flashes_this_iteration += 1;

                for coord in Self::get_adjacent_locations(self.to_coordinate(i)) {
                    if let Some(i_neighbor) = self.from_coordinate(coord) {
                        let neighbor = &mut self.grid[i_neighbor];
                        neighbor.energy_level += 1;
                    }
                }
            }

            result += flashes_this_iteration;
            if flashes_this_iteration == 0 {
                break;
            }
        }

        for octo in self.grid.iter_mut() {
            if octo.energy_level > 9 {
                octo.energy_level = 0;
                octo.has_flashed = false;
            }
        }

        return result;
    }

    fn get_adjacent_locations((x, y): (i32, i32)) -> [(i32, i32); 8] {
        [
            (x - 1, y - 1),
            (x + 0, y - 1),
            (x + 1, y - 1),
            (x - 1, y + 0),
            // (x + 0, y + 0),
            (x + 1, y + 0),
            (x - 1, y + 1),
            (x + 0, y + 1),
            (x + 1, y + 1),
        ]
    }

    fn to_coordinate(&self, i: usize) -> (i32, i32) {
        ((i % self.stride) as i32, (i / self.stride) as i32)
    }

    fn from_coordinate(&self, coord: (i32, i32)) -> Option<usize> {
        let (x, y) = coord;
        if x >= 0 && x < self.stride as i32 && y >= 0 && y < self.height as i32 {
            return Some(x as usize + y as usize * self.stride);
        }
        return None;
    }
}

pub fn solve(file_input: File) -> Result<(i64, i64), &'static str> {
    let mut grid = read_input(file_input)?;

    let mut part1 = 0;
    let mut part2: Option<i64> = None;
    for i in 1..usize::MAX {
        let num_flashed = grid.step();
        if i <= 100 {
            part1 += num_flashed;
        }

        if part2 == None && num_flashed == grid.grid.len() as i64 {
            part2 = Some(i as i64);
        }

        if part2.is_some() && i > 100 {
            break;
        }
    }

    Ok((part1, part2.unwrap()))
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
        }

        if stride == 0 {
            stride = grid.len();
        }

        line.clear();
    }

    Ok(OctopusGrid::new(grid, stride))
}
