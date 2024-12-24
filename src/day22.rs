use std::{
    collections::{HashMap, HashSet},
    fs,
};

const INPUT_FILE: &str = "inputs/day22.test";
// const INPUT_FILE: &str = "inputs/day22.input";

pub fn solve() {
    println!("Part 1: {}", solve_p1());
    println!("Part 2: {}", solve_p2());
}

fn monkey_price(price: u64) -> u64 {
    let price = f_op(64 * price, price);
    let price = f_op(price / 32, price);
    let price = f_op(2048 * price, price);
    price
}

fn f_op(v0: u64, v1: u64) -> u64 {
    (v0 ^ v1) % 16777216
}

pub fn solve_p1() -> u64 {
    let content = fs::read_to_string(INPUT_FILE).expect("Could not read input file");

    let mut sum = 0;
    for line in content.lines() {
        let input = line.parse::<u64>().unwrap();
        let mut price = input;
        for _ in 0..2000 {
            price = monkey_price(price);
        }

        // println!("{}: {}", input, price);
        sum += price;
    }

    sum
}

pub fn solve_p2() -> i64 {
    let content = fs::read_to_string(INPUT_FILE).expect("Could not read input file");

    let mut seller_changes = vec![];
    for line in content.lines() {
        let mut price_changes = vec![];
        let input = line.parse::<i64>().unwrap();
        let mut price: i64 = input;
        for _ in 0..2001 {
            let new_price = monkey_price(price as u64) as i64;
            price_changes.push((new_price % 10, new_price % 10 - price % 10));
            price = new_price;
        }
        seller_changes.push(price_changes);

        // println!("{}: {}", input, price);
    }

    let mut change_set = HashSet::new();

    for seller in seller_changes.clone() {
        for i in 3..seller.len() {
            change_set.insert([
                seller[i - 3].1,
                seller[i - 2].1,
                seller[i - 1].1,
                seller[i].1,
            ]);
        }
    }

    let mut best_pattern = [0, 0, 0, 0];
    let mut best_price = 0;
    for monkey_pattern in change_set {
        // println!("Best: {:?}: {}", best_pattern, best_price);
        let mut monkey_prices = vec![];
        for seller in seller_changes.clone() {
            'i: for i in 3..seller.len() {
                let change = [
                    seller[i - 3].1,
                    seller[i - 2].1,
                    seller[i - 1].1,
                    seller[i].1,
                ];
                if change == monkey_pattern {
                    // println!("p: {}", seller[i].0);
                    monkey_prices.push(seller[i].0);
                    break 'i;
                }
            }
        }
        // println!("Current: {:?}: {}", monkey_pattern, monkey_price);

        if monkey_prices.iter().sum::<i64>() > best_price {
            best_pattern = monkey_pattern;
            best_price = monkey_prices.iter().sum();
            println!("{:?}", monkey_prices)
        }
    }

    println!("{:?}: {}", best_pattern, best_price);

    best_price
}
