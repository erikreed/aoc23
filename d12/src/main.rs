use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::SystemTime;

use indicatif::ParallelProgressIterator;
use rayon::prelude::*;

#[derive(Clone, Copy, PartialEq)]
enum Value { ON, OFF, UNKNOWN }

const PART1: bool = false;

fn is_valid(values: &Vec<Value>, combos: &Vec<u32>,
            start_i: usize, start_j: usize) -> (bool, usize, usize) {
    let (mut i, mut j) = (start_i, start_j);
    let mut combos_iter = combos[j..].iter();
    let mut combo = combos_iter.next();

    let mut contiguous_count: u32 = 0;
    let mut last = &Value::OFF;
    let mut next_i = i;

    for c in &values[i..] {
        i += 1;
        match c {
            Value::OFF => {
                if c == last {
                    next_i = i - 1;
                    continue;
                }

                if *combo.unwrap_or(&0) != contiguous_count {
                    return (false, next_i, j);
                }
                combo = combos_iter.next();
                j += 1;
                next_i = i - 1;
                contiguous_count = 0;
            }
            Value::ON => {
                contiguous_count += 1;
            }
            Value::UNKNOWN => {
                return (true, next_i, j);
            }
        }
        last = c;
    }

    let valid = *combo.unwrap_or(&0) == contiguous_count && combos_iter.next().is_none();
    return (valid, i, j);
}

fn main() -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(File::open("d12.txt")?);
    let lines = reader.lines().collect::<Vec<_>>();
    let part1_sum: u64 = lines
        .par_iter()
        .progress_count(lines.len() as u64)
        .map(|line| {
            let start = SystemTime::now();
            let line = line.as_ref().unwrap();

            let (p1, p2) = line.split_once(' ').unwrap();

            let mut values = p1.chars().map(|c| {
                match c {
                    '.' => Value::OFF,
                    '#' => Value::ON,
                    '?' => Value::UNKNOWN,
                    _ => panic!("malformed")
                }
            }).collect::<Vec<_>>();

            let mut combos = p2.split(',').map(|c| c.parse::<u32>().unwrap()).collect::<Vec<_>>();

            if !PART1 {
                values.push(Value::UNKNOWN);
                values = values.repeat(5);
                values.pop();
                combos = combos.repeat(5);
            }

            let mut unknown_idx = values.iter()
                .enumerate()
                .filter(|(_, v)| **v == Value::UNKNOWN)
                .map(|(i, _)| i)
                .collect::<Vec<usize>>();
            unknown_idx.reverse();

            let count = count_permutations(&mut values, &combos, &mut unknown_idx, 0, 0);

            dbg!(line);
            println!("count: {} in {:.2}s", count, start.elapsed().unwrap().as_secs_f32());
            count
        }).sum();

    println!("part1 sum: {part1_sum}");

    return Ok(());
}


#[inline(always)]
fn count_permutations(values: &mut Vec<Value>,
                      combos: &Vec<u32>,
                      unknown_idx: &mut Vec<usize>,
                      start_i: usize, start_j: usize) -> u64 {
    let (valid, start_i, start_j) = is_valid(&values, &combos, start_i, start_j);
    if !valid {
        return 0;
    }
    if unknown_idx.is_empty() {
        return 1;
    }

    let mut count = 0;

    let next_idx = unknown_idx.pop().unwrap();
    for v in [Value::ON, Value::OFF] {
        values[next_idx] = v;
        count += count_permutations(values, combos, unknown_idx, start_i, start_j);
    }
    values[next_idx] = Value::UNKNOWN;
    unknown_idx.push(next_idx);

    return count;
}

