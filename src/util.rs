use crossterm::{
    cursor,
    terminal::{self, ClearType},
    ExecutableCommand,
};
use std::io::Stdout;
use std::io::{stdout, Write};
use std::ops::{Add, Div, Mul, Rem};

#[macro_export]
macro_rules! time_function {
    ($func:path) => {{
        let start = std::time::Instant::now();
        let res = $func();
        let duration = start.elapsed();
        println!(
            "Function `{}` executed in {:?}",
            stringify!($func),
            duration
        );
        res
    }};
}

#[derive(Clone, Copy, Debug)]
pub struct Coordinate<T>
where
    // T: Ord,
    T: Mul,
    T: Add,
    T: Div,
    T: Copy,
    T: Rem,
{
    pub x: T,
    pub y: T,
}

impl<T: Mul + Add + Copy + Div + Rem> Coordinate<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Div<Output = T> + Mul + Rem + Add + Copy> Div<T> for Coordinate<T> {
    type Output = Coordinate<T>;

    fn div(self, rhs: T) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
impl<T: Mul<Output = T> + Rem + Div + Add + Copy> Mul<T> for Coordinate<T> {
    type Output = Coordinate<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T: Mul + Add<Output = T> + Rem + Div + Copy> Add for Coordinate<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        return Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

impl<T: Mul + Add + Rem<Output = T> + Div + Copy> Rem for Coordinate<T> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        return Self::Output {
            x: self.x % rhs.x,
            y: self.y % rhs.y,
        };
    }
}

pub struct Term {
    stdout: Stdout,
}
impl Term {
    pub fn init() -> Self {
        let mut stdout = stdout();
        terminal::enable_raw_mode().unwrap();
        stdout.execute(terminal::Clear(ClearType::All)).unwrap();
        stdout.execute(cursor::Hide).unwrap();

        Term { stdout }
    }

    pub fn draw(&mut self, content: &String) {
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
