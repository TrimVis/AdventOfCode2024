use std::fs;

// const INPUT_FILE: &str = "inputs/day7.test";
const INPUT_FILE: &str = "inputs/day7.input";

pub fn solve() {
    println!("Part 1: {}", solve_p1());
    println!("Part 2: {}", solve_p2());
}

pub fn solve_p1() -> u64 {
    let content = fs::read_to_string(INPUT_FILE).expect("Could not read input file");

    let mut sum = 0;
    for line in content.lines() {
        let values: Vec<&str> = line.split(':').collect();
        let result: u64 = values[0].parse().unwrap();
        let values: Vec<u64> = values[1]
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let mut carries = vec![values[0]];
        for v in values.iter().skip(1) {
            let mut new_carries = vec![];
            for c in carries {
                if v * c <= result {
                    new_carries.push(v * c)
                }
                if v + c <= result {
                    new_carries.push(v + c)
                }
            }

            carries = new_carries
        }

        for c in carries {
            if c == result {
                sum += result;
                break;
            }
        }
    }

    sum
}

pub fn solve_p2() -> u64 {
    let content = fs::read_to_string(INPUT_FILE).expect("Could not read input file");

    let mut sum = 0;
    for line in content.lines() {
        let values: Vec<&str> = line.split(':').collect();
        let result: u64 = values[0].parse().unwrap();
        let values: Vec<u64> = values[1]
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let mut carries = vec![values[0]];
        for v in values.iter().skip(1) {
            let mut new_carries = vec![];
            for c in carries {
                if v * c <= result {
                    new_carries.push(v * c)
                }
                if v + c <= result {
                    new_carries.push(v + c)
                }
                let concat_res: u64 = format!("{}{}", c, v).parse().unwrap();
                if concat_res <= result {
                    new_carries.push(concat_res)
                }
            }

            carries = new_carries
        }

        for c in carries {
            if c == result {
                sum += result;
                break;
            }
        }
    }

    sum
}
