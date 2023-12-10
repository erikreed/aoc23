use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::io::BufRead;

use rayon::prelude::*;

type Coord = (i32, i32);
type Path = Vec<Coord>;

fn main() -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string("d10-p1.txt")?;
    let charmat: Vec<_> = file.lines().map(|s| s.chars().collect::<Vec<_>>()).collect();
    let mut connectivity = HashMap::new();

    let mut bi_insert = |a: Coord, b: Coord| {
        connectivity.entry(a)
            .or_insert(vec![])
            .push(b);
        connectivity.entry(b)
            .or_insert(vec![])
            .push(a);
    };


    let mut origin = (-1, -1);

    for (x, line) in charmat.iter().enumerate() {
        for (y, &char) in line.iter().enumerate() {
            let coord = (x as i32, y as i32);
            if char == 'S' {
                origin = coord;
            }

            if x > 0 && "|7FS".contains(charmat[x - 1][y]) && "|LJS".contains(char) {
                bi_insert((x as i32, y as i32), (x as i32 - 1, y as i32));
            }

            if y < charmat.len() - 1 && "-J7S".contains(charmat[x][y + 1]) && "-LFS".contains(char) {
                bi_insert((x as i32, y as i32), (x as i32, y as i32 + 1));
            }
        }
    }
    println!("allocated 1: {}", connectivity.values().map(|v| v.capacity()).sum::<usize>());
    connectivity.values_mut().for_each(|v| v.shrink_to_fit());
    println!("allocated 2: {}", connectivity.values().map(|v| v.capacity()).sum::<usize>());

    assert_ne!(origin, (-1, -1));

    fn dfs(path: &mut Path,
           visited: &mut HashSet<Coord>,
           dest: Coord,
           connectivity: &HashMap<Coord, Path>) -> Option<Path> {
        let current = *path.last().unwrap();
        visited.insert(current);

        let neighbors = connectivity.get(&current).unwrap();
        for n in neighbors {
            if path.len() > 2 && *n == dest {
                path.push(*n);
                return Some(path.clone());
            }
            if !visited.contains(n) {
                let len = path.len();
                path.push(*n);
                let dfs_result = dfs(path, visited, dest, connectivity);
                path.truncate(len);
                if dfs_result.is_some() {
                    return dfs_result;
                }
            }
        }

        return None;
    }

    let path = dfs(&mut vec![origin], &mut HashSet::new(),
                   origin, &connectivity).expect("no path found");
    println!("part1: path length: {}; half length: {}", path.len(), path.len() / 2);


    let sum: u32 = (0..charmat.len() as i32).into_par_iter()
        .map(|x| {
            let mut sum = 0;
            for y in 0..charmat.len() as i32 {
                if point_in_polygon((x, y), &path) {
                    sum += 1;
                }
            }
            sum
        })
        .sum();

    println!("part2: {}", sum);

    Ok(())
}


// tweaked from https://github.com/zikwall/geofind-rust/blob/master/src/types/mod.rs
fn point_in_polygon(point: Coord, coordinates: &Path) -> bool {
    let mut k_prev = 0;
    let mut result = false;

    for (k, p) in coordinates.iter().enumerate() {
        if &point == p {
            return false;
        }
        if k <= 0 {
            k_prev = coordinates.len() - 1
        } else {
            k_prev = k - 1
        }

        let iflng = p.1 < point.1 && coordinates[k_prev].1 >= point.1 || coordinates[k_prev].1 < point.1 && p.1 >= point.1;
        let iflat = p.0 <= point.0 || coordinates[k_prev].0 <= point.0;

        if iflng && iflat {
            if p.0 + (point.1 - p.1) / (coordinates[k_prev].1 - p.1) * (coordinates[k_prev].0 - p.0) < point.0 {
                result = !result
            }
        }
    }

    return result;
}

