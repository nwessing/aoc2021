use std::cmp::Ordering;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct LineSegment {
    start: Point,
    end: Point,
}

#[derive(Clone)]
struct IntersectionTracker {
    part1: i32,
    part2: i32,
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
        grid_max.x = grid_max.x.max(start.x).max(end.x);
        grid_max.y = grid_max.y.max(start.y).max(end.y);

        lines.push(LineSegment { start, end });

        input_line.clear();
    }

    let stride = grid_max.x + 1;
    let mut grid: Vec<IntersectionTracker> = vec![
        IntersectionTracker { part1: 0, part2: 0 };
        ((grid_max.x + 1) * (grid_max.y + 1)) as usize
    ];
    for line in lines {
        let dx = match line.start.x.cmp(&line.end.x) {
            Ordering::Greater => -1,
            Ordering::Less => 1,
            Ordering::Equal => 0,
        };
        let dy = match line.start.y.cmp(&line.end.y) {
            Ordering::Greater => -1,
            Ordering::Less => 1,
            Ordering::Equal => 0,
        };

        let mut point = line.start.clone();
        loop {
            let tracker = &mut grid[(point.x + (point.y * stride)) as usize];
            tracker.part2 += 1;
            if !(dx != 0 && dy != 0) {
                tracker.part1 += 1;
            }

            if point == line.end {
                break;
            }
            point.x += dx;
            point.y += dy;

            if point.x > grid_max.x || point.y > grid_max.y || point.x < 0 || point.y < 0 {
                break;
            }
        }
    }

    let mut part1 = 0i64;
    let mut part2 = 0i64;
    for tracker in grid.iter_mut() {
        if tracker.part1 > 1 {
            part1 += 1;
        }
        if tracker.part2 > 1 {
            part2 += 1;
        }
    }
    Ok((part1, part2))
}
