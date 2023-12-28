use std::cmp::max;
use std::collections::{BinaryHeap, HashSet};
use std::error::Error;
use std::fs;

// point, direction, distance in current direction
type TraversedNode = ((usize, usize), (i32, i32), usize);
type Path = Vec<(usize, usize)>;
type HeapNode = (i32, Path);


const PART2: bool = true;

fn main() -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string("d17.txt")?;
    let grid = text.split_whitespace()
        .map(|r| r.chars().map(|d| d.to_digit(10).unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut traversed: HashSet<TraversedNode> = HashSet::new();

    let mut frontier: BinaryHeap<HeapNode> = BinaryHeap::from([
        (0, vec![(0, 0)])
    ]);

    while !frontier.is_empty() {
        let node = frontier.pop().unwrap();
        let coord = node.1.last().unwrap();

        let (direction, same_dir_distance) = get_same_dist_direction(&node);

        let max_same_dir_distance = if PART2 { 10 } else { 3 };
        if same_dir_distance > max_same_dir_distance {
            continue;
        }

        // point, direction, distance in current direction
        let traversed_node: TraversedNode = (*coord, direction, same_dir_distance);
        if !traversed.insert(traversed_node) {
            continue;
        }
        let min_same_dir_distance = 4;

        if *coord == (grid[0].len() - 1, grid.len() - 1) && (!PART2 || min_same_dir_distance <= same_dir_distance) {
            let path: HashSet<(usize, usize)> = HashSet::from_iter(node.1);
            print_path(&grid, path);
            println!("\n\nfound goal: distance {}\n", -node.0);
            break;
        }

        let coord = (coord.0 as i32, coord.1 as i32);

        let can_switch_dir = !PART2 || (same_dir_distance >= min_same_dir_distance || node.1.len() == 1);

        let neighbors = get_neighbors(direction, coord, can_switch_dir);
        for (x, y) in neighbors {
            if x >= 0 && y >= 0 && x < grid[0].len() as i32 && y < grid.len() as i32 {
                let d = grid[y as usize][x as usize];
                let mut path = Vec::with_capacity(node.1.len() + 1);
                path.extend(node.1.iter());
                path.push((x as usize, y as usize));
                frontier.push((node.0 - d as i32, path))
            }
        }
    }

    Ok(())
}

fn get_same_dist_direction(node: &HeapNode) -> ((i32, i32), usize) {
    let deltas = &node.1[max(node.1.len() as i32 - 12, 0) as usize..];
    let mut deltas = deltas.iter().zip(deltas.iter().skip(1))
        .map(|(&b, &a)| {
            (a.0 as i32 - b.0 as i32, a.1 as i32 - b.1 as i32)
        })
        .collect::<Vec<_>>();
    deltas.reverse();
    let direction = *deltas.first().unwrap_or(&(0, 1));
    let same_dir_distance = count_same(&deltas);
    (direction, same_dir_distance)
}

fn print_path(grid: &Vec<Vec<u32>>, path: HashSet<(usize, usize)>) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let c = if path.contains(&(j, i)) { "#" } else { "." };
            print!("{c}");
        }
        println!();
    }
}

fn get_neighbors(direction: (i32, i32), coord: (i32, i32), can_switch_dir: bool) -> Vec<(i32, i32)> {
    let mut neighbors = vec![];
    match direction {
        (1, 0) => {
            neighbors.push((coord.0 + 1, coord.1));
            if can_switch_dir {
                neighbors.push((coord.0, coord.1 - 1));
                neighbors.push((coord.0, coord.1 + 1));
            }
        }
        (-1, 0) => {
            neighbors.push((coord.0 - 1, coord.1));
            if can_switch_dir {
                neighbors.push((coord.0, coord.1 - 1));
                neighbors.push((coord.0, coord.1 + 1));
            }
        }
        (0, 1) => {
            neighbors.push((coord.0, coord.1 + 1));
            if can_switch_dir {
                neighbors.push((coord.0 - 1, coord.1));
                neighbors.push((coord.0 + 1, coord.1));
            }
        }
        (0, -1) => {
            neighbors.push((coord.0, coord.1 - 1));
            if can_switch_dir {
                neighbors.push((coord.0 - 1, coord.1));
                neighbors.push((coord.0 + 1, coord.1));
            }
        }
        _ => panic!()
    }
    neighbors
}

fn count_same<T>(arr: &[T]) -> usize
    where T: Eq + Copy {
    if arr.is_empty() {
        return 0;
    }

    let first = arr[0];
    let mut count = 0;
    for e in arr {
        if first == *e {
            count += 1;
        } else {
            return count;
        }
    }
    count
}
