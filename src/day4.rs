use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
pub struct BingoBoard {
    pub grid: [u32; 25],
    pub marked: [bool; 25],
    pub won: bool,
}

impl BingoBoard {
    pub fn new(reader: &mut BufReader<File>) -> Option<Self> {
        let mut grid = [0u32; 25];

        let mut current_row = 0u32;
        let mut raw_line = String::new();
        while reader.read_line(&mut raw_line).ok()? != 0 {
            let line = raw_line.trim();
            if line.is_empty() {
                continue;
            }

            for (col, num_str) in line.split_whitespace().enumerate() {
                grid[(current_row * 5 + col as u32) as usize] =
                    u32::from_str_radix(num_str, 10).ok()?;
                if col > 4 {
                    panic!("Bad bingo board input");
                }
            }
            current_row += 1;

            if current_row == 5 {
                break;
            }

            raw_line.clear();
        }

        if current_row != 5 {
            return None;
        }

        Some(Self {
            grid,
            marked: [false; 25],
            won: false,
        })
    }

    pub fn mark(&mut self, number: u32) {
        for (i, board_num) in self.grid.iter().enumerate() {
            if *board_num == number {
                self.marked[i] = true;
            }
        }
    }

    pub fn check_win(&mut self) -> Option<u32> {
        let mut win = false;
        for i in 0..5 {
            let col_win = self.marked[i + (0 * 5)]
                && self.marked[i + (1 * 5)]
                && self.marked[i + (2 * 5)]
                && self.marked[i + (3 * 5)]
                && self.marked[i + (4 * 5)];
            let row_win = self.marked[(i * 5) + 0]
                && self.marked[(i * 5) + 1]
                && self.marked[(i * 5) + 2]
                && self.marked[(i * 5) + 3]
                && self.marked[(i * 5) + 4];
            if row_win || col_win {
                win = true;
                break;
            }
        }

        if !win {
            return None;
        }

        self.won = true;
        let mut score = 0u32;
        for (i, marked) in self.marked.iter().enumerate() {
            if !marked {
                score += self.grid[i];
            }
        }

        Some(score)
    }
}

pub fn solve(file_input: File) -> Result<(i32, i32), &'static str> {
    let mut reader = BufReader::new(file_input);
    let mut line = String::new();

    reader
        .read_line(&mut line)
        .map_err(|_| "Error reading input file")?;
    let numbers_iter = line.trim().split(',');
    let mut numbers: Vec<u32> = Vec::with_capacity(numbers_iter.clone().count());
    for num_str in numbers_iter {
        numbers.push(u32::from_str_radix(num_str, 10).map_err(|_| "Error parsing string to int")?);
    }

    line.clear();

    let mut boards = Vec::new();
    while let Some(board) = BingoBoard::new(&mut reader) {
        boards.push(board);
    }

    let mut part1 = 0u32;
    let mut part2 = 0u32;
    let mut part1_found = false;
    let mut boards_remaining = boards.len();
    'game: for number in numbers {
        for board in boards.iter_mut().filter(|b| !b.won) {
            board.mark(number);
            if let Some(score) = board.check_win() {
                if !part1_found {
                    part1 = score * number;
                    part1_found = true;
                }

                if boards_remaining == 1 {
                    part2 = score * number;
                    break 'game;
                }

                boards_remaining -= 1;
            }
        }
    }

    Ok((part1 as i32, part2 as i32))
}
