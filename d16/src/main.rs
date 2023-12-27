use std::cmp::max;
use std::collections::HashSet;
use std::error::Error;
use std::fs;

struct Grid {
    grid: Vec<Vec<char>>,
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Node {
    point: Point,
    direction: Direction,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum Direction { N, S, E, W }

impl Grid {
    pub fn new(grid: Vec<Vec<char>>) -> Self {
        Self {
            grid
        }
    }

    pub fn traverse(&self, origin: Node) -> HashSet<(i32, i32)> {
        let mut visited = HashSet::new();
        let mut occupied = HashSet::new();

        let mut frontier = vec![origin];
        let rows = self.grid.len() as i32;
        let cols = self.grid[0].len() as i32;

        while !frontier.is_empty() {
            let next = frontier.pop().unwrap();
            let (x, y) = (next.point.x, next.point.y);
            if x < 0 || y < 0 || x >= cols || y >= rows {
                continue;
            }
            let mut p = self.grid[y as usize][x as usize];
            let direction = next.direction;
            match p {
                '|' => {
                    match direction {
                        Direction::N | Direction::S => {
                            p = '.';
                        }
                        _ => {}
                    }
                }
                '-' => {
                    match direction {
                        Direction::W | Direction::E => {
                            p = '.';
                        }
                        _ => {}
                    }
                }
                _ => {}
            }


            if !visited.insert(next) {
                continue;
            }
            occupied.insert((x, y));
            match p {
                '.' => {
                    frontier.push(Node {
                        point: Point {
                            x: match direction {
                                Direction::N => { x }
                                Direction::S => { x }
                                Direction::E => { x + 1 }
                                Direction::W => { x - 1 }
                            },
                            y: match direction {
                                Direction::N => { y - 1 }
                                Direction::S => { y + 1 }
                                Direction::E => { y }
                                Direction::W => { y }
                            },
                        },
                        direction,
                    });
                }
                '|' => {
                    match direction {
                        Direction::W | Direction::E => {
                            frontier.push(Node {
                                point: Point { x, y: y - 1 },
                                direction: Direction::N,
                            });
                            frontier.push(Node {
                                point: Point { x, y: y + 1 },
                                direction: Direction::S,
                            });
                        }
                        _ => panic!()
                    }
                }
                '-' => {
                    match direction {
                        Direction::S | Direction::N => {
                            frontier.push(Node {
                                point: Point { x: x + 1, y },
                                direction: Direction::E,
                            });
                            frontier.push(Node {
                                point: Point { x: x - 1, y },
                                direction: Direction::W,
                            });
                        }
                        _ => panic!()
                    }
                }
                '/' => {
                    frontier.push(Node {
                        point: Point {
                            x: match direction {
                                Direction::N => { x + 1 }
                                Direction::S => { x - 1 }
                                Direction::E => { x }
                                Direction::W => { x }
                            },
                            y: match direction {
                                Direction::N => { y }
                                Direction::S => { y }
                                Direction::E => { y - 1 }
                                Direction::W => { y + 1 }
                            },
                        },
                        direction: match direction {
                            Direction::N => { Direction::E }
                            Direction::S => { Direction::W }
                            Direction::E => { Direction::N }
                            Direction::W => { Direction::S }
                        },
                    });
                }
                '\\' => {
                    frontier.push(Node {
                        point: Point {
                            x: match direction {
                                Direction::N => { x - 1 }
                                Direction::S => { x + 1 }
                                Direction::E => { x }
                                Direction::W => { x }
                            },
                            y: match direction {
                                Direction::N => { y }
                                Direction::S => { y }
                                Direction::E => { y + 1 }
                                Direction::W => { y - 1 }
                            },
                        },
                        direction: match direction {
                            Direction::N => { Direction::W }
                            Direction::S => { Direction::E }
                            Direction::E => { Direction::S }
                            Direction::W => { Direction::N }
                        },
                    });
                }
                _ => panic!("unknown value {p}")
            }
        }
        return occupied;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string("d16.txt")?;

    let grid = Grid::new(
        text.split_whitespace()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>()
    );

    let occupied = grid.traverse(Node { point: Point { x: 0, y: 0 }, direction: Direction::E });

    println!("part1: {}", occupied.len());

    let mut part2 = 0;
    for x in 0..grid.grid[0].len() {
        part2 = max(part2,
                    grid.traverse(Node { point: Point { x: x as i32, y: 0 }, direction: Direction::S }).len());
        part2 = max(part2,
                    grid.traverse(Node { point: Point { x: x as i32, y: grid.grid.len() as i32 - 1 }, direction: Direction::N }).len());
    }
    for y in 0..grid.grid.len() {
        part2 = max(part2,
                    grid.traverse(Node { point: Point { x: 0, y: y as i32 }, direction: Direction::E }).len());
        part2 = max(part2,
                    grid.traverse(Node { point: Point { x: grid.grid[0].len() as i32 - 1, y: y as i32 }, direction: Direction::W }).len());
    }

    println!("part2: {part2}");

    Ok(())
}
