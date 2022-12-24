use std::{
    fmt::{Debug, Display, Write},
    ops::{Add, Mul, Sub},
};

use arrayvec::ArrayVec;
use common::*;
use gcd::Gcd;
use itertools::Itertools;
use nalgebra::{DMatrix, distance};
use priority_queue::PriorityQueue;
use rustc_hash::{FxHashMap, FxHashSet};

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
    fn from(value: Point) -> (usize,usize) {
        let x = value.x.try_into().expect("invalid x coordinate");
        let y = value.y.try_into().expect("invalid y coordinate");
        (x, y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum Dir {
    N,
    S,
    W,
    E,
}

#[derive(Debug, Clone)]
struct Blizzard {
    origin: Point,
    dir: Dir,
}
impl Blizzard {
    fn location_at_time(&self, time: usize, rows: usize, cols: usize) -> Point {
        let delta = Point::from(self.dir) * Point::new(time as i64, time as i64);
        let current = self.origin + delta;
        Point::new(
            current.x.rem_euclid(cols as i64),
            current.y.rem_euclid(rows as i64),
        )
    }
}

#[derive(Debug, Clone)]
struct Problem {
    rows: usize,
    cols: usize,
    blizzards: Vec<Blizzard>,
    start: Point,
    end: Point,
    cycle_length: usize,
}
impl Problem {
    fn contains(&self, point: Point) -> bool {
        point.x >= 0 && point.y >= 0 && point.x < self.cols as i64 && point.y < self.rows as i64
    }

    fn next_phase(&self, curr_phase: usize) -> usize {
        (curr_phase + 1).rem_euclid(self.cycle_length)
    }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Copy)]
enum GridState {
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
struct ProblemState<'a> {
    problem: &'a Problem,
    time: usize,
    grid: DMatrix<GridState>,
}
impl<'a> ProblemState<'a> {
    fn with_time(problem: &Problem, time: usize) -> ProblemState {
        let mut grid = DMatrix::from_element(problem.cols, problem.rows, GridState::Blank);

        for bliz in &problem.blizzards {
            let loc = bliz.location_at_time(time, problem.rows, problem.cols);
            let loc: (usize, usize) = loc.try_into().unwrap();
            grid[loc] = match grid[loc] {
                GridState::Blank => GridState::One(bliz.dir),
                GridState::One(_) => GridState::Multiple(2),
                GridState::Multiple(m) => GridState::Multiple(m + 1),
            }
        }

        ProblemState {
            problem,
            grid,
            time,
        }
    }

    fn available_moves(&self, curr_loc: Point) -> ArrayVec<Point, 5> {
        let mut avail = ArrayVec::new();

        let deltas = [
            Point::new(0,0),
            Dir::N.into(),
            Dir::E.into(),
            Dir::S.into(),
            Dir::W.into()
        ];

        // check directions - can move into a blank space, or to start or end
        for d in deltas {
            let new_loc = curr_loc + d;
            let valid = match new_loc {
                p if p == self.problem.start => true,
                p if p == self.problem.end => true,
                p => self.problem.contains(p) && self.grid[p.to_coord()] == GridState::Blank
            };
            if valid {
                avail.push(new_loc);
            }
        }
        avail
    }

    // fn phase(&self) -> usize {
    //     self.time.rem_euclid(self.problem.cycle_length)
    // }

    // fn next_phase(&self) -> usize {
    //     (self.time + 1).rem_euclid(self.problem.cycle_length)
    // }
    
}

impl<'a> Display for ProblemState<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.problem.rows {
            for x in 0..self.problem.cols {
                let g = self.grid[(x, y)];
                Display::fmt(&g, f)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse_input(input: &str) -> AnyResult<Problem> {
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
        .ok_anyhow()?;
    let end_x = lines
        .last()
        .ok_anyhow()?
        .chars()
        .position(|c| c == '.')
        .ok_anyhow()?;

    Ok(Problem {
        rows,
        cols,
        cycle_length,
        blizzards,
        start: Point::new(start_x as i64, -1),
        end: Point::new(end_x as i64, rows as i64),
    })
}

#[derive(Debug,Copy,Clone,Hash, PartialEq, Eq)]
struct PosState {
    phase: usize,
    loc: Point,
}
impl PosState {
    fn new(phase: usize, loc: Point) -> Self {
        PosState { phase, loc }
    }
}

fn find_shortest_path(problem: &Problem) -> Option<usize> {
    const DIST_INIT: i32 = i32::MAX/2;
    const PRIO_INIT: i32 = i32::MIN;
    
    let states = (0..problem.cycle_length)
        .map(|t| ProblemState::with_time(problem, t))
        .collect_vec();

    let mut dist: FxHashMap<PosState, i32> = FxHashMap::default();
    let mut prev: FxHashMap<PosState, Option<PosState>> = FxHashMap::default();
    let mut discovered: FxHashSet<PosState> = FxHashSet::default();

    // initialise
    let start = PosState::new(0, problem.start);
    let mut queue = PriorityQueue::new();
    queue.push(start, 0);
    dist.insert(start, 0);
    prev.insert(start, None);

    while let Some((u, _prio)) = queue.pop() {
        let next_phase = problem.next_phase(u.phase);
        let next_state = &states[next_phase];

        let valid_moves = next_state.available_moves(u.loc);
        for v_point in valid_moves {
            let v = PosState::new(next_phase, v_point);

            if !discovered.contains(&v) {
                queue.push(v, PRIO_INIT);
                dist.insert(v, DIST_INIT);
                prev.insert(v, None);
                discovered.insert(v);
            }

            if queue.get(&v).is_some() {
                // distance is to current node (u) + 1
                let alt = dist.get(&u).unwrap() + 1;
                if alt < *dist.get(&v).unwrap() {
                    // update distances to this node, and record how we got here
                    *dist.get_mut(&v).unwrap() = alt;
                    *prev.get_mut(&v).unwrap() = Some(u);
                    queue.change_priority(&v, -alt);
                }
            }
        }
    }

    for dest_state in (0..problem.cycle_length).map(|p| PosState::new(p, problem.end)) {
        println!("state {dest_state:?} => {:?}", dist.get(&dest_state));
    }

    todo!()
}

fn main() -> AnyResult<()> {
    let input = read_file("day24/input.txt")?;

    let problem = parse_input(&input)?;

    for t in 0..3 {
        let state = ProblemState::with_time(&problem, t);
        println!("time = {t}");
        println!("{state}");
    }

    let init = ProblemState::with_time(&problem, 0);
    for cycle in 0..3 {
        let t = cycle * problem.cycle_length;
        let state = ProblemState::with_time(&problem, t);
        println!("cycle {cycle} time = {t}");
        println!("{state}");

        assert_eq!(init.to_string(), state.to_string());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT_BASIC: &str = indoc! {"
        #.#####
        #.....#
        #>....#
        #.....#
        #...v.#
        #.....#
        #####.#
    "};

    const TEST_INPUT_COMPLEX: &str = indoc! {"
        #.######
        #>>.<^<#
        #.<..<<#
        #>v.><>#
        #<^v^^>#
        ######.#
    "};

    #[test]
    fn parse_input_correct() {
        let problem = parse_input(TEST_INPUT_COMPLEX).unwrap();
        dbg!(&problem);
    }

    #[test]
    fn run_cycles_basic() {
        let problem = parse_input(TEST_INPUT_BASIC).unwrap();
        println!("cycles: {}", problem.cycle_length);
        for t in 0..6 {
            let state = ProblemState::with_time(&problem, t);
            println!("time {t}");
            println!("{state}");
            println!();
        }
    }

    fn test_cycles(input: &str) {
        let problem = parse_input(input).unwrap();

        let init = ProblemState::with_time(&problem, 0);
        let init_str = init.to_string();

        // assert that we keep cycling successfully
        for c in 0..5 {
            let state = ProblemState::with_time(&problem, c * problem.cycle_length);
            let state_str = state.to_string();
            assert_eq!(init_str, state_str);
        }
    }

    #[test]
    fn run_cycles_simple() {
        test_cycles(TEST_INPUT_COMPLEX);
    }

    #[test]
    fn run_cycles_complex() {
        test_cycles(TEST_INPUT_COMPLEX);
    }

    #[test]
    fn find_shortest_path_correct() {
        let problem = parse_input(TEST_INPUT_COMPLEX).unwrap();
        find_shortest_path(&problem);
    }

    // #[test]
    // fn step_once_check_larger() {
    //     let mut problem = parse_input(TEST_INPUT).unwrap();
    //     println!("{problem}");
    //     for i in 1..=10 {
    //         let count = problem.step_once();
    //         println!("i: {i}, moved: {count}");
    //         println!("{problem}");
    //     }

    //     let expected = indoc! {"
    //     "};
    //     assert_eq!(problem.to_string(), expected);
    //     assert_eq!(problem.count_empty_blocks(), 110);
    // }

    // #[test]
    // fn part2_correct() {
    //     let res = part2(TEST_INPUT).unwrap();
    //     assert_eq!(res, 20);
    // }
}
