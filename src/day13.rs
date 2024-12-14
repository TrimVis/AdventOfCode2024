use std::{cmp::min, fs, ops::Add};

use itertools::Itertools;
use num::{
    integer::{gcd, lcm},
    pow::Pow,
    Float,
};

const INPUT_FILE: &str = "inputs/day13.test";
// const INPUT_FILE: &str = "inputs/day13.input";

pub fn solve() {
    println!("Part 1: {}", solve_p1());
    // println!("Part 2: {}", solve_p2());
}

#[derive(Clone, Copy)]
enum Number<T>
where
    T: Ord,
    T: Add,
{
    Value(T),
    Infinity,
}

impl<T: Ord + Add> Number<T> {
    fn min(n0: Self, n1: Self) -> Self {
        match (n0, n1) {
            (Number::Value(v0), Number::Value(v1)) => Number::Value(min(v0, v1)),
            (Number::Infinity, Number::Infinity) => Number::Infinity,
            (v0, Number::Infinity) => v0,
            (Number::Infinity, v1) => v1,
        }
    }
}

impl<T: Ord + Add<Output = T>> Add for Number<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::Infinity, _) | (_, Number::Infinity) => Number::Infinity,
            (Number::Value(v0), Number::Value(v1)) => Number::Value(v0 + v1),
        }
    }
}

type Num = u64;
struct DpSolution {
    coins: Vec<Vec<Option<Number<Num>>>>,
    cost_a: Num,
    cost_b: Num,
    diff_a: (i64, i64),
    diff_b: (i64, i64),
    goal: (usize, usize),
}

impl DpSolution {
    fn new(
        cost_a: Num,
        cost_b: Num,
        diff_a: (i64, i64),
        diff_b: (i64, i64),
        goal: (usize, usize),
    ) -> DpSolution {
        let mut coins: Vec<Vec<Option<Number<Num>>>> =
            vec![vec![None; (goal.0 + 1) as usize]; (goal.1 + 1) as usize];
        coins[0][0] = Some(Number::Value(0));

        println!("Created!!!");

        DpSolution {
            coins,
            cost_a,
            cost_b,
            diff_a,
            diff_b,
            goal,
        }
    }

    fn solve(&mut self) -> Num {
        if let Number::Value(v) = self.check((self.goal.0 as i64, self.goal.1 as i64)) {
            v
        } else {
            0
        }
    }

    fn inbounds(&self, pos: (i64, i64)) -> bool {
        !(pos.1 < 0
            || pos.1 >= (self.coins.len() as i64)
            || pos.0 < 0
            || pos.0 >= (self.coins[0].len() as i64))
    }

    fn check(&mut self, pos: (i64, i64)) -> Number<Num> {
        if !self.inbounds(pos) {
            return Number::Infinity;
        }

        if let Some(result) = self.coins[pos.1 as usize][pos.0 as usize] {
            result
        } else {
            let result = Number::min(
                self.check((pos.0 - self.diff_a.0, pos.1 - self.diff_a.1))
                    + Number::Value(self.cost_a),
                self.check((pos.0 - self.diff_b.0, pos.1 - self.diff_b.1))
                    + Number::Value(self.cost_b),
            );
            self.coins[pos.1 as usize][pos.0 as usize] = Some(result);
            result
        }
    }
}

