use std::{
    collections::{HashMap, HashSet},
    fmt::{Display, Write},
    ops::{Add, Sub},
};

use common::{read_file, AnyResult};

#[derive(Debug, Clone, Copy, Default, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}
impl Point {
    pub const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
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

#[derive(Debug, Clone, Copy)]
enum Dir {
    N,
    S,
    W,
    E,
}

const NUM_DIRECTIONS: usize = 4;
const DIRECTIONS: [Dir; NUM_DIRECTIONS] = [Dir::N, Dir::S, Dir::W, Dir::E];
const ADJACENT_OFFSETS: [Point; 8] = {
    // compile-time const function, hence the while loops
    let mut i = 0;
    let mut x = -1;
    let mut points = [Point::new(0, 0); 8];
    while x <= 1 {
        let mut y = -1;
        while y <= 1 {
            if x == 0 && y == 0 {
                y += 1;
                continue;
            }
            points[i] = Point::new(x, y);
            i += 1;
            y += 1;
        }
        x += 1;
    }
    points
};

fn adjacent_points_move(current: Point, dir: Dir) -> [Point; 3] {
    let moved = current + dir.into();
    match dir {
        Dir::N => [moved, moved + Dir::E.into(), moved + Dir::W.into()],
        Dir::S => [moved, moved + Dir::E.into(), moved + Dir::W.into()],
        Dir::W => [moved, moved + Dir::N.into(), moved + Dir::S.into()],
        Dir::E => [moved, moved + Dir::N.into(), moved + Dir::S.into()],
    }
}

#[derive(Debug, Clone)]
struct Elf {
    current_location: Point,
    proposed_location: Option<Point>,
    // TODO: consider whether we can move this to the Problem instead
    next_move_cycle: usize,
}
impl Elf {
    fn new(loc: Point) -> Elf {
        Elf {
            current_location: loc,
            proposed_location: None,
            next_move_cycle: 0,
        }
    }
}

#[derive(Debug, Clone)]
struct Problem {
    elves: Vec<Elf>,
    current_locations: HashSet<Point>,
    proposed_location_counts: HashMap<Point, usize>,
}
impl Problem {
    fn new_with_elves(elves: Vec<Elf>) -> Problem {
        let current_locations = elves.iter().map(|e| e.current_location).collect();
        Problem {
            elves,
            current_locations,
            proposed_location_counts: HashMap::new(),
        }
    }

    fn bounding_box(&self) -> (Point, Point) {
        let first = self.elves.first().unwrap();

        let mut min_x = first.current_location.x;
        let mut max_x = min_x;

        let mut min_y = first.current_location.y;
        let mut max_y = min_y;

        for elf in &self.elves {
            min_x = min_x.min(elf.current_location.x);
            max_x = max_x.max(elf.current_location.x);

            min_y = min_y.min(elf.current_location.y);
            max_y = max_y.max(elf.current_location.y);
        }
        (Point::new(min_x, min_y), Point::new(max_x, max_y))
    }

    fn count_empty_blocks(&self) -> usize {
        let bb = self.bounding_box();
        let mut empty = 0;
        for x in bb.0.x..=bb.1.x {
            for y in bb.0.y..=bb.1.y {
                if !self.current_locations.contains(&Point::new(x, y)) {
                    empty += 1;
                }
            }
        }
        empty
    }

