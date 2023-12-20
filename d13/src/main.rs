use std::cmp::min;
use std::error::Error;
use std::fs;
use std::io::BufRead;

fn check_mirrored(row: &Vec<i32>) -> Option<usize> {
    let len = row.len();
    for i in 1..len {
        let mut sub1 = (&row[..i]).to_vec();
        let mut sub2 = (&row[i..]).to_vec();

        let minlen = min(sub1.len(), sub2.len());
        sub1.reverse();

        if &sub1[..minlen] == &sub2[..minlen] {
            return Some(i);
        }
    }
    return None;
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string("d13.txt")?;

    let mut part1_sum = 0;
    let mut part2_sum = 0;

    for grid in file.split("\n\n") {
        println!("\n{grid}\n");

        let mut grid_mat = vec![];
        for line in grid.lines() {
            grid_mat.push(line.chars()
                .map(|c| c == '#')
                .collect::<Vec<_>>()
            );
        }

        let (rows, cols) = get_row_cols(&grid_mat);
        let sum = sum_checks(&rows, &cols);

        part1_sum += sum;
        dbg!(sum);

        let mut found = false;
        for i in 0..rows.len() {
            if found {
                break;
            }
            for j in 0..cols.len() {
                grid_mat[i][j] = !grid_mat[i][j];
                let (rows, cols) = get_row_cols(&grid_mat);
                grid_mat[i][j] = !grid_mat[i][j];

                let sum2 = sum_checks(&rows, &cols);
                if sum2 > 0 && sum2 != sum {
                    part2_sum += sum2;
                    found = true;
                    dbg!(sum2);
                    break;
                }
            }
        }
        assert!(found);
    }

    println!("part1_sum: {part1_sum}");
    println!("part2_sum: {part2_sum}");
    return Ok(());
}

fn sum_checks(rows: &Vec<i32>, cols: &Vec<i32>) -> usize {
    let row_check = check_mirrored(&rows);
    let col_check = check_mirrored(&cols);

    let mut sum = 0;
    sum += col_check.unwrap_or(0);
    sum += row_check.unwrap_or(0) * 100;
    assert!(!(col_check.is_some() && row_check.is_some()));

    sum
}

fn get_row_cols(grid_mat: &Vec<Vec<bool>>) -> (Vec<i32>, Vec<i32>) {
    let ncols = grid_mat[0].len();
    let nrows = grid_mat.len();
    let rows = (0..nrows)
        .map(|i| {
            let mut sum = 0;
            for j in 0..ncols {
                sum = sum << 1;
                sum += grid_mat[i][j] as i32;
            }
            sum
        })
        .collect::<Vec<_>>();

    let cols = (0..ncols)
        .map(|j| {
            let mut sum = 0;
            for i in 0..nrows {
                sum = sum << 1;
                sum += grid_mat[i][j] as i32;
            }
            sum
        })
        .collect::<Vec<_>>();
    (rows, cols)
}

