use rayon::prelude::*;
use std::collections::BTreeMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_numbers(line: &str) -> Vec<i64> {
    line.trim().split(' ').map(|c| c.parse().unwrap()).collect()
}


// dest, start, lem
#[derive(Debug)]
struct Range(i64, i64, i64);


fn traverse_maps(ranges_x_to_y: &BTreeMap<(String, String), Vec<Range>>, mut current: i64) -> i64 {
    let order = [
        "seed", "soil", "fertilizer", "water", "light", "temperature", "humidity", "location"
    ];

    for (a, b) in order.iter().zip(order.iter().skip(1)) {
        let ranges = ranges_x_to_y.get(&(a.to_string(), b.to_string())).unwrap();
        let mapped = ranges.iter()
            .find(|r| r.1 <= current && r.1 + r.2 > current)
            .map(|r| current - r.1 + r.0)
            .unwrap_or(current);
        assert!(mapped >= 0);
        current = mapped;
    }
    current
}

fn main() -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(File::open("d5-p1.txt")?);
    let mut lines = reader.lines();

    let header = lines.next().unwrap()?;
    let (header, numbers) = header.split_once(':').unwrap();
    assert_eq!(header, "seeds");
    let seed_numbers = get_numbers(numbers);

    // x-to-y map
    let mut ranges_x_to_y: BTreeMap<(String, String), Vec<Range>> = BTreeMap::new();

    lines.next();
    loop {
        let line = lines.next();
        if line.is_none() {
            break;
        }
        let header = dbg!(line.unwrap()?);
        let lookup_xy = header.split_once(' ').unwrap().0
            .split_once("-to-")
            .unwrap();

        let mut ranges: Vec<Range> = vec![];
        loop {
            let line = lines.next().unwrap_or(Ok("".to_string())).unwrap();
            if line.is_empty() {
                break;
            }
            let numbers = get_numbers(&line);
            ranges.push(Range(numbers[0], numbers[1], numbers[2]));
        }
        ranges_x_to_y.insert((
                                 lookup_xy.0.to_string(), lookup_xy.1.to_string()
                             ), ranges);
    }

    let smallest = seed_numbers.iter().map(|seed| {
        traverse_maps(&ranges_x_to_y, *seed)
    }).min().unwrap();
    println!("part1 smallest: {smallest}");


    let smallest = seed_numbers.iter().step_by(2)
        .zip(seed_numbers.iter().skip(1).step_by(2))
        .collect::<Vec<_>>()
        .par_iter()
        .flat_map(|(&a, &b)| dbg!(a..a+b))
        .map(|seed| {
            traverse_maps(&ranges_x_to_y, seed)
        }).min().unwrap();
    println!("part2 smallest: {smallest}");


    return Ok(());
}
