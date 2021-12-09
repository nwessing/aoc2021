use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Copy, Clone)]
struct SevenSegmentDisplay {
    segments: [bool; 7],
    lit_segment_count: u32,
}

impl SevenSegmentDisplay {
    fn new() -> Self {
        Self {
            segments: [false; 7],
            lit_segment_count: 0,
        }
    }
}

struct SevenSegmentDisplayCollection {
    displays: [SevenSegmentDisplay; 10],
    to_decipher: [SevenSegmentDisplay; 4],
}

impl SevenSegmentDisplayCollection {
    fn new() -> Self {
        Self {
            displays: [SevenSegmentDisplay::new(); 10],
            to_decipher: [SevenSegmentDisplay::new(); 4],
        }
    }
}

//   0:      1:      2:      3:      4:
//  aaaa    ....    aaaa    aaaa    ....
// b    c  .    c  .    c  .    c  b    c
// b    c  .    c  .    c  .    c  b    c
//  ....    ....    dddd    dddd    dddd
// e    f  .    f  e    .  .    f  .    f
// e    f  .    f  e    .  .    f  .    f
//  gggg    ....    gggg    gggg    ....
//
//   5:      6:      7:      8:      9:
//  aaaa    aaaa    aaaa    aaaa    aaaa
// b    .  b    .  .    c  b    c  b    c
// b    .  b    .  .    c  b    c  b    c
//  dddd    dddd    ....    dddd    dddd
// .    f  e    f  .    f  e    f  .    f
// .    f  e    f  .    f  e    f  .    f
//  gggg    gggg    ....    gggg    gggg
//  a appears 8 times
//  b appears 6 times
//  c appears 8 times
//  d appears 7 times
//  e appears 4 times
//  f appears 9 times
//  g appears 7 times

pub fn solve(file_input: File) -> Result<(i64, i64), &'static str> {
    let mut reader = BufReader::new(file_input);
    let mut line = String::new();

    let mut collections: Vec<SevenSegmentDisplayCollection> = Vec::with_capacity(1000);

    while reader
        .read_line(&mut line)
        .map_err(|_| "Error parsing line")?
        != 0
    {
        let mut input_iter = line.trim().split('|');
        let digits = get_digit_list(&mut input_iter)?;
        let to_decipher = get_digit_list(&mut input_iter)?;

        let mut collection = SevenSegmentDisplayCollection::new();
        for (i, digit) in digits.enumerate() {
            parse_displays(&mut collection.displays[i], digit)?;
        }
        for (i, digit) in to_decipher.enumerate() {
            parse_displays(&mut collection.to_decipher[i], digit)?;
        }

        collections.push(collection);
        line.clear();
    }

    let number_definitions: Vec<SevenSegmentDisplay> = [
        "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
    ]
    .iter()
    .map(|segment_str| {
        let mut result = SevenSegmentDisplay::new();
        parse_displays(&mut result, segment_str).unwrap();
        return result;
    })
    .collect();

    let mut part1 = 0i64;
    let mut part2 = 0i64;
    for collection in collections.iter() {
        let mut segment_counts = [0u32; 7];
        for display in collection.displays.iter() {
            for (i, segment) in display.segments.iter().enumerate() {
                if *segment {
                    segment_counts[i as usize] += 1;
                }
            }
        }

        let b = get_segment_by_count(&segment_counts, |_, count| count == 6);
        let e = get_segment_by_count(&segment_counts, |_, count| count == 4);
        let f = get_segment_by_count(&segment_counts, |_, count| count == 9);

        let one = get_digit(&collection, |digit| digit.lit_segment_count == 2);
        let seven = get_digit(&collection, |digit| digit.lit_segment_count == 3);
        let a = get_different_segment(&one.segments, &seven.segments);

        let c = get_segment_by_count(&segment_counts, |i, count| i != a as usize && count == 8);
        let four = get_digit(&collection, |digit| {
            digit.lit_segment_count == 4
                && digit.segments[b as usize]
                && digit.segments[c as usize]
                && digit.segments[f as usize]
        });

        let mut d = 0u32;
        for (i, digit) in four.segments.iter().enumerate() {
            let i = i as u32;
            if *digit && i != b && i != c && i != f {
                d = i as u32;
            }
        }

        let g = (6 + 5 + 4 + 3 + 2 + 1 + 0) - (a + b + c + d + e + f);

        let remap = [a, b, c, d, e, f, g];
        let mut multiplier = 10i64.pow(collection.to_decipher.len() as u32 - 1);
        let mut solution: i64 = 0;

        for to_solve in collection.to_decipher {
            let lit_segment_count = to_solve.lit_segment_count;
            if lit_segment_count == 7
                || lit_segment_count == 2
                || lit_segment_count == 3
                || lit_segment_count == 4
            {
                part1 += 1;
            }

            'next_number: for (number, definition) in number_definitions.iter().enumerate() {
                for i in 0..7 {
                    if to_solve.segments[remap[i] as usize] != definition.segments[i] {
                        continue 'next_number;
                    }
                }
                solution += number as i64 * multiplier;
                multiplier /= 10;
            }
        }
        part2 += solution;
    }

    Ok((part1, part2))
}

fn get_segment_by_count<P>(segment_counts: &[u32; 7], predicate: P) -> u32
where
    P: Fn(usize, u32) -> bool,
{
    segment_counts
        .iter()
        .enumerate()
        .find_map(|(i, x)| {
            if predicate(i, *x) {
                Some(i as u32)
            } else {
                None
            }
        })
        .unwrap()
}

fn get_different_segment(a: &[bool; 7], b: &[bool; 7]) -> u32 {
    for i in 0..7 {
        if a[i] != b[i] {
            return i as u32;
        }
    }

    panic!("No difference found!");
}

fn convert_char(character: char) -> u32 {
    character as u32 - 'a' as u32
}

fn get_digit_list<'a>(
    input_iter: &mut std::str::Split<'a, char>,
) -> Result<std::str::Split<'a, char>, &'static str> {
    Ok(input_iter
        .next()
        .ok_or("Invalid input format")?
        .trim()
        .split(' '))
}

fn get_digit<P>(collection: &SevenSegmentDisplayCollection, predicate: P) -> &SevenSegmentDisplay
where
    P: FnMut(&&SevenSegmentDisplay) -> bool,
{
    collection.displays.iter().find(predicate).unwrap()
}

fn parse_displays(display: &mut SevenSegmentDisplay, digits: &str) -> Result<(), &'static str> {
    let mut lit_segment_count = 0u32;
    for character in digits.chars() {
        display.segments[convert_char(character) as usize] = true;
        lit_segment_count += 1;
    }
    display.lit_segment_count = lit_segment_count;
    Ok(())
}
