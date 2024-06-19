use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};

const VERBOSE: bool = false;

fn main() -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(File::open("d14.txt")?);

    let mut grid = vec![];

    for line in reader.lines() {
        let line = line?;
        grid.push(line.chars().collect::<Vec<_>>());
    }

    let mut grid2 = grid.clone();

    let iters = settle_grid(&mut grid, Direction::N);
    dbg!(iters);

    let part1_sum = sum_grid(&grid);
    dbg!(part1_sum);


    let mut cycle_to_sum = HashMap::new();
    cycle_to_sum.insert(0, part1_sum);

    let mut hash_to_cycle = HashMap::new();

    let mut found_cycle = -1;
    let mut first_cycle_idx = -1;

    for cycle_idx in 1..1000 {
        for dir in [Direction::N, Direction::W, Direction::S, Direction::E] {
            settle_grid(&mut grid2, dir);
        }

        if VERBOSE {
            println!("\n\n===   {cycle_idx}   ===\n");
            grid2.iter().for_each(|r| println!("{}", String::from_iter(r)));
            println!();
        }

        let mut hasher = DefaultHasher::new();
        grid2.hash(&mut hasher);
        let sum = sum_grid(&grid2);
        cycle_to_sum.insert(cycle_idx, sum);


        let existing = hash_to_cycle.insert(hasher.finish(), cycle_idx);
        if existing.is_some() {
            found_cycle = dbg!(cycle_idx);
            first_cycle_idx = dbg!(existing.unwrap());
            break;
        }
    }
    assert_ne!(found_cycle, -1);
    assert_ne!(first_cycle_idx, -1);

    let part2_sum = cycle_to_sum.get(&(1e9 as i32 % (found_cycle - first_cycle_idx))).unwrap();
    dbg!(part2_sum);
    return Ok(());
}

fn sum_grid(grid: &Vec<Vec<char>>) -> u32 {
    let mut sum = 0;
    for j in 0..grid[0].len() {
        for i in 0..grid.len() {
            if grid[i][j] == 'O' {
                sum += (grid.len() - i) as u32
            }
        }
    }
    sum
}


enum Direction { N, S, E, W }

fn settle_grid(grid: &mut Vec<Vec<char>>, direction: Direction) -> i32 {
    let mut changed = true;
    let mut iters = 0;
    while changed {
        changed = false;
        iters += 1;

        match direction {
            Direction::N => {
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
            Direction::S => {
                for j in 0..grid[0].len() {
                    for i in (0..grid.len() - 1).rev() {
                        let c = grid[i][j];
                        let lower = grid[i + 1][j];
                        if c == 'O' && lower == '.' {
                            grid[i][j] = '.';
                            grid[i + 1][j] = 'O';
                            changed = true;
                        }
                    }
                }
            }

            Direction::E => {
                for i in 0..grid.len() {
                    for j in (0..grid[0].len() - 1).rev() {
                        let c = grid[i][j];
                        let upper = grid[i][j + 1];
                        if c == 'O' && upper == '.' {
                            grid[i][j] = '.';
                            grid[i][j + 1] = 'O';
                            changed = true;
                        }
                    }
                }
            }
            Direction::W => {
                for i in 0..grid.len() {
                    for j in 1..grid[0].len() {
                        let c = grid[i][j];
                        let upper = grid[i][j - 1];
                        if c == 'O' && upper == '.' {
                            grid[i][j] = '.';
                            grid[i][j - 1] = 'O';
                            changed = true;
                        }
                    }
                }
            }
        }
    }
    iters
}
