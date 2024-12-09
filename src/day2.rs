use std::fs;

// const INPUT_FILE: &str = "inputs/day2.test";
const INPUT_FILE: &str = "inputs/day2.input";

pub fn solve() {
    println!("Part 1: {}", solve_p1());
    println!("Part 2: {}", solve_p2());
}

pub fn solve_p1() -> i64 {
    let content = fs::read_to_string(INPUT_FILE).expect("Could not read input file");

    let mut reports = vec![];

    for line in content.lines() {
        let report: Vec<i64> = line
            .split_whitespace()
            .map(|v| v.parse().expect("Reports should only contain numbers"))
            .collect();
        reports.push(report);
    }

    let mut safe_reports = 0;
    for r in reports {
        let is_valid = if r.len() < 2 {
            true
        } else {
            let mut is_valid = false;
            let is_inc = r[1] - r[0] > 0;
            for (c, n) in r.iter().zip(r.iter().skip(1)) {
                let diff = n - c;
                is_valid = if is_inc { diff > 0 } else { diff < 0 };
                is_valid &= diff.abs() <= 3;
                // println!("s: {} - {}", is_valid, diff);
                if !is_valid {
                    break;
                }
            }
            is_valid
        };
        safe_reports += is_valid as i64;
        // println!("{}", is_valid);
    }

    safe_reports
}

pub fn solve_p2() -> i64 {
    let content = fs::read_to_string(INPUT_FILE).expect("Could not read input file");

    let mut reports = vec![];

    for line in content.lines() {
        let report: Vec<i64> = line
            .split_whitespace()
            .map(|v| v.parse().expect("Reports should only contain numbers"))
            .collect();
        reports.push(report);
    }

    let mut safe_reports = 0;
    for r in reports {
        let is_valid = if r.len() < 2 {
            true
        } else {
            let mut is_valid = false;
            let is_inc = r[1] - r[0] > 0;
            for (c, n) in r.iter().zip(r.iter().skip(1)) {
                let diff = n - c;
                is_valid = if is_inc { diff > 0 } else { diff < 0 };
                is_valid &= diff.abs() <= 3;
                // println!("s: {} - {}", is_valid, diff);
                if !is_valid {
                    break;
                }
            }

            // Try again with a level removed
            if !is_valid {
                for skip_i in 0..r.len() {
                    for (i, (c, n)) in r.iter().zip(r.iter().skip(1)).enumerate() {
                        if i == skip_i {
                            continue;
                        }
                        let diff = n - c;
                        is_valid = if is_inc { diff > 0 } else { diff < 0 };
                        is_valid &= diff.abs() <= 3;
                        // println!("s: {} - {}", is_valid, diff);
                        if !is_valid {
                            break;
                        }
                    }
                    if is_valid {
                        break;
                    }
                }
            }
            is_valid
        };
        safe_reports += is_valid as i64;
        // println!("{}", is_valid);
    }

    safe_reports
}
