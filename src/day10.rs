use std::{collections::HashSet, fs};

// const INPUT_FILE: &str = "inputs/day10.test";
const INPUT_FILE: &str = "inputs/day10.input";

pub fn solve() {
    println!("Part 1: {}", solve_p1());
    println!("Part 2: {}", solve_p2());
}

fn hike_trail_rated(map: &Vec<Vec<u8>>, start_pos: (usize, usize)) -> usize {
    // println!("Walking Position: {:?}", start_pos);
    let curr_height = map[start_pos.1][start_pos.0];
    if curr_height == 9 {
        return 1;
    }

    let dirs = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let (x, y): (i16, i16) = (start_pos.0 as i16, start_pos.1 as i16);
    let mut targets = 0;
    for (dx, dy) in dirs {
        let (x, y): (i16, i16) = (x + dx, y + dy);
        if x < 0 || y < 0 || (y as usize) >= map.len() || (x as usize) >= map[y as usize].len() {
            continue;
        }
        let next_height = map[y as usize][x as usize];
        if next_height == curr_height + 1 {
            targets += hike_trail_rated(map, (x as usize, y as usize));
        }
    }

    targets
}

fn hike_trail_head(map: &Vec<Vec<u8>>, start_pos: (usize, usize)) -> HashSet<(usize, usize)> {
    // println!("Walking Position: {:?}", start_pos);
    let curr_height = map[start_pos.1][start_pos.0];
    if curr_height == 9 {
        return HashSet::from([start_pos]);
    }

    let dirs = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let (x, y): (i16, i16) = (start_pos.0 as i16, start_pos.1 as i16);
    let mut targets = HashSet::new();
    for (dx, dy) in dirs {
        let (x, y): (i16, i16) = (x + dx, y + dy);
        if x < 0 || y < 0 || (y as usize) >= map.len() || (x as usize) >= map[y as usize].len() {
            continue;
        }
        let next_height = map[y as usize][x as usize];
        if next_height == curr_height + 1 {
            let next_targets = hike_trail_head(map, (x as usize, y as usize));
            targets.extend(&next_targets);
        }
    }

    targets
}

pub fn solve_p1() -> usize {
    let content = fs::read_to_string(INPUT_FILE).expect("Could not read input file");

    let map: Vec<Vec<u8>> = content
        .lines()
        .map(|line| {
            line.chars()
                .map(|height| {
                    height
                        .to_string()
                        .parse::<u8>()
                        .expect("Expected integer heights")
                })
                .collect()
        })
        .collect();

    let mut trails = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 0 {
                let no_trails = hike_trail_head(&map, (x, y));
                trails += no_trails.len();
            }
        }
    }

    trails
}

pub fn solve_p2() -> usize {
    let content = fs::read_to_string(INPUT_FILE).expect("Could not read input file");

    let map: Vec<Vec<u8>> = content
        .lines()
        .map(|line| {
            line.chars()
                .map(|height| {
                    height
                        .to_string()
                        .parse::<u8>()
                        .expect("Expected integer heights")
                })
                .collect()
        })
        .collect();

    let mut trails = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 0 {
                let no_trails = hike_trail_rated(&map, (x, y));
                trails += no_trails;
            }
        }
    }

    trails
}
