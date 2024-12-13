use std::{collections::HashSet, fs};

use itertools::Itertools;

const DIRS: [(i64, i64); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
// const INPUT_FILE: &str = "inputs/day12.test";
const INPUT_FILE: &str = "inputs/day12.input";

pub fn solve() {
    println!("Part 1: {}", solve_p1());
    println!("Part 2: {}", solve_p2());
}

#[allow(dead_code)]
struct Region {
    id: String,
    map: Vec<Vec<bool>>,
    visited: Vec<Vec<bool>>,
}

impl Region {
    fn from_map(map: &Vec<Vec<String>>, id: String) -> Region {
        let map: Vec<Vec<bool>> = map
            .iter()
            .map(|row| row.iter().map(|e| *e == id).collect())
            .collect();
        let visited: Vec<Vec<bool>> = vec![vec![false; map[0].len()]; map.len()];

        Region { id, map, visited }
    }

    fn get_price(&mut self, discount_edges: bool) -> u64 {
        let mut price = 0;
        for y in 0..self.map.len() {
            for x in 0..self.map[0].len() {
                if let Some((area, perimiter, no_sides)) = self.traverse((x as i64, y as i64)) {
                    let iprice = if !discount_edges {
                        area * perimiter
                    } else {
                        area * no_sides
                    };
                    if iprice > 0 {
                        println!(
                            "Region {} has a intermediate price result of {} (area: {}, no_sides: {}, perimiter: {})",
                            self.id, iprice, area, no_sides, perimiter
                        );
                    }
                    price += iprice;
                }
            }
        }
        println!("Region {} has a price of {}", self.id, price);
        price
    }

    fn oobounds(&self, pos: (i64, i64)) -> bool {
        pos.1 < 0
            || pos.0 < 0
            || pos.1 >= (self.map.len() as i64)
            || pos.0 >= (self.map[0].len() as i64)
    }

    fn check_visited(&mut self, pos: (i64, i64)) -> bool {
        self.oobounds(pos) || {
            let pos = (pos.1 as usize, pos.0 as usize);
            let res = self.visited[pos.1][pos.0];
            self.visited[pos.1][pos.0] = true;
            res
        }
    }

    fn check_pos(&self, pos: (i64, i64)) -> bool {
        if self.oobounds(pos) {
            false
        } else {
            self.map[pos.1 as usize][pos.0 as usize]
        }
    }

    fn check_dpos(&self, pos: (i64, i64), dir: (i64, i64)) -> bool {
        self.check_pos((pos.0 as i64 + dir.0, pos.1 as i64 + dir.1))
    }

    fn traverse(&mut self, pos: (i64, i64)) -> Option<(u64, u64, u64)> {
        if self.check_visited(pos) || !self.check_pos(pos) {
            return None;
        }

        let curr_pos = self.check_pos(pos);
        let mut area = curr_pos as u64;
        let mut perimeter = DIRS
            .iter()
            .filter(|&dir| !self.check_dpos(pos, *dir))
            .count() as u64;

        let mut sides = if curr_pos {
            DIRS.iter()
                .zip(DIRS.iter().cycle().skip(1))
                .filter(|&(&d0, &d1)| {
                    let d_combined = (d0.0 + d1.0, d0.1 + d1.1);
                    let is_outward_edge = !self.check_dpos(pos, d0)
                        && !self.check_dpos(pos, d1)
                        && !self.check_dpos(pos, d_combined);
                    let is_inward_edge = self.check_dpos(pos, d0)
                        && self.check_dpos(pos, d1)
                        && !self.check_dpos(pos, d_combined);
                    is_outward_edge || is_inward_edge
                })
                .count() as u64
        } else {
            0
        };

        if curr_pos {
            for dir in DIRS {
                let pos = (pos.0 + dir.0, pos.1 + dir.1);
                // println!("{:?}", pos);

                if let Some((narea, nperimeter, nsides)) = self.traverse(pos) {
                    area += narea;
                    sides += nsides;
                    perimeter += nperimeter;
                }
            }
        }

        Some((area, perimeter, sides))
    }
}

pub fn solve_p1() -> u64 {
    let content = fs::read_to_string(INPUT_FILE).expect("Could not read input file");

    let map: Vec<Vec<String>> = content
        .lines()
        .map(|l| l.chars().map(|c| c.to_string()).collect())
        .collect();
    let ids: HashSet<&String> = HashSet::from_iter(map.iter().flatten());

    let mut total_price = 0;
    for id in ids.iter().sorted_by_key(|v| v.to_string()) {
        let mut region = Region::from_map(&map, (*id).clone());
        let price = region.get_price(false);
        total_price += price;
    }

    total_price
}

pub fn solve_p2() -> u64 {
    let content = fs::read_to_string(INPUT_FILE).expect("Could not read input file");

    let map: Vec<Vec<String>> = content
        .lines()
        .map(|l| l.chars().map(|c| c.to_string()).collect())
        .collect();
    let ids: HashSet<&String> = HashSet::from_iter(map.iter().flatten());

    let mut total_price = 0;
    for id in ids.iter().sorted_by_key(|v| v.to_string()) {
        let mut region = Region::from_map(&map, (*id).clone());
        let price = region.get_price(true);
        total_price += price;
    }

    total_price
}
