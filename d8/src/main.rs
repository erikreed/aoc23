use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(File::open("d8-p1.txt")?);
    let mut lines = reader.lines();

    let sequence = lines.next().unwrap()?;
    lines.next();

    // AAA = (BBB, BBB)
    let re = Regex::new(r"([A-Z\d]+) = \(([A-Z\d]+), ([A-Z\d]+)\)").unwrap();
    let mut paths = HashMap::new();

    for line in lines {
        let line = line?;
        let parts: Vec<_> = re.captures_iter(&line)
            .map(|c| c.extract::<3>())
            .flat_map(|c| c.1)
            .collect();
        paths.insert(parts[0].to_string(), (parts[1].to_string(), parts[2].to_string()));
    }

    let mut current = "AAA";
    let mut i = 0;

    while current != "ZZZ" {
        let step = sequence.as_bytes()[i % sequence.len()] as char;
        current = &paths.get(current)
            .map(|(l, r)| if step == 'L' { l } else { r })
            .unwrap();

        i += 1;
    }

    println!("part1: {i}");

    let mut origins: Vec<_> = paths.keys()
        .filter(|&s| s.ends_with('A'))
        .collect();

    let cycle_lengths: Vec<_> = origins.iter_mut().map(|current| {
        let mut i = 0;

        while !current.ends_with('Z') {
            let step = sequence.as_bytes()[i % sequence.len()] as char;
            *current = &paths.get(*current)
                .map(|(l, r)| if step == 'L' { l } else { r })
                .unwrap();

            i += 1;
        }
        return i;
    }).collect();
    dbg!(&cycle_lengths);

    let gcd: u64 = cycle_lengths.iter().fold(1, |a, b| lcm(a, *b as u64));

    println!("part2: {gcd}");

    return Ok(());
}

pub fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}

pub fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

