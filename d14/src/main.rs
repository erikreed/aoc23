use std::cmp::min;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::mem::swap;

fn main() -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(File::open("d14.txt")?);

    let mut grid = vec![];

    for line in reader.lines() {
        let line = line?;
        grid.push(line.chars().collect::<Vec<_>>());
    }

    let mut changed = true;
    let mut iters = 0;
    while changed {
        changed = false;
        iters += 1;

        for j in 0..grid[0].len() {
            for i in 1..grid.len() {
                let c = grid[i][j];
                let upper = grid[i - 1][j];
                if c == 'O' && upper == '.' {
                    grid[i][j] = '.';
                    grid[i - 1][j] = 'O';
                    changed = true;
                }
            }
        }
    }
    dbg!(iters);
    let mut part1_sum = 0;
    for j in 0..grid[0].len() {
        for i in 0..grid.len() {
            if grid[i][j] == 'O' {
                part1_sum += grid.len() - i
            }
        }
    }
    dbg!(part1_sum);

    return Ok(());
}
