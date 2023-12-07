use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let times = [61, 67, 75, 71];
    let distances = [430, 1036, 1307, 1150];

    let mut product = 1u32;
    for (&time, distance) in times.iter().zip(distances.iter()) {
        let count = (1..time).map(|t| (time - t) * t)
            .filter(|d| d > distance)
            .count();
        product *= count as u32;
    }

    println!("part1: {product}");

    let time: u64 = times.map(|t| t.to_string()).join("").parse().unwrap();
    let distance: u64 = distances.map(|t| t.to_string()).join("").parse().unwrap();

    let count = (1..time).map(|t| (time - t) * t)
        .filter(|d| d > &distance)
        .count();
    println!("part2: {count}");

    return Ok(());
}