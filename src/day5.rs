use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fs,
};

// const INPUT_FILE: &str = "inputs/day5.test";
const INPUT_FILE: &str = "inputs/day5.input";

pub fn solve() {
    println!("Part 1: {}", solve_p1());
    println!("Part 2: {}", solve_p2());
}

pub fn solve_p1() -> u64 {
    let content = fs::read_to_string(INPUT_FILE).expect("Could not read input file");

    let mut rules: HashMap<u64, Vec<u64>> = HashMap::new();
    let mut update_set: HashSet<Vec<u64>> = HashSet::new();

    let mut rule_mode = true;
    for line in content.lines() {
        if line.trim().is_empty() {
            rule_mode = false;
        } else if rule_mode {
            let rule: Vec<u64> = line.split('|').map(|v| v.parse().unwrap()).collect();
            assert!(
                rule.len() == 2,
                "Expected a rule to consists of two elements"
            );

            if let Some(v) = rules.get_mut(&rule[0]) {
                v.push(rule[1])
            } else {
                rules.insert(rule[0], vec![rule[1]]);
            }
        } else {
            update_set.insert(line.split(',').map(|v| v.parse().unwrap()).collect());
        }
    }

    let mut result = 0;
    for update in update_set {
        let mut is_valid = true;
        'outer: for i in 0..update.len() {
            for j in i + 1..update.len() {
                if rules.contains_key(&update[j]) && rules[&update[j]].contains(&update[i]) {
                    is_valid = false;
                    break 'outer;
                }
            }
        }
        if is_valid {
            let center_pos = update.len() / 2 - (if update.len() % 2 == 0 { 1 } else { 0 });
            result += update[center_pos];
        }
    }

    result
}

pub fn solve_p2() -> u64 {
    let content = fs::read_to_string(INPUT_FILE).expect("Could not read input file");

    let mut rules: HashMap<u64, Vec<u64>> = HashMap::new();
    let mut update_set: HashSet<Vec<u64>> = HashSet::new();

    let mut rule_mode = true;
    for line in content.lines() {
        if line.trim().is_empty() {
            rule_mode = false;
        } else if rule_mode {
            let rule: Vec<u64> = line.split('|').map(|v| v.parse().unwrap()).collect();
            assert!(
                rule.len() == 2,
                "Expected a rule to consists of two elements"
            );

            if let Some(v) = rules.get_mut(&rule[0]) {
                v.push(rule[1])
            } else {
                rules.insert(rule[0], vec![rule[1]]);
            }
        } else {
            update_set.insert(line.split(',').map(|v| v.parse().unwrap()).collect());
        }
    }

    let mut result = 0;
    for update in update_set {
        let mut is_valid = true;
        'outer: for i in 0..update.len() {
            for j in i + 1..update.len() {
                if rules.contains_key(&update[j]) && rules[&update[j]].contains(&update[i]) {
                    is_valid = false;
                    break 'outer;
                }
            }
        }
        if !is_valid {
            let mut update = update.clone();
            update.sort_by(|v0, v1| {
                if rules.contains_key(&v1) && rules[&v1].contains(&v0) {
                    return Ordering::Less;
                }

                return Ordering::Equal;
            });

            let center_pos = update.len() / 2 - (if update.len() % 2 == 0 { 1 } else { 0 });
            result += update[center_pos];
        }
    }

    result
}
