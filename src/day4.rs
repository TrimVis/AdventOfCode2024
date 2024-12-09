use std::fs;

// const INPUT_FILE: &str = "inputs/day4.test";
const INPUT_FILE: &str = "inputs/day4.input";

pub fn solve() {
    println!("Part 1: {}", solve_p1());
    println!("Part 2: {}", solve_p2());
}

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];
fn check_xmas_position(grid: &Vec<Vec<char>>, pos: (i64, i64), dir: (i64, i64)) -> bool {
    let mut x = pos.0;
    let mut y = pos.1;
    let len = 4;

    if x + len * dir.0 < 0
        || x + len * dir.0 >= grid[0].len() as i64
        || y + len * dir.1 < 0
        || y + len * dir.1 >= grid.len() as i64
    {
        return false;
    }

    let mut is_xmas = true;
    for i in 0..len {
        is_xmas &= grid[x as usize][y as usize] == XMAS[i as usize];
        if !is_xmas {
            break;
        }
        x += dir.0;
        y += dir.1;
    }
    is_xmas
}

pub fn solve_p1() -> u64 {
    let content = fs::read_to_string(INPUT_FILE).expect("Could not read input file");

    let grid: Vec<Vec<char>> = content.lines().map(|l| l.chars().collect()).collect();

    let mut result = 0;
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            for d_x in -1..2 {
                for d_y in -1..2 {
                    let ires = check_xmas_position(&grid, (x as i64, y as i64), (d_x, d_y));
                    // println!("ires: {} ({},{})", ires, x, y);
                    result += ires as u64;
                }
            }
        }
    }

    result
}

fn check_mas_position(grid: &Vec<Vec<char>>, pos: (i64, i64)) -> bool {
    if pos.0 == 0
        || pos.0 == grid.len() as i64 - 1
        || pos.1 == 0
        || pos.1 == grid[0].len() as i64 - 1
    {
        false
    } else {
        let (x0, x1) = (pos.0 - 1, pos.0 + 2);
        let (y0, y1) = (pos.1 - 1, pos.1 + 2);

        let mut s0 = [' ', ' ', ' '];
        for (i, (x, y)) in (x0..x1).zip(y0..y1).enumerate() {
            s0[i] = grid[x as usize][y as usize];
        }
        if !(s0 == ['M', 'A', 'S'] || s0 == ['S', 'A', 'M']) {
            return false;
        }
        let mut s1 = [' ', ' ', ' '];
        for (i, (x, y)) in (x0..x1).rev().zip(y0..y1).enumerate() {
            s1[i] = grid[x as usize][y as usize];
        }
        s1 == ['M', 'A', 'S'] || s1 == ['S', 'A', 'M']
    }
}

pub fn solve_p2() -> i64 {
    let content = fs::read_to_string(INPUT_FILE).expect("Could not read input file");

    let grid: Vec<Vec<char>> = content.lines().map(|l| l.chars().collect()).collect();

    let mut result = 0;
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            result += check_mas_position(&grid, (x as i64, y as i64)) as i64;
        }
    }

    result
}
