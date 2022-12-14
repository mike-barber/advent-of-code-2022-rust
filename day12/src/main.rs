use std::{
    collections::HashMap,
    ops::{Add, Sub},
};

use anyhow::{anyhow, bail};
use priority_queue::PriorityQueue;

#[derive(Debug)]
struct Grid<T> {
    values: Vec<T>,
    width: usize,
    height: usize,
}
impl<T> Grid<T> {
    fn get(&self, point: &Point) -> Option<&T> {
        if self.contains(point) {
            let ix = point.0 + point.1 * self.width as isize;
            self.values.get(ix as usize)
        } else {
            None
        }
    }

    fn contains(&self, point: &Point) -> bool {
        (0..self.width as isize).contains(&point.0) && (0..self.height as isize).contains(&point.1)
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
    grid: Grid<i32>,
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

fn valid_moves(grid: &Grid<i32>, current: Point) -> [Option<Point>; 4] {
    let mut moves = [None; 4];
    let mut i = 0;

    let current_height = grid.get(&current).unwrap();
    for dir in [Dir::U, Dir::D, Dir::L, Dir::R] {
        let p = current + dir.into();

        if let Some(h) = grid.get(&p) {
            if (h - 1) <= *current_height {
                moves[i] = Some(p);
                i += 1;
            }
        }
    }

    moves
}

// Dijkstra's shortest path algorith, using distances between vertices as 1
fn find_path_dijkstra(problem: &Problem, start: Point) -> Option<Vec<Point>> {
    let mut dist: HashMap<Point, i32> = HashMap::new();
    let mut prev: HashMap<Point, Option<Point>> = HashMap::new();

    // initialise problem
    let mut q = PriorityQueue::new();
    for x in 0..problem.grid.width {
        for y in 0..problem.grid.height {
            let point = Point(x as isize, y as isize);
            dist.insert(point, i32::MAX / 2);
            prev.insert(point, None);
            // queue priority is highest first
            q.push(point, i32::MIN);
        }
    }
    *dist.get_mut(&start).unwrap() = 0;
    q.change_priority(&start, 0);

    // update all the reachable nodes
    while let Some((u, _)) = q.pop() {
        let valid_moves = valid_moves(&problem.grid, u);
        for v in valid_moves {
            // consider valid neighbours still in the queue
            if v.is_none() {
                continue;
            }
            let v = v.unwrap();
            if q.get(&v).is_some() {
                // distance is to current node (u) + 1
                let alt = dist.get(&u).unwrap() + 1;
                if alt < *dist.get(&v).unwrap() {
                    // update distances to this node, and record how we got here
                    *dist.get_mut(&v).unwrap() = alt;
                    *prev.get_mut(&v).unwrap() = Some(u);
                    q.change_priority(&v, -alt);
                }
            }
        }
    }

    // reverse iteration over previous elements to find the path
    let mut path = Vec::new();
    let mut u = problem.destination;
    path.push(u);
    while let Some(Some(v)) = prev.get(&u) {
        u = *v;
        path.push(u)
    }

    if path.last().unwrap() == &start {
        Some(path)
    } else {
        None
    }
}

fn part1(problem: &Problem) -> Option<usize> {
    let solution = find_path_dijkstra(problem, problem.start);
    solution.map(|s| s.len() - 1)
}

// This could be more efficient -- run Dijkstra in reverse from the end,
// and then consider the shortest path found to any destination; however,
// this would require re-engineering the feasibility function as well as
// the termination conditions. This is good enough for answering the question.
fn part2(problem: &Problem) -> usize {
    let mut minimum_steps = usize::MAX;
    for x in 0..problem.grid.width as isize {
        for y in 0..problem.grid.height as isize {
            let p = Point(x, y);
            let h = problem.grid.get(&p).unwrap();
            // height a is zero
            if *h == 0 {
                if let Some(path) = find_path_dijkstra(problem, p) {
                    minimum_steps = minimum_steps.min(path.len() - 1);
                }
            }
        }
    }
    minimum_steps
}

fn main() -> anyhow::Result<()> {
    let inputs = common::read_file("input.txt")?;
    let problem = parse_input(&inputs)?;

    let res1 = part1(&problem).unwrap();
    println!("part1: {res1:?}");

    let res2 = part2(&problem);
    println!("part2: {res2}");

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
        parse_input(TEST_INPUT).unwrap();
    }

    #[test]
    fn part1_correct() {
        let problem = parse_input(TEST_INPUT).unwrap();
        let solution = part1(&problem).unwrap();
        assert_eq!(solution, 31);
    }

    #[test]
    fn part2_correct() {
        let problem = parse_input(TEST_INPUT).unwrap();
        let solution = part2(&problem);
        assert_eq!(solution, 29);
    }
}
