// use std::{collections::LinkedList, fs};
use std::{collections::HashMap, fs};

use crate::time_function;

// const INPUT_FILE: &str = "inputs/day11.test";
const INPUT_FILE: &str = "inputs/day11.input";

pub fn solve() {
    println!("Part 1: {}", time_function!(solve_p1));
    println!("Part 2: {}", time_function!(solve_p2));
}

#[allow(dead_code)]
fn blink_flatmap2(stones: Vec<u64>, no_blinks: usize) -> usize {
    let mut stones = stones;
    for _ in 0..no_blinks {
        stones = stones
            .iter()
            .flat_map(|stone| {
                match stone {
                    0 => [Some(1), None],
                    n if n.to_string().len() % 2 == 0 => {
                        let n = n.to_string();
                        let elem0 = n[0..n.len() / 2].parse().unwrap();
                        let elem1 = n[n.len() / 2..].parse().unwrap();
                        [Some(elem0), Some(elem1)]
                    }
                    n => [Some(n * 2024), None],
                }
                .iter()
                .filter_map(|x| *x)
                .collect::<Vec<u64>>()
            })
            .collect();
    }

    stones.len()
}

#[allow(dead_code)]
fn blink_flatmap(stones: Vec<u64>, no_blinks: usize) -> usize {
    let mut stones = stones;
    for _ in 0..no_blinks {
        stones = stones
            .iter()
            .flat_map(|stone| match stone {
                0 => vec![1],
                n if n.to_string().len() % 2 == 0 => {
                    let n = n.to_string();
                    let elem0 = n[0..n.len() / 2].parse().unwrap();
                    let elem1 = n[n.len() / 2..].parse().unwrap();
                    vec![elem0, elem1]
                }
                n => vec![n * 2024],
            })
            .collect();
    }

    stones.len()
}

#[allow(dead_code)]
fn blink_vecinplace_lessstr(stones: Vec<u64>, no_blinks: usize) -> usize {
    let mut stones = stones;
    for _ in 0..no_blinks {
        let mut i = 0;
        let len = stones.len();
        while i < len {
            let curr = stones.get_mut(i).unwrap();
            let mut prev: Option<u64> = None;
            *curr = match *curr {
                0 => 1,
                n if n >= 10 && (n as f64).log10().floor() % 2.0 == 1.0 => {
                    // println!("n: '{}'", n);
                    let n = n.to_string();
                    prev = Some(n[0..n.len() / 2].parse().unwrap());
                    n[n.len() / 2..].parse().unwrap()
                }
                n => n * 2024,
            };
            if let Some(prev) = prev {
                stones.push(prev);
            }
            i += 1;
        }
    }

    stones.len()
}

#[allow(dead_code)]
fn blink_vecinplace2(stones: Vec<u64>, no_blinks: usize) -> usize {
    let mut stones = stones;
    for _ in 0..no_blinks {
        let mut i = 0;
        let len = stones.len();
        while i < len {
            let curr = stones.get_mut(i).unwrap();
            let mut prev: Option<u64> = None;
            *curr = match *curr {
                0 => 1,
                n if n.to_string().len() % 2 == 0 => {
                    let n = n.to_string();
                    prev = Some(n[0..n.len() / 2].parse().unwrap());
                    n[n.len() / 2..].parse().unwrap()
                }
                n => n * 2024,
            };
            if let Some(prev) = prev {
                stones.push(prev);
            }
            i += 1;
        }
    }

    stones.len()
}

#[allow(dead_code)]
fn blink_vecinplace(stones: Vec<u64>, no_blinks: usize) -> usize {
    let mut stones = stones;
    for _ in 0..no_blinks {
        let len = stones.len();
        let mut add: Vec<u64> = vec![];
        for curr in stones.iter_mut().take(len) {
            *curr = match *curr {
                0 => 1,
                n if n.to_string().len() % 2 == 0 => {
                    let n = n.to_string();
                    add.push(n[0..n.len() / 2].parse().unwrap());
                    n[n.len() / 2..].parse().unwrap()
                }
                n => n * 2024,
            };
        }
        stones.extend(add);
    }

    stones.len()
}

