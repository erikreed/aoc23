use std::collections::BTreeSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(File::open("d4-p1.txt")?);

    let re = Regex::new(r"\d+").unwrap();

    let get_numbers = |s: &str| -> BTreeSet<_> {
        re.find_iter(s)
            .map(|s| s.as_str().parse::<u32>().unwrap())
            .collect()
    };

    let mut part1_sum = 0;
    let mut part2_sum = 0;
    let mut mart2_counts: Vec<u32> = vec![1; 1000];

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        let (set1, set2) = line.split_once(':').unwrap().1
            .split_once('|').unwrap();
        let (set1, set2) = (get_numbers(&set1), get_numbers(&set2));
        let overlap: Vec<_> = set1.intersection(&set2).collect();

        if !overlap.is_empty() {
            part1_sum += 2_u32.pow(overlap.len() as u32 - 1);
        }
        for j in 1..=overlap.len() {
            mart2_counts[i + j] += mart2_counts[i];
        }
        part2_sum += mart2_counts[i];
    }

    println!("part 1 sum: {part1_sum}");
    // 128374 too low
    println!("part 2 sum: {part2_sum}");
    return Ok(());
}