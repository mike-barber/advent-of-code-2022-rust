use std::{fs::File, io::Read};

use day14::GridSquare::*;
use day14::*;

#[derive(Debug, Clone)]
struct Problem {
    grid: Grid<GridSquare>,
    sand_origin: Point,
}
impl Problem {
    // drop sand and find resting location, None if we fall off the grid
    fn drop_sand(&self) -> Option<Point> {
        let delta_below = Point(0, 1);
        let delta_left = Point(-1, 1);
        let delta_right = Point(1, 1);

        let mut cur = self.sand_origin;
        loop {
            // check move down
            let point_below = cur + delta_below;
            let below = self.grid.get(&point_below)?;
            if *below == Blank {
                cur = point_below;
                continue;
            }

            // check move left and right
            let point_left = cur + delta_left;
            let left = self.grid.get(&point_left)?;
            if *left == Blank {
                cur = point_left;
                continue;
            }

            let point_right = cur + delta_right;
            let right = self.grid.get(&point_right)?;
            if *right == Blank {
                cur = point_right;
                continue;
            }

            // no more moves left; sand grain comes to rest here
            return Some(cur);
        }
    }
}

fn read_file(file_name: &str) -> anyhow::Result<String> {
    let mut contents = String::new();
    File::open(file_name)?.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse_rocks(line: &str, x_offset: isize) -> anyhow::Result<Vec<Point>> {
    let points = line
        .split(" -> ")
        .map(|seg| {
            let mut seg_iter = seg.split(",");
            let x: isize = seg_iter.next().ok_anyhow()?.parse()?;
            let y: isize = seg_iter.next().ok_anyhow()?.parse()?;
            Ok(Point(x - x_offset, y))
        })
        .collect();

    points
}

fn parse_input(test_input: &str) -> anyhow::Result<Problem> {
    let x_offset = 450;
    let lines = test_input.lines().collect::<Vec<_>>();

    let rocks: anyhow::Result<Vec<_>> = lines
        .iter()
        .map(|line| parse_rocks(line, x_offset))
        .collect();
    let rocks = rocks?;

    // determine dimensions and create grid
    let max_x = rocks.iter().flatten().map(|p| p.0).max().ok_anyhow()?;
    let max_y = rocks.iter().flatten().map(|p| p.1).max().ok_anyhow()?;
    let mut grid = Grid::new(max_x as usize + 1, max_y as usize + 1, Blank);

    // populate grid with the rocks
    for rock in rocks {
        for pair in rock.windows(2) {
            if let [a, b] = pair {
                let dir = (*b - *a).signum();
                let mut cur = *a;
                while cur != *b {
                    *grid.get_mut(&cur).ok_anyhow()? = Rock;
                    cur = cur + dir;
                }
                *grid.get_mut(&cur).ok_anyhow()? = Rock;
            }
        }
    }

    let sand_origin = Point(500 - x_offset, 0);
    Ok(Problem { grid, sand_origin })
}

fn part1(problem: &mut Problem) -> anyhow::Result<i32> {
    let mut came_to_rest = 0;
    while let Some(resting_location) = problem.drop_sand() {
        let entry = problem.grid.get_mut(&resting_location).ok_anyhow()?;
        *entry = Sand;
        came_to_rest += 1;

        // println!("At rest: {came_to_rest}");
        // println!("{}", problem.grid);
    }

    Ok(came_to_rest)
}

fn main() -> anyhow::Result<()> {
    let problem = parse_input(&read_file("input.txt")?)?;

    let mut problem_part1 = problem.clone();
    let res_part1 = part1(&mut problem_part1)?;
    println!("part 1 result: {res_part1}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        498,4 -> 498,6 -> 496,6
        503,4 -> 502,4 -> 502,9 -> 494,9
    "};

    #[test]
    fn parse_inputs_succeeds() {
        parse_input(TEST_INPUT).unwrap();
    }

    #[test]
    fn part1_correct() {
        let mut problem = parse_input(TEST_INPUT).unwrap();
        let res = part1(&mut problem).unwrap();
        // println!("{}", problem.grid);
        assert_eq!(res, 24);
    }
}
