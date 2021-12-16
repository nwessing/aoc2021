use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Point(i32, i32);

#[derive(Debug)]
enum Fold {
    X(i32),
    Y(i32),
}

fn fold_value(value: i32, axis: i32) -> i32 {
    if value > axis {
        return axis - (value - axis);
    }

    return value;
}
pub fn solve(file_input: File) -> Result<(i64, i64), &'static str> {
    let (mut points, folds) = read_input(file_input)?;

    let mut width = 0i32;
    let mut height = 0i32;
    for point in points.iter() {
        width = point.0.max(width);
        height = point.1.max(height);
    }

    let mut counts: HashSet<Point> = HashSet::with_capacity(1000);
    let mut part1 = 0i64;
    for (i, fold) in folds.iter().enumerate() {
        match fold {
            Fold::X(_) => {
                width /= 2;
            }
            Fold::Y(_) => {
                height /= 2;
            }
        }
        for point in points.iter_mut() {
            match fold {
                Fold::X(amount) => {
                    point.0 = fold_value(point.0, *amount);
                }
                Fold::Y(amount) => {
                    point.1 = fold_value(point.1, *amount);
                }
            }
        }

        if i == 0 {
            for point in points.iter() {
                counts.insert(*point);
            }
            part1 = counts.len() as i64;
        }
    }

    counts.clear();
    for point in points.iter() {
        counts.insert(*point);
    }

    for y in 0..height {
        for x in 0..width {
            print!(
                "{}",
                if counts.contains(&Point(x, y)) {
                    '#'
                } else {
                    '.'
                }
            );
        }
        println!("");
    }

    Ok((part1, -123456789i64))
}

fn read_input(file_input: File) -> Result<(Vec<Point>, Vec<Fold>), &'static str> {
    let mut reader = BufReader::new(file_input);
    let mut line = String::new();

    let mut points: Vec<Point> = Vec::with_capacity(1000);
    let mut folds: Vec<Fold> = Vec::with_capacity(100);

    while reader
        .read_line(&mut line)
        .map_err(|_| "Error parsing line")?
        != 0
    {
        if line.trim().len() == 0 {
            break;
        }

        let mut iter = line.trim().split(',');
        let point = Point(
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
        );
        points.push(point);

        line.clear();
    }

    line.clear();
    while reader
        .read_line(&mut line)
        .map_err(|_| "Error parsing line")?
        != 0
    {
        let mut iter = line.trim().split(' ');
        iter.next();
        iter.next();
        let mut iter = iter.next().unwrap().split('=');
        let axis = iter.next().unwrap();
        let number: i32 = iter.next().unwrap().parse().unwrap();

        let fold = match axis {
            "x" => Fold::X(number),
            "y" => Fold::Y(number),
            _ => {
                panic!("Unexpected axis");
            }
        };

        folds.push(fold);

        line.clear();
    }

    Ok((points, folds))
}
