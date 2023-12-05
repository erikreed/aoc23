use std::cmp::max;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(File::open("d2-1.txt")?);

    // 12 red cubes, 13 green cubes, and 14 blue cubes
    let max_counts = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);

    let mut part1_sum_valid_game_ids = 0;
    let mut part2_sum = 0;

    for line in reader.lines() {
        let line = line?;
        let game_id: i32 = line.split_once(':').unwrap().0
            .split_once(' ').unwrap().1
            .parse()?;

        let mut valid = true;
        let mut part2_max_counts = HashMap::new();

        for set in line.split(':').last().unwrap().split(';') {
            for color_pair in set.split(',') {
                let (count, color) = color_pair.trim().split_once(' ').unwrap();
                let count: i32 = count.parse()?;

                part2_max_counts.entry(color)
                    .and_modify(|e| { *e = max(*e, count) })
                    .or_insert(count);

                if max_counts.get(color).unwrap() < &count {
                    valid = false;
                }
            }
        }
        if valid {
            part1_sum_valid_game_ids += game_id;
        }
        part2_sum += part2_max_counts.values().fold(1, |a, b| a * b);
    }

    println!("part 1 sum: {part1_sum_valid_game_ids}");
    println!("part 2 sum: {part2_sum}");

    return Ok(());
}