use std::cmp::{max, min};
use std::collections::HashSet;
use itertools::Itertools;

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Coord = (i32, i32);

fn main() -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(File::open("d11-p1.txt")?);

    let mut grid: Vec<Coord> = vec![];

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        line.chars()
            .enumerate()
            .filter(|&(j, c)| c == '#')
            .for_each(|(j, _)| grid.push((i as i32, j as i32)));
    }

    let mut occupied_rows: Vec<_> = grid.iter().map(|&(x, y)| x).collect();
    occupied_rows.dedup();
    let empty_rows: HashSet<_> = (0..=*occupied_rows.last().unwrap())
        .filter(|i| !occupied_rows.contains(i))
        .collect();

    let mut occupied_cols: Vec<_> = grid.iter().map(|&(x, y)| y).collect();
    occupied_cols.sort();
    occupied_cols.dedup();
    let empty_cols: HashSet<_> = (0..=*occupied_cols.last().unwrap())
        .filter(|i| !occupied_cols.contains(i))
        .collect();

    let mut part1_sum = 0;
    let mut part2_sum: u64 = 0;

    for pair in grid.iter().combinations(2) {
        let (&a, &b) = pair.iter().next_tuple().unwrap();
        let (x1, y1) = *a;
        let (x2, y2) = *b;

        let max_x = max(x1, x2);
        let min_x = min(x1, x2);
        let max_y = max(y1, y2);
        let min_y = min(y1, y2);

        let dy = max_y - min_y;
        let dx = max_x - min_x;

        let doubles_x = (min_x + 1..max_x)
            .filter(|i| empty_rows.contains(i))
            .count() as i32;

        let doubles_y = (min_y + 1..max_y)
            .filter(|i| empty_cols.contains(i))
            .count() as i32;

        let l1_distance = dx + dy + doubles_x + doubles_y;
        part1_sum += l1_distance;
        part2_sum += (dx + dy) as u64 + (doubles_x + doubles_y) as u64 * (1e6 as u64 - 1);
    }

    println!("part1_sum: {part1_sum}");
    println!("part2_sum: {part2_sum}");

    return Ok(());
}

