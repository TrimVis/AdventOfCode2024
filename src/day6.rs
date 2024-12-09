use rayon::prelude::*;
use std::{fmt, fs};

use crate::time_function;

// const INPUT_FILE: &str = "inputs/day6.test";
const INPUT_FILE: &str = "inputs/day6.input";

#[derive(PartialEq, Debug, Clone, Copy)]
enum Tile {
    UNVISITED,
    VISITED,
    OBSTRUCTION,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tile::UNVISITED => write!(f, "."),
            Tile::VISITED => write!(f, "x"),
            Tile::OBSTRUCTION => write!(f, "#"),
        }
    }
}

type Map = Vec<Vec<Tile>>;
type LastVisitMap = Vec<Vec<Vec<(i64, i64)>>>;

pub fn solve() {
    println!("Part 1: {}", time_function!(solve_p1));
    // Takes 30s to execute in debug mode
    // println!("Part 2: {}", time_function!(solve_p2));
}

pub fn solve_p1() -> u64 {
    let content = fs::read_to_string(INPUT_FILE).expect("Could not read input file");

    let mut guard_pos: (i64, i64) = (0, 0);
    let mut guard_dir: (i64, i64) = (0, -1);

    let mut map: Map = vec![];
    for (y, line) in content.lines().enumerate() {
        let row = line
            .chars()
            .enumerate()
            .map(|(x, tile)| match tile {
                '^' => {
                    guard_pos = (x as i64, y as i64);
                    Tile::UNVISITED
                }
                '.' => Tile::UNVISITED,
                '#' => Tile::OBSTRUCTION,
                _ => unreachable!("Unknown tile character"),
            })
            .collect();

        map.push(row);
    }

    let x_len = map[0].len() as i64;
    let y_len = map.len() as i64;
    let inbounds = |(x, y)| -> bool { x >= 0 && x < x_len && y >= 0 && y < y_len };

    loop {
        map[guard_pos.1 as usize][guard_pos.0 as usize] = Tile::VISITED;

        let new_pos = (guard_pos.0 + guard_dir.0, guard_pos.1 + guard_dir.1);
        if !inbounds(new_pos) {
            break;
        }

        if map[new_pos.1 as usize][new_pos.0 as usize] == Tile::OBSTRUCTION {
            guard_dir = match guard_dir {
                (1, 0) => (0, 1),
                (0, -1) => (1, 0),
                (-1, 0) => (0, -1),
                (0, 1) => (-1, 0),
                _ => unreachable!(),
            };
        } else {
            guard_pos = new_pos;
        }
    }

    let mut result = 0;
    for row in map {
        // println!("{:?}", row);
        for point in row {
            if point == Tile::VISITED {
                result += 1;
            }
        }
    }

    result
}

#[allow(dead_code)]
pub fn solve_p2() -> u64 {
    let content = fs::read_to_string(INPUT_FILE).expect("Could not read input file");

    let mut guard_pos: (i64, i64) = (0, 0);
    let guard_dir: (i64, i64) = (0, -1);

    let mut last_visited: LastVisitMap = vec![];
    let mut map: Map = vec![];
    for (y, line) in content.lines().enumerate() {
        let row: Vec<Tile> = line
            .chars()
            .enumerate()
            .map(|(x, tile)| match tile {
                '^' => {
                    guard_pos = (x as i64, y as i64);
                    Tile::UNVISITED
                }
                '.' => Tile::UNVISITED,
                '#' => Tile::OBSTRUCTION,
                _ => unreachable!("Unknown tile character"),
            })
            .collect();

        last_visited.push(vec![vec![]; row.len()]);
        map.push(row);
    }

    let x_len: usize = map[0].len();
    let y_len: usize = map.len();
    let inbounds =
        |(x, y)| -> bool { x >= 0 && x < (x_len as i64) && y >= 0 && y < (y_len as i64) };

    (0..y_len)
        .into_par_iter()
        .map(|obstruction_y| {
            (0..x_len)
                .map(|obstruction_x| {
                    let mut guard_pos: (i64, i64) = guard_pos.clone();
                    let mut guard_dir: (i64, i64) = guard_dir.clone();
                    let mut map = map.clone();
                    let mut last_visited = last_visited.clone();

                    if map[obstruction_y][obstruction_x as usize] == Tile::OBSTRUCTION {
                        return 0;
                    }
                    map[obstruction_y][obstruction_x as usize] = Tile::OBSTRUCTION;
                    loop {
                        let new_pos = (guard_pos.0 + guard_dir.0, guard_pos.1 + guard_dir.1);
                        if !inbounds(new_pos) {
                            return 0;
                        } else if last_visited[new_pos.1 as usize][new_pos.0 as usize]
                            .contains(&guard_dir)
                        {
                            return 1;
                        }

                        last_visited[guard_pos.1 as usize][guard_pos.0 as usize].push(guard_dir);

                        if map[new_pos.1 as usize][new_pos.0 as usize] == Tile::OBSTRUCTION {
                            guard_dir = match guard_dir {
                                (1, 0) => (0, 1),
                                (0, -1) => (1, 0),
                                (-1, 0) => (0, -1),
                                (0, 1) => (-1, 0),
                                _ => unreachable!(),
                            };
                        } else {
                            guard_pos = new_pos;
                        }
                    }
                })
                .reduce(|a, b| a + b)
                .unwrap()
        })
        .reduce(|| 0, |a, b| a + b)
}
