use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};

fn main() {
    let file = File::open("d1-1.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut part1_sum = 0;
    for line in reader.by_ref().lines() {
        let digits: Vec<_> = line.unwrap()
            .chars()
            .filter(|c| c.is_ascii_digit())
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        part1_sum += match digits.len() {
            1 => digits[0] * 10 + digits[0],
            _ => digits[0] * 10 + digits[digits.len() - 1],
        }
    }

    println!("part 1 sum: {part1_sum}");

    reader.seek(SeekFrom::Start(0)).unwrap();

    let digit_strings = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"
    ];
    let digit_map: HashMap<&&str, u32> = HashMap::from_iter(digit_strings.iter()
        .enumerate()
        .map(|(i, k)| (k, i as u32 + 1)));

    let mut part2_sum = 0;
    for line in reader.lines().map(|line| line.unwrap()) {
        let mut digits: Vec<_> = digit_strings.iter()
            .map(|s| (line.match_indices(s), *digit_map.get(s).unwrap() as u32))
            .flat_map(|(i, e)| i.map(move |ii| (ii, e)))
            .collect();
        digits.extend(
            (0..10).into_iter()
                .map(|i| (line.match_indices(char::from_digit(i, 10).unwrap()), i))
                .flat_map(|(i, e)| i.map(move |ii| (ii, e)))
        );
        digits.sort_by_key(|(i, _)| i.0);
        part2_sum += match digits.len() {
            1 => digits[0].1 * 10 + digits[0].1,
            _ => digits[0].1 * 10 + digits[digits.len() - 1].1,
        }
    }
    println!("part 2 sum: {part2_sum}");
}