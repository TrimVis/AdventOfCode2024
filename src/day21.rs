use std::{collections::HashMap, fs};

const INPUT_FILE: &str = "inputs/day21.test";
// const INPUT_FILE: &str = "inputs/day21.input";

pub fn solve() {
    println!("Part 1: {}", solve_p1());
    println!("Part 2: {}", solve_p2());
}

fn generate_moves() -> HashMap<(char, char), String> {
    let mut moves: HashMap<(char, char), String> = HashMap::new();

    let numpad = [
        ['7', '8', '9'],
        ['4', '5', '6'],
        ['1', '2', '3'],
        ['x', '0', 'A'],
    ];

    for y_start in 0..numpad.len() {
        for x_start in 0..numpad[y_start].len() {
            for y_end in 0..numpad.len() {
                for x_end in 0..numpad[y_end].len() {
                    let start = numpad[y_start][x_start];
                    let end = numpad[y_end][x_end];
                    if start == 'x' || end == 'x' {
                        continue;
                    }

                    let y_diff = y_start as i64 - y_end as i64;
                    let x_diff = x_start as i64 - x_end as i64;
                    let mut mv = "".to_string();
                    if y_diff > 0 {
                        mv += "^".repeat(y_diff.abs() as usize).as_str();
                        if x_diff > 0 {
                            mv += "<".repeat(x_diff.abs() as usize).as_str();
                        } else if x_diff < 0 {
                            mv += ">".repeat(x_diff.abs() as usize).as_str();
                        }
                    } else if y_diff < 0 {
                        if x_diff > 0 {
                            mv += "<".repeat(x_diff.abs() as usize).as_str();
                        } else if x_diff < 0 {
                            mv += ">".repeat(x_diff.abs() as usize).as_str();
                        }
                        mv += "v".repeat(y_diff.abs() as usize).as_str();
                    } else {
                        if x_diff > 0 {
                            mv += "<".repeat(x_diff.abs() as usize).as_str();
                        } else if x_diff < 0 {
                            mv += ">".repeat(x_diff.abs() as usize).as_str();
                        }
                    }
                    mv += "A";
                    moves.insert((start, end), mv);
                }
            }
        }
    }

    let dirpad = [['x', '^', 'A'], ['<', 'v', '>']];

    for y_start in 0..dirpad.len() {
        for x_start in 0..dirpad[y_start].len() {
            for y_end in 0..dirpad.len() {
                for x_end in 0..dirpad[y_end].len() {
                    let start = dirpad[y_start][x_start];
                    let end = dirpad[y_end][x_end];
                    if start == 'x' || end == 'x' {
                        continue;
                    }

                    let y_diff = y_start as i64 - y_end as i64;
                    let x_diff = x_start as i64 - x_end as i64;
                    let mut mv = "".to_string();
                    if x_diff > 0 {
                        mv += ">".repeat(x_diff.abs() as usize).as_str();
                    } else if x_diff < 0 {
                        mv += "<".repeat(x_diff.abs() as usize).as_str();
                    }
                    if y_diff > 0 {
                        mv += "^".repeat(y_diff.abs() as usize).as_str();
                    } else if y_diff < 0 {
                        mv += "v".repeat(y_diff.abs() as usize).as_str();
                    }
                    mv += "A";
                    moves.insert((start, end), mv);
                }
            }
        }
    }

    moves
}

pub fn solve_p1() -> i64 {
    let content = fs::read_to_string(INPUT_FILE).expect("Could not read input file");

    let moves = generate_moves();

    let mut result = 0;

    for line in content.lines() {
        println!("Input: {}", line);

        let mut curr_line = line.to_string();
        for _ in 0..3 {
            let mut curr_moves = "".to_string();
            let aline = "A".to_string() + curr_line.as_str();
            let pairs = aline.chars().zip(curr_line.chars());
            for (start, dest) in pairs {
                curr_moves += moves[&(start, dest)].as_str();
            }
            println!("Intermediate: {}", curr_moves);

            curr_line = curr_moves;
        }

        let len = curr_line.len();
        let num_part = line.replace("A", "").parse::<usize>().unwrap();
        let res = len * num_part;
        result += res;
        println!("Result: {} ({} * {})", res, len, num_part);
    }

    0
}

pub fn solve_p2() -> i64 {
    let _content = fs::read_to_string(INPUT_FILE).expect("Could not read input file");

    0
}
