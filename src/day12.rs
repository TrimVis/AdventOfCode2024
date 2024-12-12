use std::{collections::HashSet, fs};

const DIRS: [(i64, i64); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
const INPUT_FILE: &str = "inputs/day12.test";
// const INPUT_FILE: &str = "inputs/day12.input";

pub fn solve() {
    println!("Part 1: {}", solve_p1());
    println!("Part 2: {}", solve_p2());
}

#[allow(dead_code)]
struct Region {
    id: char,
    map: Vec<Vec<bool>>,
    visited: Vec<Vec<bool>>,
}

impl Region {
    fn from_map(map: &Vec<Vec<char>>, id: char) -> Region {
        let map: Vec<Vec<bool>> = map
            .iter()
            .map(|row| row.iter().map(|&e| e == id).collect())
            .collect();
        let visited: Vec<Vec<bool>> = vec![vec![false; map[0].len() + 2]; map.len() + 2];

        Region { id, map, visited }
    }

    fn traverse(&mut self, pos: (usize, usize)) -> (u64, u64) {
        let curr_pos = self.map[pos.1][pos.0];
        let mut area = curr_pos as u64;
        let mut perimeter = !curr_pos as u64;

        if !curr_pos {
            return (area, perimeter);
        }
        if self.visited[pos.1][pos.0] {
            return (0, 0);
        }
        self.visited[pos.1][pos.0] = true;

        for dir in DIRS {
            let pos = (pos.0 as i64 + dir.0, pos.1 as i64 + dir.1);
            if self.oobounds(pos) {
                perimeter += 1;
                continue;
            }

            let (narea, nperimeter) = self.traverse((pos.0 as usize, pos.1 as usize));
            area += narea;
            perimeter += nperimeter;
        }

        (area, perimeter)
    }

    fn get_price(&mut self) -> u64 {
        let mut price = 0;
        for y in 0..self.map.len() {
            for x in 0..self.map[0].len() {
                if !self.visited[y][x] {
                    let (area, perimiter) = self.traverse((x, y));
                    let iprice = area * perimiter;
                    price += iprice;
                }
            }
        }
        price
    }

    fn oobounds(&self, pos: (i64, i64)) -> bool {
        pos.1 < 0
            || pos.0 < 0
            || pos.1 >= (self.map.len() as i64)
            || pos.0 >= (self.map[0].len() as i64)
    }

    fn check_visited(&mut self, pos: (i64, i64)) -> bool {
        let pos = ((pos.1 + 1) as usize, (pos.0 + 1) as usize);

        let res = self.visited[pos.1][pos.0];
        self.visited[pos.1][pos.0] = true;
        res
    }

    fn check_pos(&self, pos: (i64, i64)) -> bool {
        if !self.oobounds(pos) {
            self.map[pos.1 as usize][pos.0 as usize]
        } else {
            false
        }
    }

    fn check_dpos(&self, pos: (i64, i64), dir: (i64, i64)) -> bool {
        let pos = (pos.0 as i64 + dir.0, pos.1 as i64 + dir.1);
        self.check_pos(pos)
    }

    fn traverse_discounted(&mut self, pos: (i64, i64)) -> Option<(u64, u64)> {
        if self.check_visited(pos) {
            return None;
        }

        let curr_pos = self.check_pos(pos);
        let mut area = curr_pos as u64;
        let mut sides = DIRS
            .iter()
            .zip(DIRS.iter().cycle().skip(1))
            .filter(|&(&d0, &d1)| {
                if curr_pos {
                    !self.check_dpos(pos, d0) && !self.check_dpos(pos, d1)
                } else {
                    self.check_dpos(pos, d0) && self.check_dpos(pos, d1)
                }
            })
            .count() as u64;
        if sides > 0 {
            println!(
                "{}: Position {:?} ({}) is a edge {}",
                self.id, pos, curr_pos, sides
            );
        }

        if !curr_pos {
            return Some((0, sides));
        }

        for dir in DIRS {
            let pos = (pos.0 as i64 + dir.0, pos.1 as i64 + dir.1);

            if let Some((narea, nsides)) = self.traverse_discounted(pos) {
                area += narea;
                sides += nsides;
            }
        }

        Some((area, sides))
    }

    fn get_discounted_price(&mut self) -> u64 {
        let mut price = 0;
        for y in 0..self.map.len() {
            for x in 0..self.map[0].len() {
                if let Some((area, no_sides)) = self.traverse_discounted((x as i64, y as i64)) {
                    // Finally compute the discounted price
                    let iprice = area * no_sides;
                    price += iprice;
                    if iprice > 0 {
                        println!(
                            "Region {} has a intermediate price result of {} (area: {}, no_sides: {})",
                            self.id, price, area, no_sides
                        );
                    }
                }
            }
        }
        println!("Region {} has a price of {}", self.id, price);
        price
    }
}

pub fn solve_p1() -> u64 {
    let content = fs::read_to_string(INPUT_FILE).expect("Could not read input file");

    let map: Vec<Vec<char>> = content.lines().map(|l| l.chars().collect()).collect();
    let ids: HashSet<&char> = HashSet::from_iter(map.iter().flatten());

    let mut total_price = 0;
    for id in ids {
        let mut region = Region::from_map(&map, *id);
        let price = region.get_price();
        total_price += price;
    }

    total_price
}

pub fn solve_p2() -> u64 {
    let content = fs::read_to_string(INPUT_FILE).expect("Could not read input file");

    let map: Vec<Vec<char>> = content.lines().map(|l| l.chars().collect()).collect();
    let ids: HashSet<&char> = HashSet::from_iter(map.iter().flatten());

    let mut total_price = 0;
    for id in ids {
        let mut region = Region::from_map(&map, *id);
        let price = region.get_discounted_price();
        total_price += price;
    }

    total_price
}
