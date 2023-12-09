use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(File::open("d9-p1.txt")?);

    let mut part1_sum = 0;
    let mut part2_sum = 0;

    for line in reader.lines() {
        let mut numbers: Vec<i32> = line?.split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let mut first_diffs = vec![numbers[0]];
        let mut last_diffs = vec![*numbers.last().unwrap()];

        loop {
            let diffs: Vec<i32> = numbers.iter()
                .zip(numbers.iter().skip(1))
                .map(|(a, b)| b - a)
                .collect();
            first_diffs.push(diffs[0]);
            last_diffs.push(*diffs.last().unwrap());

            if diffs.iter().all(|&e| e == 0) {
                break;
            }
            numbers = diffs;
        }
        let mut next = 0;
        for &d in last_diffs.iter().rev() {
            next = d + next;
        }
        part1_sum += next;

        let mut next = 0;
        for &d in first_diffs.iter().rev() {
            next = d - next;
        }
        part2_sum += next;
    }

    println!("part1_sum: {part1_sum}");
    println!("part2_sum: {part2_sum}");

    return Ok(());
}

