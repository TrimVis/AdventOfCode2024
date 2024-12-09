use std::{
    collections::{HashMap, HashSet},
    fs,
};

// const INPUT_FILE: &str = "inputs/day8.test";
const INPUT_FILE: &str = "inputs/day8.input";

pub fn solve() {
    println!("Part 1: {}", solve_p1());
    println!("Part 2: {}", solve_p2());
}

pub fn solve_p1() -> usize {
    let content = fs::read_to_string(INPUT_FILE).expect("Could not read input file");

    let mut antennas: HashMap<char, Vec<(i64, i64)>> = HashMap::new();
    for (y, line) in content.lines().enumerate() {
        for (x, p) in line.chars().enumerate() {
            match p {
                '.' => continue,
                c => {
                    if let Some(v) = antennas.get_mut(&c) {
                        v.push((x as i64, y as i64));
                    } else {
                        antennas.insert(c, vec![(x as i64, y as i64)]);
                    }
                }
            }
        }
    }
    let y_len = content.lines().count() as i64;
    let x_len = content.lines().next().unwrap().len() as i64;

    // let mut antinodes: Vec<Vec<Option<char>>> = vec![vec![None; x_len]; y_len];

    let mut unique_locs = HashSet::new();
    for (_c, points) in antennas {
        for (i, a) in points.iter().enumerate() {
            for b in points.iter().skip(i + 1) {
                let (x0, y0) = (2 * a.0 - b.0, 2 * a.1 - b.1);
                if x0 >= 0 && x0 < x_len && y0 >= 0 && y0 < y_len {
                    unique_locs.insert((x0, y0));
                }
                let (x1, y1) = (2 * b.0 - a.0, 2 * b.1 - a.1);
                if x1 >= 0 && x1 < x_len && y1 >= 0 && y1 < y_len {
                    unique_locs.insert((x1, y1));
                }
            }
        }
    }

    unique_locs.len()
}
pub fn solve_p2() -> usize {
    let content = fs::read_to_string(INPUT_FILE).expect("Could not read input file");

    let mut antennas: HashMap<char, Vec<(i64, i64)>> = HashMap::new();
    for (y, line) in content.lines().enumerate() {
        for (x, p) in line.chars().enumerate() {
            match p {
                '.' => continue,
                c => {
                    if let Some(v) = antennas.get_mut(&c) {
                        v.push((x as i64, y as i64));
                    } else {
                        antennas.insert(c, vec![(x as i64, y as i64)]);
                    }
                }
            }
        }
    }
    let y_len = content.lines().count() as i64;
    let x_len = content.lines().next().unwrap().len() as i64;

    // let mut antinodes: Vec<Vec<Option<char>>> = vec![vec![None; x_len]; y_len];

    let mut unique_locs = HashSet::new();
    for (_c, points) in antennas {
        for (i, a) in points.iter().enumerate() {
            for b in points.iter().skip(i + 1) {
                for i in 0.. {
                    let (x, y) = (a.0 + i * (a.0 - b.0), a.1 + i * (a.1 - b.1));
                    if x >= 0 && x < x_len && y >= 0 && y < y_len {
                        unique_locs.insert((x, y));
                    } else {
                        break;
                    }
                }
                for i in 0.. {
                    let (x, y) = (b.0 + i * (b.0 - a.0), b.1 + i * (b.1 - a.1));
                    if x >= 0 && x < x_len && y >= 0 && y < y_len {
                        unique_locs.insert((x, y));
                    } else {
                        break;
                    }
                }
            }
        }
    }

    unique_locs.len()
}
