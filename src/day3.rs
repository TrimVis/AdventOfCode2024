use regex::Regex;
use std::fs;

// const INPUT_FILE: &str = "inputs/day3.test";
const INPUT_FILE: &str = "inputs/day3.input";

pub fn solve() {
    println!("Part 1: {}", solve_p1());
    println!("Part 2: {}", solve_p2());
}

pub fn solve_p1() -> u64 {
    let content = fs::read_to_string(INPUT_FILE).expect("Could not read input file");

    let mut result = 0;
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    for (_i, [a, b]) in re.captures_iter(&content).map(|c| c.extract()) {
        // println!("a: {}, b: {} - {}", a, b, _i);
        let a = a.parse::<u64>().unwrap();
        let b = b.parse::<u64>().unwrap();
        result += a * b;
    }

    result
}

pub fn solve_p2() -> u64 {
    let content = fs::read_to_string(INPUT_FILE).expect("Could not read input file");

    let mut result = 0;
    let mut do_ = true;
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)|don't\(()()\)|do\(()()\)").unwrap();
    for (m, [a, b]) in re.captures_iter(&content).map(|c| c.extract()) {
        // println!("{} - {:?}", m, c);
        if m == "don't()" {
            do_ = false;
        } else if m == "do()" {
            do_ = true;
        } else if do_ && m.starts_with("mul(") {
            let a = a.parse::<u64>().unwrap();
            let b = b.parse::<u64>().unwrap();
            result += a * b;
        }
    }

    result
}
