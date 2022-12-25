pub mod part1;
pub mod part2;

use std::{
    fmt::{Debug, Display, Write},
    ops::{Add, Mul, Sub},
};

use arrayvec::ArrayVec;
use common::*;
use gcd::Gcd;
use itertools::Itertools;
use nalgebra::DMatrix;

#[derive(Debug, Clone, Copy, Default, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}
impl Point {
    pub const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn to_coord(self) -> (usize, usize) {
        self.into()
    }
}
impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}
impl Mul for Point {
    type Output = Point;

    fn mul(self, rhs: Self) -> Self::Output {
        Point::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl From<Dir> for Point {
    fn from(value: Dir) -> Self {
        match value {
            Dir::N => Point::new(0, -1),
            Dir::S => Point::new(0, 1),
            Dir::W => Point::new(-1, 0),
            Dir::E => Point::new(1, 0),
        }
    }
}

impl From<Point> for (usize, usize) {
    fn from(value: Point) -> (usize, usize) {
        let x = value.x.try_into().expect("invalid x coordinate");
        let y = value.y.try_into().expect("invalid y coordinate");
        // note matrix coordinates are (row,col)
        (y, x)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Dir {
    N,
    S,
    W,
    E,
}

#[derive(Debug, Clone)]
pub struct Blizzard {
    origin: Point,
    dir: Dir,
}
impl Blizzard {
    pub fn location_at_time(&self, time: usize, rows: usize, cols: usize) -> Point {
        let delta = Point::from(self.dir) * Point::new(time as i64, time as i64);
        let current = self.origin + delta;
        Point::new(
            current.x.rem_euclid(cols as i64),
            current.y.rem_euclid(rows as i64),
        )
    }
}

#[derive(Debug, Clone)]
pub struct Problem {
    rows: usize,
    cols: usize,
    blizzards: Vec<Blizzard>,
    start: Point,
    end: Point,
    cycle_length: usize,
}
impl Problem {
    pub fn contains(&self, point: Point) -> bool {
        point.x >= 0 && point.y >= 0 && point.x < self.cols as i64 && point.y < self.rows as i64
    }

    pub fn next_phase(&self, curr_phase: usize) -> usize {
        (curr_phase + 1).rem_euclid(self.cycle_length)
    }

    pub fn cycle_length(&self) -> usize {
        self.cycle_length
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Copy)]
pub enum GridState {
    Blank,
    One(Dir),
    Multiple(usize),
}
impl Display for GridState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GridState::Blank => f.write_char('.'),
            GridState::One(d) => match d {
                Dir::N => f.write_char('^'),
                Dir::S => f.write_char('v'),
                Dir::W => f.write_char('<'),
                Dir::E => f.write_char('>'),
            },
            GridState::Multiple(m) => match m {
                0..=9 => write!(f, "{m}"),
                _ => f.write_char('+'),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProblemState<'a> {
    problem: &'a Problem,
    grid: DMatrix<GridState>,
}
impl<'a> ProblemState<'a> {
    pub fn with_time(problem: &Problem, time: usize) -> ProblemState {
        let mut grid = DMatrix::from_element(problem.rows, problem.cols, GridState::Blank);

        for bliz in &problem.blizzards {
            let loc = bliz.location_at_time(time, problem.rows, problem.cols);
            let loc: (usize, usize) = loc.try_into().unwrap();
            grid[loc] = match grid[loc] {
                GridState::Blank => GridState::One(bliz.dir),
                GridState::One(_) => GridState::Multiple(2),
                GridState::Multiple(m) => GridState::Multiple(m + 1),
            }
        }

        ProblemState { problem, grid }
    }

    pub fn available_moves(&self, curr_loc: Point) -> ArrayVec<Point, 5> {
        let mut avail = ArrayVec::new();

        let deltas = [
            Point::new(0, 0),
            Dir::N.into(),
            Dir::E.into(),
            Dir::S.into(),
            Dir::W.into(),
        ];

        // check directions - can move into a blank space, or to start or end
        for d in deltas {
            let new_loc = curr_loc + d;
            let valid = match new_loc {
                p if p == self.problem.start => true,
                p if p == self.problem.end => true,
                p => self.problem.contains(p) && self.grid[p.to_coord()] == GridState::Blank,
            };
            if valid {
                avail.push(new_loc);
            }
        }
        avail
    }
}

impl<'a> Display for ProblemState<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.problem.rows {
            for x in 0..self.problem.cols {
                let g = self.grid[(y, x)];
                Display::fmt(&g, f)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn parse_input(input: &str) -> AnyResult<Problem> {
    let lines = input.lines().collect_vec();
    let rows = lines.len() - 2;
    let cols = lines.first().ok_anyhow()?.len() - 2;

    let gcd = rows.gcd(cols);
    let cycle_length = rows * cols / gcd;
    println!("rows {rows} cols {cols} gcd {gcd} cycle {cycle_length}");

    let mut blizzards = vec![];
    for y in 0..rows {
        let line = lines[y + 1];
        for (x, ch) in line.chars().skip(1).take(cols).enumerate() {
            let dir = match ch {
                '>' => Some(Dir::E),
                '<' => Some(Dir::W),
                '^' => Some(Dir::N),
                'v' => Some(Dir::S),
                _ => None,
            };

            if let Some(d) = dir {
                blizzards.push(Blizzard {
                    origin: Point::new(x as i64, y as i64),
                    dir: d,
                })
            }
        }
    }

    let start_x = lines
        .first()
        .ok_anyhow()?
        .chars()
        .position(|c| c == '.')
        .ok_anyhow()?
        - 1;

    let end_x = lines
        .last()
        .ok_anyhow()?
        .chars()
        .position(|c| c == '.')
        .ok_anyhow()?
        - 1;

    Ok(Problem {
        rows,
        cols,
        cycle_length,
        blizzards,
        start: Point::new(start_x as i64, -1),
        end: Point::new(end_x as i64, rows as i64),
    })
}
