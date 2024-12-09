use std::{collections::HashMap, fs};

// const INPUT_FILE: &str = "inputs/day1.test";
const INPUT_FILE: &str = "inputs/day1.input";

pub fn solve() {
    println!("Part 1: {}", solve_p1());
    println!("Part 2: {}", solve_p2());
}

pub fn solve_p1() -> i64 {
    let content = fs::read_to_string(INPUT_FILE).expect("Could not read input file");

    let mut list0: Vec<i64> = vec![];
    let mut list1: Vec<i64> = vec![];

    for line in content.lines() {
        let pair: Vec<i64> = line
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect();
        list0.push(pair[0]);
        list1.push(pair[1]);
    }

    list0.sort();
    list1.sort();

    let mut sum = 0;
    for (p0, p1) in list0.iter().zip(list1) {
        sum += (p0 - p1).abs();
    }

    return sum;
}

pub fn solve_p2() -> i64 {
    let content = fs::read_to_string(INPUT_FILE).expect("Could not read input file");

    let mut list0: Vec<i64> = vec![];
    let mut list1: HashMap<i64, i64> = HashMap::new();

    for line in content.lines() {
        let pair: Vec<i64> = line
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect();
        list0.push(pair[0]);
        if let Some(v) = list1.get_mut(&pair[1]) {
            *v += 1;
        } else {
            list1.insert(pair[1], 1);
        }
    }

    let mut sim_score = 0;
    for v in list0 {
        if let Some(rep) = list1.get(&v) {
            sim_score += v * *rep;
        }
    }

    return sim_score;
}
