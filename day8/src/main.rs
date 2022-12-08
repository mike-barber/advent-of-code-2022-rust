use anyhow::bail;
use std::{fs::File, io::Read};

#[derive(Debug)]
struct Grid {
    values: Vec<u8>,
    width: usize,
    height: usize,
}
impl Grid {
    fn get(&self, x: isize, y: isize) -> Option<u8> {
        if (0..self.width as isize).contains(&x) && (0..self.height as isize).contains(&y) {
            let ix = x + y * self.height as isize;
            self.values.get(ix as usize).copied()
        } else {
            None
        }
    }

    fn visible_from_edge(&self, x: usize, y: usize) -> bool {
        // edge always visible
        if x == 0 || y == 0 || x == self.width - 1 || y == self.height - 1 {
            return true;
        }

        let x = x as isize;
        let y = y as isize;
        self.scan_all_lower(x, y, -1, 0)
            || self.scan_all_lower(x, y, 1, 0)
            || self.scan_all_lower(x, y, 0, -1)
            || self.scan_all_lower(x, y, 0, 1)
    }

    fn scan_all_lower(&self, mut x: isize, mut y: isize, dx: isize, dy: isize) -> bool {
        let height = self.get(x, y).expect("not on grid");
        loop {
            x += dx;
            y += dy;
            if let Some(h) = self.get(x, y) {
                if h >= height {
                    return false;
                }
            } else {
                break;
            }
        }
        true
    }

    fn scenic_score_from_tree(&self, x: usize, y: usize) -> usize {
        let x = x as isize;
        let y = y as isize;
        self.scan_visible_distance(x, y, -1, 0)
            * self.scan_visible_distance(x, y, 1, 0)
            * self.scan_visible_distance(x, y, 0, -1)
            * self.scan_visible_distance(x, y, 0, 1)
    }

    fn scan_visible_distance(&self, mut x: isize, mut y: isize, dx: isize, dy: isize) -> usize {
        let own_height = self.get(x, y).expect("not on grid");
        let mut distance = 0;
        loop {
            x += dx;
            y += dy;
            
            if let Some(h) = self.get(x, y) {
                distance += 1;
                if h >= own_height {
                    break;
                }
            } else {
                break;
            }
        }
        distance
    }
}
impl TryFrom<&[&str]> for Grid {
    type Error = anyhow::Error;

    fn try_from(rows: &[&str]) -> Result<Self, Self::Error> {
        let height = rows.len();
        if height == 0 {
            bail!("no rows")
        }

        let width = rows[0].len();
        let mut values = vec![0; width * height];
        for y in 0..height {
            for (x, ch) in rows[y].char_indices() {
                let v = (ch as u32 - '0' as u32) as u8;
                values[x + y * height] = v;
            }
        }

        Ok(Grid {
            width,
            height,
            values,
        })
    }
}

fn read_file(file_name: &str) -> String {
    let mut contents = String::new();
    File::open(file_name)
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();
    contents
}

fn parse_input(input: &str) -> anyhow::Result<Grid> {
    let lines = input.lines().collect::<Vec<_>>();
    Grid::try_from(lines.as_slice())
}

fn part1(grid: &Grid) -> usize {
    let mut visible_count = 0;
    for x in 0..grid.width {
        for y in 0..grid.height {
            if grid.visible_from_edge(x, y) {
                visible_count += 1;
            }
        }
    }
    visible_count
}

fn part2(grid: &Grid) -> usize {
    let mut max_score = 0;
    for x in 0..grid.width {
        for y in 0..grid.height {
            max_score = max_score.max(grid.scenic_score_from_tree(x, y));
        }
    }
    max_score
}

fn main() -> anyhow::Result<()> {
    let grid = parse_input(&read_file("input.txt"))?;

    let part1_res = part1(&grid);
    println!("part 1 result = {part1_res}");

    let part2_res = part2(&grid);
    println!("part 2 result = {part2_res}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        30373
        25512
        65332
        33549
        35390
    "};

    #[test]
    fn parse_inputs_succeeds() {
        parse_input(TEST_INPUT).unwrap();
    }

    #[test]
    fn part1_correct() {
        let grid = parse_input(TEST_INPUT).unwrap();
        let res = part1(&grid);
        assert_eq!(res, 21);
    }

    #[test]
    fn scenic_score_correct() {
        let grid = parse_input(TEST_INPUT).unwrap();
        assert_eq!(grid.scenic_score_from_tree(2, 1), 4);
        assert_eq!(grid.scenic_score_from_tree(2, 3), 8);
    }

    #[test]
    fn part2_correct() {
        let grid = parse_input(TEST_INPUT).unwrap();
        let res = part2(&grid);
        assert_eq!(res, 8);
    }
}
