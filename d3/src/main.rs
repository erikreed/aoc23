use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fs;

use regex::Regex;

#[derive(Eq, Ord, PartialOrd, PartialEq)]
struct Symbol {
    x: i32,
    y: i32,
    c: char,
}

fn check_symbol(lines: &Vec<&str>, x: i32, y: i32) -> Option<Symbol> {
    let rows = lines.len() as i32;
    let cols = lines[0].len() as i32;

    for i in -1..=1 {
        for j in -1..=1 {
            if x + i < 0 || x + i >= rows || y + j < 0 || y + j >= cols {
                continue;
            }
            let c = char::from(lines[(x + i) as usize].as_bytes()[(y + j) as usize]);
            if !c.is_ascii_digit() && c != '.' {
                return Some(Symbol { x: x + i, y: y + j, c });
            }
        }
    }
    return None;
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string("d3-p1.txt")?;
    let lines: Vec<_> = file
        .trim()
        .split('\n')
        .collect();


    let mut part1_sum = 0;
    let mut part2_sum = 0;
    let re = Regex::new(r"\d+").unwrap();
    let mut gear_to_adjacent_number: BTreeMap<Symbol, i32> = BTreeMap::new();

    for (i, &line) in lines.iter().enumerate() {
        for number_match in re.find_iter(line) {
            let number: i32 = number_match.as_str().parse()?;
            let symbol_adjacent = (number_match.start()..number_match.end())
                .into_iter()
                .any(|y| check_symbol(&lines, i as i32, y as i32).is_some());
            if symbol_adjacent {
                part1_sum += number;
            }

            let gears: BTreeSet<Symbol> = (number_match.start()..number_match.end())
                .into_iter()
                .filter_map(|y| check_symbol(&lines, i as i32, y as i32))
                .filter(|s| s.c == '*')
                .collect();
            for gear in gears {
                match gear_to_adjacent_number.get(&gear) {
                    Some(adjacent) => {
                        part2_sum += number * adjacent;
                    },
                    None => {
                        gear_to_adjacent_number.insert(gear, number);
                    }
                }

            }
        }
    }

    println!("part 1 sum: {part1_sum}");
    println!("part 2 sum: {part2_sum}");
    return Ok(());
}