    fn step_once(&mut self) -> usize {
        // phase 1 -- proposed moves
        self.proposed_location_counts.clear();
        for i in 0..self.elves.len() {
            let elf = self.elves.get_mut(i).unwrap();

            let mut updated_position = None;
            let next_move_cyle = elf.next_move_cycle;

            // elf wants to move if there are any elves adjacent to it
            let elf_wants_to_move = ADJACENT_OFFSETS
                .map(|m| elf.current_location + m)
                .iter()
                .any(|p| self.current_locations.contains(p));

            // determine the proposed move
            if elf_wants_to_move {
                for dir in DIRECTIONS
                    .iter()
                    .cycle()
                    .skip(next_move_cyle)
                    .take(NUM_DIRECTIONS)
                {
                    let adjacent = adjacent_points_move(elf.current_location, *dir);
                    if !adjacent.iter().any(|p| self.current_locations.contains(p)) {
                        // available: take this as the elf's proposed move
                        updated_position = Some(elf.current_location + Point::from(*dir));
                        break;
                    }
                }
            }
            elf.next_move_cycle = (elf.next_move_cycle + 1) % NUM_DIRECTIONS;

            if let Some(pos) = updated_position {
                elf.proposed_location = Some(pos);
                *self.proposed_location_counts.entry(pos).or_default() += 1;
            } else {
                elf.proposed_location = None;
            }
        }

        // phase 2 -- move to proposed locations if possible
        let mut count_moved = 0;
        for i in 0..self.elves.len() {
            let elf = self.elves.get_mut(i).unwrap();

            if let Some(pos) = elf.proposed_location {
                // move if the elf is the only one proposing the new location
                let dest_count = self.proposed_location_counts.get(&pos).unwrap();
                if *dest_count == 1 {
                    elf.current_location = pos;
                    count_moved += 1;
                }
            }
        }

        // and finally update the map
        self.current_locations.clear();
        for elf in &self.elves {
            self.current_locations.insert(elf.current_location);
        }

        count_moved
    }
}
impl Display for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bb = self.bounding_box();
        for y in bb.0.y..=bb.1.y {
            for x in bb.0.x..=bb.1.x {
                if self.current_locations.contains(&Point::new(x, y)) {
                    f.write_char('#')?;
                } else {
                    f.write_char('.')?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse_input(input: &str) -> AnyResult<Problem> {
    let mut elves = vec![];
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                elves.push(Elf::new(Point::new(x as i64, y as i64)))
            }
        }
    }
    Ok(Problem::new_with_elves(elves))
}

fn part1(input: &str) -> AnyResult<usize> {
    let mut problem = parse_input(input)?;
    for _ in 1..=10 {
        problem.step_once();
    }
    Ok(problem.count_empty_blocks())
}

fn main() -> AnyResult<()> {
    let input = read_file("day23/input.txt")?;

    println!("part1 result: {}", part1(&input)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        ..............
        ..............
        .......#......
        .....###.#....
        ...#...#.#....
        ....#...##....
        ...#.###......
        ...##.#.##....
        ....#..#......
        ..............
        ..............
        ..............
    "};

    const TEST_INPUT_SMALL: &str = indoc! {"
        .....
        ..##.
        ..#..
        .....
        ..##.
        .....
    "};

    #[test]
    fn parse_input_correct() {
        let problem = parse_input(TEST_INPUT).unwrap();
        dbg!(&problem);
        println!("{}", problem);
    }

    #[test]
    fn step_once_check_small() {
        let mut problem = parse_input(TEST_INPUT_SMALL).unwrap();
        println!("{problem}");
        for i in 1..=3 {
            let count = problem.step_once();
            println!("i: {i}, moved: {count}");
            println!("{problem}");
        }
        let expected = indoc! {"
            ..#..
            ....#
            #....
            ....#
            .....
            ..#..
        "};
        assert_eq!(problem.to_string(), expected);
    }

    #[test]
    fn step_once_check_larger() {
        let mut problem = parse_input(TEST_INPUT).unwrap();
        println!("{problem}");
        for i in 1..=10 {
            let count = problem.step_once();
            println!("i: {i}, moved: {count}");
            println!("{problem}");
        }

        let expected = indoc! {"
            ......#.....
            ..........#.
            .#.#..#.....
            .....#......
            ..#.....#..#
            #......##...
            ....##......
            .#........#.
            ...#.#..#...
            ............
            ...#..#..#..
        "};
        assert_eq!(problem.to_string(), expected);
        assert_eq!(problem.count_empty_blocks(), 110);
    }

    // #[test]
    // fn part1_correct() {
    //     let input = parse_input(TEST_INPUT).unwrap();
    //     let res = part1(&input).unwrap();
    //     assert_eq!(res, 152);
    // }

    // #[test]
    // fn part2_correct() {
    //     let input = parse_input(TEST_INPUT).unwrap();
    //     let res = part2(&input).unwrap();
    //     assert_eq!(res, 301);
    // }
}
