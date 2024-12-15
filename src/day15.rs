use colored::{ColoredString, Colorize};
use std::fs;
use std::{thread, time::Duration};

use crossterm::event::{self, Event, KeyCode, KeyEvent};
use itertools::{Either, Itertools};

use crate::util::{Coordinate, Term};

// const INPUT_FILE: &str = "inputs/day15.test0";
// const INPUT_FILE: &str = "inputs/day15.test1";
// const INPUT_FILE: &str = "inputs/day15.test2";
const INPUT_FILE: &str = "inputs/day15.input";

pub fn solve() {
    println!("Part 1: {}", solve_p1());
    println!("Part 2: {}", solve_p2());
}

#[derive(Debug, PartialEq)]
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

    #[allow(dead_code)]
    fn flip(&self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Field {
    Empty,
    Wall,
    Robot,
    Box,
    BoxLeft,
    BoxRight,
}
impl Field {
    fn from_char_double(c: char) -> Result<[Self; 2], &'static str> {
        match c {
            '#' => Ok([Self::Wall, Self::Wall]),
            '@' => Ok([Self::Robot, Self::Empty]),
            'O' => Ok([Self::BoxLeft, Self::BoxRight]),
            '.' => Ok([Self::Empty, Self::Empty]),
            _ => Err("Received an invalid character"),
        }
    }
    fn from_char(c: char) -> Result<Self, &'static str> {
        match c {
            '#' => Ok(Self::Wall),
            '@' => Ok(Self::Robot),
            'O' => Ok(Self::Box),
            '.' => Ok(Self::Empty),
            _ => Err("Received an invalid character"),
        }
    }

    fn to_string(&self) -> ColoredString {
        match self {
            Self::Empty => Colorize::dimmed(" "),
            Self::Wall => Colorize::dimmed("#"),
            Self::Robot => Colorize::red("@"),
            Self::Box => Colorize::blue("█"),
            Self::BoxLeft => Colorize::blue("["),
            Self::BoxRight => Colorize::blue("]"),
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
                    Some(Coordinate::new(r as i64, i as i64))
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
            let mut term = Term::init();
            let mut sleep_time = 100000000;
            let mut paused = false;

            let mut moves = moves.iter();
            loop {
                let art = format!(
                    "Robot Position: {:?} \t \n{}\n\n",
                    self.robot_position,
                    self.ascii_art(),
                );
                term.draw(&art);
                if event::poll(Duration::from_nanos(sleep_time)).unwrap() {
                    if let Event::Key(KeyEvent { code, .. }) = event::read().unwrap() {
                        match code {
                            KeyCode::Char('q') | KeyCode::Esc => break,
                            KeyCode::Char(' ') => paused = !paused,
                            KeyCode::Up => sleep_time = (sleep_time / 2).max(1),
                            KeyCode::Down => sleep_time *= 2,
                            _ => {}
                        }
                    }
                }
                thread::sleep(Duration::from_nanos(sleep_time));
                if paused {
                    continue;
                }

                if let Some(mv) = moves.next() {
                    let _ = self.step(mv);
                } else {
                    break;
                }
            }
        } else {
            for mv in moves {
                let _ = self.step(mv);
            }
        }
    }

    fn step(&mut self, mv: &Move) -> Result<(), &'static str> {
        self.move_field(self.robot_position, mv)
    }

    fn check_move(&self, pos: Coordinate<i64>, mv: &Move) -> bool {
        let target_pos = pos + mv.get_dir_coord();
        if !self.inbounds(target_pos) {
            return false;
        }

        let target = self.get_pos(target_pos);
        match target {
            Field::Wall => false,
            Field::Empty => true,
            Field::Box => self.check_move(target_pos, mv),
            Field::BoxLeft | Field::BoxRight => {
                // Try to move the box away
                self.check_move(target_pos, mv) && {
                    if [Move::Up, Move::Down].contains(mv) {
                        let mut other_box_pos = target_pos;
                        if target == Field::BoxLeft {
                            other_box_pos = other_box_pos + Coordinate::new(1, 0)
                        } else {
                            other_box_pos = other_box_pos + Coordinate::new(-1, 0)
                        };
                        self.check_move(other_box_pos, mv)
                    } else {
                        true
                    }
                }
            }
            Field::Robot => false,
        }
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
            Field::BoxLeft | Field::BoxRight => {
                if self.check_move(target_pos, mv) {
                    if [Move::Up, Move::Down].contains(mv) {
                        let mut other_box_pos = target_pos;
                        if target == Field::BoxLeft {
                            other_box_pos = other_box_pos + Coordinate::new(1, 0)
                        } else {
                            other_box_pos = other_box_pos + Coordinate::new(-1, 0)
                        };
                        if self.check_move(other_box_pos, mv) {
                            let _ = self.move_field(other_box_pos, mv);
                        } else {
                            return Err("Right side of box blocked");
                        }
                    }
                    let _ = self.move_field(target_pos, mv);

                    self.map[pos.y as usize][pos.x as usize] = self.get_pos(target_pos);
                    self.map[target_pos.y as usize][target_pos.x as usize] = current;
                } else {
                    return Err("Left Side of Box is blocked");
                }
            }
            Field::Robot => {
                // Don't do anything in that case
                return Ok(());
                // unreachable!("This should not have happened! Tried to move into robot")
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
                if [Field::Box, Field::BoxLeft].contains(&self.map[y][x]) {
                    box_positions.push(Coordinate::new(x as i64, y as i64));
                }
            }
        }
        box_positions
    }

    fn ascii_art(&self) -> String {
        self.map
            .iter()
            .map(|row| row.iter().map(|f| f.to_string()).join(""))
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

    println!("{}\n", map.ascii_art(),);
    let mut result = 0;
    for box_coord in map.get_all_boxes() {
        result += 100 * box_coord.y + box_coord.x;
    }

    result
}

pub fn solve_p2() -> i64 {
    let content = fs::read_to_string(INPUT_FILE).expect("Could not read input file");
    let (map, instructions): (Vec<Vec<Field>>, Vec<Vec<Move>>) =
        content.lines().partition_map(|line| {
            let line = line.trim();
            if line.starts_with("#") {
                Either::Left(
                    line.chars()
                        .map(|c| Field::from_char_double(c).unwrap())
                        .flatten()
                        .collect(),
                )
            } else {
                Either::Right(line.chars().map(|c| Move::from_char(c).unwrap()).collect())
            }
        });
    let mut map = Map::new(map);
    let instructions = instructions.iter().flatten().collect();
    let visualize = false;
    map.step_many(&instructions, visualize);

    println!("{}\n", map.ascii_art(),);
    let mut result = 0;
    for box_coord in map.get_all_boxes() {
        // let dy = box_coord.y.min((box_coord.y + 2 - map.height).abs());
        // let dx = box_coord.x.min((box_coord.x + 2 - map.width).abs());
        let dy = box_coord.y;
        let dx = box_coord.x;
        result += 100 * dy + dx;
    }

    result
}
