use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, PartialEq, Eq)]
enum CaveType {
    Start,
    End,
    Leaf,
    Large,
}

#[derive(Debug)]
struct Cave {
    name: String,
    portals: Vec<usize>,
    ctype: CaveType,
}

struct Path {
    visited: Vec<i32>,
    has_double_visited_small_room: bool,
}

pub fn solve(file_input: File) -> Result<(i64, i64), &'static str> {
    let caves = parse_input(file_input)?;

    let start = caves
        .iter()
        .enumerate()
        .find_map(|(i, cave)| {
            if cave.ctype == CaveType::Start {
                return Some(i);
            }
            None
        })
        .unwrap();

    let mut path = Path {
        visited: vec![0; caves.len()],
        has_double_visited_small_room: false,
    };

    let result = find_paths(&caves, &mut path, start);

    Ok((result.part1, result.part2))
}

struct Answer {
    part1: i64,
    part2: i64,
}

impl std::ops::Add<Answer> for Answer {
    type Output = Answer;

    fn add(self, rhs: Self) -> Answer {
        Answer {
            part1: self.part1 + rhs.part1,
            part2: self.part2 + rhs.part2,
        }
    }
}
impl std::ops::AddAssign<Answer> for Answer {
    fn add_assign(&mut self, rhs: Self) {
        self.part1 += rhs.part1;
        self.part2 += rhs.part2;
    }
}

fn find_paths(caves: &[Cave], current_path: &mut Path, from: usize) -> Answer {
    current_path.visited[from] += 1;
    let current = &caves[from];

    let mut result = Answer { part1: 0, part2: 0 };
    for portal in current.portals.iter() {
        let next = &caves[*portal];
        match next.ctype {
            CaveType::Leaf => {
                if current_path.visited[*portal] == 0 {
                    result += find_paths(caves, current_path, *portal);
                } else if !current_path.has_double_visited_small_room {
                    current_path.has_double_visited_small_room = true;
                    result.part2 += find_paths(caves, current_path, *portal).part2;
                    current_path.has_double_visited_small_room = false;
                }
            }
            CaveType::End => {
                result.part1 += 1;
                result.part2 += 1;
            }
            CaveType::Start => {}
            CaveType::Large => {
                result += find_paths(caves, current_path, *portal);
            }
        }
    }

    current_path.visited[from] -= 1;
    return result;
}

fn parse_input(file_input: File) -> Result<Vec<Cave>, &'static str> {
    let mut reader = BufReader::new(file_input);
    let mut line = String::new();

    let mut caves: Vec<Cave> = Vec::with_capacity(100);

    while reader
        .read_line(&mut line)
        .map_err(|_| "Error parsing line")?
        != 0
    {
        let mut parts = line.trim().split('-');
        let cave1 = parts.next().ok_or("Bad input")?;
        let cave2 = parts.next().ok_or("Bad input")?;

        let enter = add_cave(&mut caves, cave1);
        let exit = add_cave(&mut caves, cave2);

        caves[enter].portals.push(exit);
        caves[exit].portals.push(enter);

        line.clear();
    }
    Ok(caves)
}

fn get_cave_type(name: &str) -> CaveType {
    match name {
        "start" => CaveType::Start,
        "end" => CaveType::End,
        _ => {
            if name.chars().next().unwrap().is_uppercase() {
                CaveType::Large
            } else {
                CaveType::Leaf
            }
        }
    }
}

fn add_cave(caves: &mut Vec<Cave>, name: &str) -> usize {
    for (i, cave) in caves.iter_mut().enumerate() {
        if cave.name == name {
            return i;
        }
    }

    let result = caves.len();
    caves.push(Cave {
        name: name.to_string(),
        ctype: get_cave_type(name),
        portals: Vec::with_capacity(20),
    });

    return result;
}
