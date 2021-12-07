use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct LineSegment {
    start: Point,
    end: Point,
}

fn parse_point(mut input: std::str::Split<&str>) -> Result<Point, &'static str> {
    Ok(Point {
        x: input
            .next()
            .ok_or("bad input")?
            .parse()
            .map_err(|_| "bad input")?,
        y: input
            .next()
            .ok_or("bad input")?
            .parse()
            .map_err(|_| "bad input")?,
    })
}

pub fn solve(file_input: File) -> Result<(i64, i64), &'static str> {
    let mut reader = BufReader::new(file_input);
    let mut input_line = String::new();

    let mut lines: Vec<LineSegment> = Vec::with_capacity(1000);
    // let mut grid_min = Point {
    //     x: i32::MAX,
    //     y: i32::MAX,
    // };
    let mut grid_max = Point { x: 0, y: 0 };
    while reader
        .read_line(&mut input_line)
        .map_err(|_| "Error reading input file")?
        > 0
    {
        let mut iter = input_line.trim().split(" -> ");
        let start = iter.next().ok_or("bad input")?.split(",");
        let end = iter.next().ok_or("bad input")?.split(",");

        let start = parse_point(start)?;
        let end = parse_point(end)?;
        // grid_min.x = grid_min.x.min(start.x).min(end.x);
        // grid_min.y = grid_min.y.min(start.y).min(end.y);
        grid_max.x = grid_max.x.max(start.x).max(end.x);
        grid_max.y = grid_max.y.max(start.y).max(end.y);

        lines.push(LineSegment { start, end });

        input_line.clear();
    }
    // println!("max = {:?}, min = {:?}", grid_max, grid_min);

    // let stride = grid_max.x - grid_min.x;
    let stride = grid_max.x + 1;
    // let mut grid: Vec<i32> = vec![0; ((max.x - min.x + 1) * (max.y - min.y + 1)) as usize];
    let mut grid: Vec<i32> = vec![0; ((grid_max.x + 1) * (grid_max.y + 1)) as usize];
    for line in lines {
        if line.start.x != line.end.x && line.start.y != line.end.y {
            continue;
        }

        let x_start = line.start.x.min(line.end.x);
        let x_end = line.start.x.max(line.end.x);
        if x_start != x_end {
            for x in x_start..=x_end {
                grid[(x + (line.start.y * stride)) as usize] += 1;
            }
        }

        let y_start = line.start.y.min(line.end.y);
        let y_end = line.start.y.max(line.end.y);
        if y_start != y_end {
            for y in y_start..=y_end {
                grid[(line.start.x + (y * stride)) as usize] += 1;
            }
        }
    }

    let part1 = grid.iter().filter(|x| **x > 1).count() as i64;

    // for line in lines {
    //     println!(
    //         "{},{} -> {},{}",
    //         line.start.x, line.start.y, line.end.x, line.end.y
    //     );
    // }

    // let part1 = 0i64;
    let part2 = 0i64;

    Ok((part1, part2))
}