fn solve_la(
    cost_a: Num,
    cost_b: Num,
    v_a: (i64, i64),
    v_b: (i64, i64),
    goal_point: (usize, usize),
) -> Option<Num> {
    let mut res = None;
    for ((v_a, v_b), (cost_a, cost_b)) in [(v_a, v_b), (v_b, v_a)]
        .iter()
        .zip([(cost_a, cost_b), (cost_b, cost_a)])
    {
        let (da, db) = (v_a.0 as f64 / v_a.1 as f64, v_b.0 as f64 / v_b.1 as f64);
        let (la, lb) = (
            (v_a.0.pow(2) as f64 + v_a.1.pow(2) as f64).sqrt(),
            (v_b.0.pow(2) as f64 + v_b.1.pow(2) as f64).sqrt(),
        );

        let g_y = goal_point.1 as f64;

        // Find intersection point between between va and vb
        // Assume vb intersects with goal
        let i_point = (g_y / (da + db), g_y - da * g_y / (db + da));

        // Calculate the number of coins needed, by using one of the line coordinates as lenght
        let steps_a: f64 = i_point.0 / (v_a.0 as f64);
        let steps_b: f64 = (g_y - i_point.1) / (v_b.1 as f64);

        println!("intersection point: {:?} ", i_point);
        println!("len a: {}; len b: {}", la, lb);
        println!("da: {}; db: {}", da, db);
        println!("steps a: {}; steps b: {}", steps_a, steps_b);
        println!(
            "total costs: {}",
            steps_a * (cost_a as f64) + steps_b * (cost_b as f64)
        );
        if steps_a % 1.0 != 0.0 || steps_b % 1.0 != 0.0 {
            continue;
        }
        let new_res = (steps_a as u64) * cost_a + (steps_b as u64) * cost_b;
        if let Some(prev_res) = res {
            if new_res < prev_res {
                res = Some(new_res);
            }
        }
    }
    res
}

pub fn solve_p1() -> Num {
    let content = fs::read_to_string(INPUT_FILE).expect("Could not read input file");

    let mut result = 0;
    for mut task in content.lines().chunks(4).into_iter() {
        let diff_a: &str = task.next().unwrap().strip_prefix("Button A: ").unwrap();
        let diff_a: (i64, i64) = diff_a
            .splitn(2, ", ")
            .map(|v| v.splitn(2, "+").nth(1).unwrap().parse::<i64>().unwrap())
            .next_tuple()
            .unwrap();

        let diff_b: &str = task.next().unwrap().strip_prefix("Button B: ").unwrap();
        let diff_b: (i64, i64) = diff_b
            .splitn(2, ", ")
            .map(|v| v.splitn(2, "+").nth(1).unwrap().parse::<i64>().unwrap())
            .next_tuple()
            .unwrap();

        let goal: &str = task.next().unwrap().strip_prefix("Prize: ").unwrap();
        let goal: (usize, usize) = goal
            .splitn(2, ", ")
            .map(|v| v.splitn(2, "=").nth(1).unwrap().parse::<usize>().unwrap())
            .next_tuple()
            .unwrap();

        let mut dp = DpSolution::new(3, 1, diff_a, diff_b, goal);
        let ires = dp.solve();

        println!("Args: ({},{},{:?},{:?},{:?})", 3, 1, diff_a, diff_b, goal);
        println!("[DP] Intermediate Result: {}", ires);

        let ires2 = solve_la(3, 1, diff_a, diff_b, goal);
        println!("[LinAlg] Intermediate Result: {:?}", ires2);

        result += ires;
    }

    result
}

pub fn solve_p2() -> Num {
    let content = fs::read_to_string(INPUT_FILE).expect("Could not read input file");

    let mut result = 0;
    for mut task in content.lines().chunks(4).into_iter() {
        let diff_a: &str = task.next().unwrap().strip_prefix("Button A: ").unwrap();
        let diff_a: (i64, i64) = diff_a
            .splitn(2, ", ")
            .map(|v| v.splitn(2, "+").nth(1).unwrap().parse::<i64>().unwrap())
            .next_tuple()
            .unwrap();

        let diff_b: &str = task.next().unwrap().strip_prefix("Button B: ").unwrap();
        let diff_b: (i64, i64) = diff_b
            .splitn(2, ", ")
            .map(|v| v.splitn(2, "+").nth(1).unwrap().parse::<i64>().unwrap())
            .next_tuple()
            .unwrap();

        let goal: &str = task.next().unwrap().strip_prefix("Prize: ").unwrap();
        let goal: (usize, usize) = goal
            .splitn(2, ", ")
            .map(|v| v.splitn(2, "=").nth(1).unwrap().parse::<usize>().unwrap() + 10000000000000)
            .next_tuple()
            .unwrap();

        let mut dp = DpSolution::new(3, 1, diff_a, diff_b, goal);
        let ires = dp.solve();
        println!("Args: ({},{},{:?},{:?},{:?})", 3, 1, diff_a, diff_b, goal);
        println!("Intermediate Result: {}", ires);

        result += ires;
    }

    result
}
