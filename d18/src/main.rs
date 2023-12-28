use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use geo::{EuclideanLength, LineString};
use geo::algorithm::area::Area;
use geo::Polygon;

const PART2: bool = true;

type Coord = (i32, i32);
type Path = Vec<Coord>;


fn main() -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(File::open("d18.txt")?);

    let mut path = vec![(0, 0)];
    for line in reader.lines() {
        let line = line?;
        let parts = line.split_whitespace().collect::<Vec<_>>();

        let hex_distance = i32::from_str_radix(&parts[2][2..7], 16).unwrap();
        let hex_dir = parts[2].as_bytes()[parts[2].len() - 2] - '0' as u8;

        let (dir, distance);

        if PART2 {
            distance = hex_distance;
            dir = match hex_dir {
                0 => 'R',
                1 => 'D',
                2 => 'L',
                3 => 'U',
                _ => panic!("malformed")
            }
        } else {
            dir = parts[0].chars().next().unwrap();
            distance = parts[1].parse::<i32>()?;
        }

        let delta = match dir {
            'U' => { (-distance, 0) }
            'D' => { (distance, 0) }
            'L' => { (0, -distance) }
            'R' => { (0, distance) }
            _ => panic!("malformed")
        };
        let previous = *path.last().unwrap();
        path.push((previous.0 + delta.0, previous.1 + delta.1));
    }
    assert_eq!(path[0], path[path.len() - 1]);


    let line = LineString::from_iter(path.iter().map(|&(y, x)| (y as f64, x as f64)));
    let polygon = Polygon::new(
        line,
        vec![],
    );
    let area = polygon.unsigned_area() + polygon.exterior().euclidean_length() / 2.0 + 1.0;
    println!("area: {}", area);

    Ok(())
}
