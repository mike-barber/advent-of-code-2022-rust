use anyhow::anyhow;
use std::{
    collections::HashSet,
    hash::Hash,
    ops::{Add, Sub},
};
use strum::EnumString;

#[derive(Debug, Clone, Copy, Default, Hash, Eq, PartialEq)]
struct Point(i32, i32);
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

#[derive(Debug, Clone, Copy, EnumString)]
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

#[derive(Debug, Clone, Copy)]
struct Instruction(Dir, usize);
impl TryFrom<&str> for Instruction {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut fields = value.split_whitespace();

        let dir_field = fields.next().ok_or_else(|| anyhow!("missing dir"))?;
        let repeat_field = fields.next().ok_or_else(|| anyhow!("missing count"))?;

        let dir: Dir = dir_field.try_into()?;
        let repeat: usize = repeat_field.parse()?;

        Ok(Instruction(dir, repeat))
    }
}

#[derive(Debug, Clone)]
struct Rope(Vec<Point>);
impl Rope {
    fn new(length: usize) -> Self {
        Rope(vec![Point::default(); length])
    }

    fn follow(leading: Point, current: Point) -> Point {
        let distance = leading - current;
        match distance {
            Point(-1..=1, -1..=1) => current,
            Point(x, 0) => leading - Point(x.signum(), 0),
            Point(0, y) => leading - Point(0, y.signum()),
            Point(x, y) => {
                let mv = Point(x.signum(), y.signum());
                current + mv
            }
        }
    }

    fn move_dir(self, dir: Dir) -> Rope {
        let mut points = self.0;

        // update head
        points[0] = points[0] + dir.into();

        // rest of the points follow the changes
        for i in 1..points.len() {
            points[i] = Self::follow(points[i - 1], points[i]);
        }

        // updated rope
        Rope(points)
    }

    fn tail(&self) -> Point {
        *self.0.last().unwrap()
    }
}

fn tail_position_count(instructions: &[Instruction], rope_length: usize) -> usize {
    let mut history = HashSet::new();
    let mut rope = Rope::new(rope_length);

    history.insert(rope.tail());
    for instruction in instructions {
        for _ in 0..instruction.1 {
            rope = rope.move_dir(instruction.0);
            history.insert(rope.tail());
        }
    }
    history.len()
}

fn part1(instructions: &[Instruction]) -> usize {
    tail_position_count(instructions, 2)
}

fn part2(instructions: &[Instruction]) -> usize {
    tail_position_count(instructions, 10)
}

fn parse_input(inputs: &str) -> anyhow::Result<Vec<Instruction>> {
    inputs.lines().map(Instruction::try_from).collect()
}

fn main() -> anyhow::Result<()> {
    let instructions = parse_input(&common::read_file("input.txt")?)?;

    let part1_res = part1(&instructions);
    println!("part 1 result = {part1_res}");

    let part2_res = part2(&instructions);
    println!("part 2 result = {part2_res}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;
    use indoc::indoc;

    const TEST_INPUT_PART1: &str = indoc! {"
        R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2
    "};

    const TEST_INPUT_PART2: &str = indoc! {"
        R 5
        U 8
        L 8
        D 3
        R 17
        D 10
        L 25
        U 20
    "};

    #[test]
    fn parse_inputs_succeeds() {
        parse_input(TEST_INPUT_PART1).unwrap();
    }

    #[test]
    fn part1_correct() {
        let instructions = parse_input(TEST_INPUT_PART1).unwrap();
        let res = part1(&instructions);
        assert_eq!(res, 13);
    }

    #[test]
    fn part2_correct() {
        let instructions = parse_input(TEST_INPUT_PART2).unwrap();
        let res = part2(&instructions);
        assert_eq!(res, 36);
    }
}
