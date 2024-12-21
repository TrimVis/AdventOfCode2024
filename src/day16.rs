use core::fmt;
use std::{
    fmt::{Display, Formatter},
    fs,
};

use itertools::Itertools;
use num::Float;

use crate::util::Coordinate;

const INPUT_FILE: &str = "inputs/day16.test";
// const INPUT_FILE: &str = "inputs/day16.input";

pub fn solve() {
    println!("Part 1: {}", solve_p1());
    println!("Part 2: {}", solve_p2());
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Dir {
    North,
    South,
    West,
    East,
}

impl Dir {
    fn turn_left(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::West => Self::South,
            Self::South => Self::East,
            Self::East => Self::North,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Self::West => Self::North,
            Self::South => Self::West,
            Self::East => Self::South,
            Self::North => Self::East,
        }
    }

    fn to_coord(&self) -> Coordinate<i64> {
        match self {
            Self::West => Coordinate::new(-1, 0),
            Self::South => Coordinate::new(0, -1),
            Self::East => Coordinate::new(1, 0),
            Self::North => Coordinate::new(0, 1),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Field {
    Wall,
    Empty,
    Space(i64),
    Start,
    End,
}

impl Field {
    fn from_char(c: char) -> Self {
        match c {
            '#' => Self::Wall,
            '.' => Self::Empty,
            'S' => Self::Start,
            'E' => Self::End,
            _ => unimplemented!("This character is not supported: {}", c),
        }
    }
}

struct Dp {
    // Stores the score at each position and for each input direction
    table: Vec<Vec<Field>>,
    start_pos: Coordinate<i64>,
}

impl Dp {
    fn from_input(content: String) -> Self {
        // TODO: This should set the positions of all walls to -1
        // The destination should have a score of 0
        let table = content
            .lines()
            .map(|row| row.chars().map(Field::from_char).collect())
            .collect();
        let start_pos = content
            .lines()
            .enumerate()
            .find_map(|(y, row)| match row.chars().find_position(|&c| c == 'S') {
                Some((x, _)) => Some(Coordinate::new(x as i64, y as i64)),
                None => None,
            })
            .unwrap();

        Dp { table, start_pos }
    }

    fn get_t(&self, pos: &Coordinate<i64>) -> Field {
        self.table[pos.y as usize][pos.x as usize]
    }
    fn set_t(&mut self, pos: Coordinate<i64>, val: Field) {
        self.table[pos.y as usize][pos.x as usize] = val
    }

    fn dp(&mut self, carry: i64, pos: Coordinate<i64>, dir: Dir) {
        println!("{:?} {:?}\n{}", pos, dir, self);
        match self.get_t(&pos) {
            Field::End | Field::Space(_) | Field::Wall => {}
            Field::Start => {
                self.set_t(pos, Field::Space(carry));
            }
            Field::Empty => {
                self.set_t(pos, Field::Space(carry));

                let mut check_dir =
                    |d: Dir, add_carry| self.dp(carry + add_carry, pos + d.to_coord(), d);

                check_dir(dir, 1);
                check_dir(dir.turn_left(), 1001);
                check_dir(dir.turn_right(), 1001);
                check_dir(dir.turn_right().turn_right(), 2001);
            }
        };
    }

    fn solve(&mut self) -> i64 {
        for dir in [Dir::North, Dir::South, Dir::West, Dir::East] {
            self.dp(self.end_position + dir.to_coord(), dir, 1);
        }
        self.get_t(self.start_pos).unwrap()
    }
}

impl Into<String> for &Field {
    fn into(self) -> String {
        match self {
            Field::End => "E".to_string(),
            Field::Start => "S".to_string(),
            Field::Space(v) => ((*v as f64).log10().round() as u8).to_string(),
            Field::Wall => "#".to_string(),
            Field::Empty => ".".to_string(),
        }
    }
}

impl Display for Dp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let disp: String = self
            .table
            .iter()
            .map(|row| row.iter().map_into::<String>().collect::<String>())
            .join("\n");
        write!(f, "Start {:?}\n{}", self.start_pos, disp)
    }
}

pub fn solve_p1() -> i64 {
    let content = fs::read_to_string(INPUT_FILE).expect("Could not read input file");

    let mut problem = Dp::from_input(content);
    problem.solve()
}

pub fn solve_p2() -> i64 {
    let content = fs::read_to_string(INPUT_FILE).expect("Could not read input file");

    let mut problem = Dp::from_input(content);
    problem.solve()
}