fn blink_vecinplace_nostr(stones: Vec<u64>, no_blinks: usize) -> usize {
    let mut stones = stones;
    for _step in 0..no_blinks {
        // println!("Step {}/{}", step, no_blinks);
        let mut i = 0;
        let len = stones.len();
        while i < len {
            let curr = stones.get_mut(i).unwrap();
            let mut prev: Option<u64> = None;
            *curr = match *curr {
                0 => 1,
                n => {
                    let no_decimals = (n as f64).log10().floor() as u32 + 1;
                    if n >= 10 && no_decimals % 2 == 0 {
                        // println!("n: '{}' - Decimal Count: {}", n, no_decimals);
                        let base = 10u64.pow(no_decimals / 2);
                        let small = n % base;
                        let large = (n - small) / base;
                        // println!("'{}' == '{}' '{}'", n, large, small);
                        prev = Some(small);
                        large
                    } else {
                        n * 2024
                    }
                }
            };
            if let Some(prev) = prev {
                stones.push(prev);
            }
            i += 1;
        }
    }

    stones.len()
}

fn blink_vecinplace_nostr_noredund(stones: Vec<u64>, no_blinks: usize) -> usize {
    let mut stones: HashMap<usize, u64> =
        HashMap::from_iter(stones.iter().map(|v| (*v as usize, 1)));
    for _step in 0..no_blinks {
        let mut new_ks = vec![];
        for (k, v) in stones.iter_mut() {
            if *v == 0 {
                continue;
            }
            new_ks.extend(match *k {
                0 => vec![(1, *v)],
                n => {
                    let n = n as u64;
                    let no_decimals = (n as f64).log10().floor() as u32 + 1;
                    if n >= 10 && no_decimals % 2 == 0 {
                        // println!("n: '{}' - Decimal Count: {}", n, no_decimals);
                        let base = 10u64.pow(no_decimals / 2);
                        let small = n % base;
                        let large = (n - small) / base;
                        // println!("'{}' == '{}' '{}'", n, large, small);
                        vec![(large, *v), (small, *v)]
                    } else {
                        vec![(n * 2024, *v)]
                    }
                }
            });
            *v = 0;
        }
        for (new_k, new_v) in new_ks {
            let new_k = new_k as usize;
            let v = stones.get_mut(&new_k);
            if let Some(v) = v {
                *v += new_v;
            } else {
                stones.insert(new_k, new_v);
            }
        }
    }

    stones.iter().fold(0, |c, (_, v)| c + (*v as usize))
}

pub fn solve_p1() -> usize {
    let content = fs::read_to_string(INPUT_FILE).expect("Could not read input file");

    let stones: Vec<u64> = content
        .lines()
        .next()
        .expect("Expected at least one line of input")
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect();

    let flatmap = || blink_flatmap(stones.clone(), 25);
    time_function!(flatmap);
    let flatmap2 = || blink_flatmap2(stones.clone(), 25);
    time_function!(flatmap2);
    let vecinplace = || blink_vecinplace(stones.clone(), 25);
    time_function!(vecinplace);
    let vecinplace2 = || blink_vecinplace2(stones.clone(), 25);
    let vecinplace_lessstr = || blink_vecinplace_lessstr(stones.clone(), 25);
    time_function!(vecinplace_lessstr);
    let vecinplace_nostr = || blink_vecinplace_nostr(stones.clone(), 25);
    time_function!(vecinplace_nostr);
    let vecinplace_nostr_noredund = || blink_vecinplace_nostr_noredund(stones.clone(), 25);
    time_function!(vecinplace_nostr_noredund);

    println!(
        "expected result: {}",
        blink_vecinplace_nostr(stones.clone(), 6)
    );

    //blink_vecinplace_nostr(stones, 25)
    blink_vecinplace_nostr_noredund(stones, 25)
}

pub fn solve_p2() -> usize {
    let content = fs::read_to_string(INPUT_FILE).expect("Could not read input file");

    let stones: Vec<u64> = content
        .lines()
        .next()
        .expect("Expected at least one line of input")
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect();

    blink_vecinplace_nostr_noredund(stones, 75)
}
