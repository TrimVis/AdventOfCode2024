use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{self, ClearType},
    ExecutableCommand,
};
use std::io::{stdout, Write};
use std::{fs, io::Stdout};
use std::{thread, time::Duration};

use crate::util::Coordinate;

// const INPUT_FILE: &str = "inputs/day14.test";
const INPUT_FILE: &str = "inputs/day14.input";

pub fn solve() {
    println!("Part 1: {}", solve_p1());
    println!("Part 2: {}", solve_p2());
}

#[derive(Debug)]
struct Robot {
    position: Coordinate<i64>,
    velocity: Coordinate<i64>,
}

impl Robot {
    fn from_string(line: &str) -> Robot {
        let (p, v) = line.split_once(" ").unwrap();
        let p = p.split_once("=").unwrap().1.split_once(",").unwrap();
        let v = v.split_once("=").unwrap().1.split_once(",").unwrap();
        let position = Coordinate::new(p.0.parse().unwrap(), p.1.parse().unwrap());
        let velocity = Coordinate::new(v.0.parse().unwrap(), v.1.parse().unwrap());

        Robot { position, velocity }
    }

    fn step(&mut self, n: i64, map_corner: Option<Coordinate<i64>>) {
        self.position = self.position + self.velocity * n;
        if let Some(map_corner) = map_corner {
            self.position = self.position % map_corner;
            // Do it again, so we have a positive number
            self.position = self.position + map_corner;
            self.position = self.position % map_corner;
        }
    }
}

struct Robots {
    robots: Vec<Robot>,
    map_corner: Coordinate<i64>,
}

impl Robots {
    fn from_string(content: &str, map_corner: Coordinate<i64>) -> Robots {
        let mut robots = vec![];
        for line in content.lines() {
            robots.push(Robot::from_string(line));
        }

        Robots { robots, map_corner }
    }

    fn step(&mut self, n: i64) {
        self.robots
            .iter_mut()
            .for_each(|r| r.step(n, Some(self.map_corner)));
    }

    fn ascii_art(&self, double_width: bool) -> String {
        let height = self.map_corner.y as usize;
        let mut width = self.map_corner.x as usize;
        if double_width {
            width *= 2;
        }

        let mut bitmap: Vec<Vec<char>> = vec![vec![' '; width + 3]; height + 2];
        self.robots.iter().for_each(|r| {
            let x = r.position.x as usize + 1;
            let y = r.position.y as usize + 1;
            if !double_width {
                bitmap[y][x] = '█';
            } else {
                bitmap[y][2 * x] = '█';
                bitmap[y][2 * x - 1] = '█';
            }
        });

        for (i, row) in bitmap.iter_mut().enumerate() {
            if i == 0 || i == height + 1 {
                row.fill('=')
            }
            row[0] = '#';
            row[width + 1] = '#';
            row[width + 2] = '\n';
        }

        String::from_iter(bitmap.iter().flatten())
    }
}

fn get_safety_factor(map_corner: Coordinate<i64>, positions: Vec<Coordinate<i64>>) -> usize {
    let center = map_corner / 2;
    let mut quadrants: [usize; 4] = [0, 0, 0, 0];
    for position in positions {
        if position.x == center.x || position.y == center.y {
            continue;
        }
        let q = match (position.x < center.x, position.y < center.y) {
            (true, false) => 3,
            (false, true) => 2,
            (false, false) => 1,
            (true, true) => 0,
        };
        quadrants[q] += 1;
    }

    quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3]
}

pub fn solve_p1() -> usize {
    let content = fs::read_to_string(INPUT_FILE).expect("Could not read input file");
    let mut final_positions: Vec<Coordinate<i64>> = vec![];

    // let corner = Coordinate::new(11, 7);
    let corner = Coordinate::new(101, 103);
    for line in content.lines() {
        let mut robot = Robot::from_string(line);
        robot.step(100, Some(corner));
        final_positions.push(robot.position);
    }
    get_safety_factor(corner, final_positions)
}

struct Term {
    stdout: Stdout,
}
impl Term {
    fn init() -> Self {
        let mut stdout = stdout();
        terminal::enable_raw_mode().unwrap();
        stdout.execute(terminal::Clear(ClearType::All)).unwrap();
        stdout.execute(cursor::Hide).unwrap();

        Term { stdout }
    }

    fn draw(&mut self, content: &String) {
        for (i, line) in content.lines().enumerate() {
            self.stdout.execute(cursor::MoveTo(0, i as u16)).unwrap();
            write!(self.stdout, "{}", line).unwrap();
        }
        self.stdout.flush().unwrap();
    }
}

impl Drop for Term {
    fn drop(&mut self) {
        self.stdout.execute(cursor::Show).unwrap();
        terminal::disable_raw_mode().unwrap();
    }
}

pub fn solve_p2() -> i64 {
    // let corner = Coordinate::new(11, 7);
    let corner = Coordinate::new(101, 103);

    let content = fs::read_to_string(INPUT_FILE).expect("Could not read input file");
    let mut robots = Robots::from_string(content.as_str(), corner);

    // Visualizes the tree appearing
    let mut sleep_time = 100000000;
    let mut paused = false;
    let mut reverse = false;
    let mut term = Term::init();
    let mut frame = 0;
    loop {
        if event::poll(Duration::from_nanos(sleep_time)).unwrap() {
            if let Event::Key(KeyEvent { code, .. }) = event::read().unwrap() {
                match code {
                    KeyCode::Up => {
                        sleep_time /= 2;
                    }
                    KeyCode::Down => {
                        sleep_time *= 2;
                    }
                    KeyCode::Left => {
                        frame -= 1;
                        robots.step(-1);
                    }
                    KeyCode::Right => {
                        frame += 1;
                        robots.step(1);
                    }
                    KeyCode::Char(' ') => {
                        paused = !paused;
                    }
                    KeyCode::Char('r') => {
                        reverse = !reverse;
                    }
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    _ => {}
                }
            }
        }
        if !paused {
            let step = if reverse { -1 } else { 1 };
            frame += step;
            robots.step(step);
        }
        let fb = format!(
            "[Frame {:4.}] \t \t [{}]{} \n{}",
            frame,
            if paused { "PAUSED" } else { "RUNNING" },
            if reverse { "(REVERSING)" } else { "" },
            robots.ascii_art(true)
        );
        term.draw(&fb);
        thread::sleep(Duration::from_nanos(sleep_time));
    }

    0
}
