use std::{
    collections::HashSet,
    fs::File,
    io::Read,
    ops::{Add, Sub},
};

use anyhow::{anyhow, bail};

#[derive(Debug)]
struct Grid {
    values: Vec<i32>,
    width: usize,
    height: usize,
}
impl Grid {
    fn get(&self, point: Point) -> Option<i32> {
        if (0..self.width as isize).contains(&point.0)
            && (0..self.height as isize).contains(&point.1)
        {
            let ix = point.0 + point.1 * self.width as isize;
            self.values.get(ix as usize).copied()
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Hash, Eq, PartialEq)]
struct Point(isize, isize);
impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    U,
    D,
    L,
    R,
}
impl From<Dir> for Point {
    fn from(dir: Dir) -> Self {
        match dir {
            Dir::U => Point(0, 1),
            Dir::D => Point(0, -1),
            Dir::L => Point(-1, 0),
            Dir::R => Point(1, 0),
        }
    }
}

#[derive(Debug)]
struct Problem {
    start: Point,
    destination: Point,
    grid: Grid,
}

fn read_file(file_name: &str) -> String {
    let mut contents = String::new();
    File::open(file_name)
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();
    contents
}

fn parse_input(inputs: &str) -> anyhow::Result<Problem> {
    let rows: Vec<_> = inputs.lines().collect();

    let height = rows.len();
    if height == 0 {
        bail!("no rows")
    }

    let mut start = None;
    let mut destination = None;

    let width = rows[0].len();
    let mut values = vec![0; width * height];
    for y in 0..height {
        for (x, mut ch) in rows[y].char_indices() {
            if ch == 'S' {
                start = Some(Point(x as isize, y as isize));
                ch = 'a'
            }
            if ch == 'E' {
                destination = Some(Point(x as isize, y as isize));
                ch = 'z'
            }

            let v = ch as i32 - 'a' as i32;
            values[x + y * width] = v;
        }
    }

    let start = start.ok_or_else(|| anyhow!("missing start"))?;
    let destination = destination.ok_or_else(|| anyhow!("missing end"))?;

    Ok(Problem {
        start,
        destination,
        grid: Grid {
            width,
            height,
            values,
        },
    })
}

fn valid_moves_from(grid: &Grid, current: Point, history: &HashSet<Point>) -> [Option<Point>; 4] {
    let mut moves = [None; 4];
    let mut options = [None; 4];
    let mut i = 0;

    let current_height = grid.get(current).unwrap();

    for dir in [Dir::U, Dir::D, Dir::L, Dir::R] {
        let p = current + dir.into();

        if history.contains(&p) {
            continue;
        }

        if let Some(h) = grid.get(p) {
            if (h - 1) <= current_height {
                options[i] = Some((p, h));
                i += 1;
            }
        }
    }

    options.sort_by_key(|opt| match opt {
        None => -1,
        Some((_, h)) => *h,
    });

    let mut it = options.iter().rev();
    let mut j = 0;
    while let Some(Some((p, _))) = it.next() {
        moves[j] = Some(*p);
        j += 1;
    }

    moves
}

fn find_path(
    problem: &Problem,
    current: Point,
    history: &mut HashSet<Point>,
) -> Option<HashSet<Point>> {
    if current == problem.destination {
        return Some(history.clone());
    }

    let options = valid_moves_from(&problem.grid, current, history);
    let mut best_solution: Option<HashSet<Point>> = None;
    for next_move in options {
        if let Some(next_move) = next_move {
            // insert point and check if we've got a solution (recursively)
            history.insert(next_move);
            if let Some(found) = find_path(problem, next_move, history) {
                if best_solution.is_none() || found.len() < best_solution.as_ref().unwrap().len() {
                    best_solution = Some(found);
                }
            }
            // not found
            history.remove(&next_move);
        }
    }

    best_solution
}

fn part1(problem: &Problem) -> Option<usize> {
    let mut history = HashSet::default();
    let solution = find_path(problem, problem.start, &mut history);
    solution.map(|s| s.len())
}

fn main() -> anyhow::Result<()> {
    let inputs = read_file("input.txt");
    let problem = parse_input(&inputs)?;
    println!("{problem:?}");
    let solution = part1(&problem).unwrap();
    println!("{solution:?}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi
    "};

    #[test]
    fn parse_inputs_succeeds() {
        let problem = parse_input(TEST_INPUT).unwrap();
        println!("{problem:?}");
    }

    #[test]
    fn part1_correct() {
        let problem = parse_input(TEST_INPUT).unwrap();
        let solution = part1(&problem).unwrap();
        assert_eq!(solution, 31);
    }
}
