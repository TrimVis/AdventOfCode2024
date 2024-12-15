use std::fs;

use itertools::{Either, Itertools};

use crate::util::Coordinate;

// const INPUT_FILE: &str = "inputs/day15.test0";
// const INPUT_FILE: &str = "inputs/day15.test1";
const INPUT_FILE: &str = "inputs/day15.input";

pub fn solve() {
    println!("Part 1: {}", solve_p1());
    println!("Part 2: {}", solve_p2());
}

#[derive(Debug)]
enum Move {
    Left,
    Right,
    Up,
    Down,
}
impl Move {
    fn from_char(c: char) -> Result<Self, &'static str> {
        match c {
            '<' => Ok(Self::Left),
            '>' => Ok(Self::Right),
            '^' => Ok(Self::Up),
            'v' => Ok(Self::Down),
            _ => Err("Got an invalid character"),
        }
    }

    fn get_dir_coord(&self) -> Coordinate<i64> {
        match self {
            Self::Left => Coordinate::new(-1, 0),
            Self::Right => Coordinate::new(1, 0),
            Self::Up => Coordinate::new(0, -1),
            Self::Down => Coordinate::new(0, 1),
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Field {
    Empty,
    Wall,
    Robot,
    Box,
}
impl Field {
    fn from_char(c: char) -> Result<Self, &'static str> {
        match c {
            '#' => Ok(Self::Wall),
            '@' => Ok(Self::Robot),
            'O' => Ok(Self::Box),
            '.' => Ok(Self::Empty),
            _ => Err("Received an invalid character"),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Self::Empty => ' ',
            Self::Wall => '#',
            Self::Robot => '@',
            Self::Box => '█',
        }
    }
}
struct Map {
    map: Vec<Vec<Field>>,
    robot_position: Coordinate<i64>,

    width: i64,
    height: i64,
}
impl Map {
    fn new(map: Vec<Vec<Field>>) -> Self {
        // Find robot position and replace it with a empty field
        let robot_position = map
            .iter()
            .enumerate()
            .find_map(|(i, row)| {
                if let Some((r, _)) = row.iter().find_position(|&f| *f == Field::Robot) {
                    Some(Coordinate::new(i as i64, r as i64))
                } else {
                    None
                }
            })
            .unwrap();

        let height = map.len() as i64;
        let width = map.get(0).unwrap().len() as i64;

        Map {
            map,
            robot_position,
            height,
            width,
        }
    }

    fn inbounds(&self, coord: Coordinate<i64>) -> bool {
        coord.x >= 0 && coord.y >= 0 && coord.x < self.width && coord.y < self.height
    }
    fn get_pos(&self, pos: Coordinate<i64>) -> Field {
        self.map[pos.y as usize][pos.x as usize]
    }
    fn step_many(&mut self, moves: &Vec<&Move>, visualize: bool) {
        if visualize {
            println!("{}\n\n", self.ascii_art());
        }
        for mv in moves {
            self.step(mv);
            if visualize {
                println!("Moved: {:?}\n{}\n\n", mv, self.ascii_art());
            }
        }
    }

    fn step(&mut self, mv: &Move) {
        let _ = self.move_field(self.robot_position, mv);
    }

    fn move_field(&mut self, pos: Coordinate<i64>, mv: &Move) -> Result<(), &'static str> {
        let target_pos = pos + mv.get_dir_coord();
        if !self.inbounds(target_pos) {
            return Err("Out of bounds");
        }

        let current = self.get_pos(pos);
        let target = self.get_pos(target_pos);
        match target {
            Field::Wall => return Err("Move into wall"),
            Field::Empty => {
                self.map[pos.y as usize][pos.x as usize] = target;
                self.map[target_pos.y as usize][target_pos.x as usize] = current;
            }
            Field::Box => {
                // Try to move the box away
                if self.move_field(target_pos, mv).is_ok() {
                    // And if that works, move the current field
                    self.map[pos.y as usize][pos.x as usize] = self.get_pos(target_pos);
                    self.map[target_pos.y as usize][target_pos.x as usize] = current;
                } else {
                    return Err("Box is blocked");
                }
            }
            Field::Robot => {
                unreachable!("This should not have happened! Tried to move into robot")
            }
        }

        if current == Field::Robot {
            self.robot_position = target_pos;
        }

        return Ok(());
    }

    fn get_all_boxes(&self) -> Vec<Coordinate<i64>> {
        let mut box_positions = vec![];
        for y in 1..(self.height as usize) {
            for x in 1..(self.width as usize) {
                if self.map[y][x] == Field::Box {
                    box_positions.push(Coordinate::new(x as i64, y as i64));
                }
            }
        }
        box_positions
    }

    fn ascii_art(&self) -> String {
        self.map
            .iter()
            .map(|row| String::from_iter(row.iter().map(|f| f.to_char())))
            .join("\n")
    }
}

pub fn solve_p1() -> i64 {
    let content = fs::read_to_string(INPUT_FILE).expect("Could not read input file");
    let (map, instructions): (Vec<Vec<Field>>, Vec<Vec<Move>>) =
        content.lines().partition_map(|line| {
            let line = line.trim();
            if line.starts_with("#") {
                Either::Left(line.chars().map(|c| Field::from_char(c).unwrap()).collect())
            } else {
                Either::Right(line.chars().map(|c| Move::from_char(c).unwrap()).collect())
            }
        });
    let mut map = Map::new(map);
    let instructions = instructions.iter().flatten().collect();
    let visualize = false;
    map.step_many(&instructions, visualize);

    let mut result = 0;
    for box_coord in map.get_all_boxes() {
        result += 100 * box_coord.y + box_coord.x;
    }

    result
}

pub fn solve_p2() -> i64 {
    let _content = fs::read_to_string(INPUT_FILE).expect("Could not read input file");

    0
}
