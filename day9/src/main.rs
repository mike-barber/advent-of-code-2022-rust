use anyhow::anyhow;
use std::{
    collections::HashSet,
    default,
    fs::File,
    hash::Hash,
    io::Read,
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

        let dir_field = fields.next().ok_or(anyhow!("missing dir"))?;
        let repeat_field = fields.next().ok_or(anyhow!("missing count"))?;

        let dir: Dir = dir_field.try_into()?;
        let repeat: usize = repeat_field.parse()?;

        Ok(Instruction(dir, repeat))
    }
}

#[derive(Debug, Clone, Default)]
struct RopeSegment {
    head: Point,
    tail: Point,
}
impl RopeSegment {
    fn move_dir(self, dir: Dir) -> RopeSegment {
        // new head position
        let new_head = self.head + dir.into();

        // new tail position
        let distance = new_head - self.tail;
        let new_tail = match distance {
            Point(-1..=1, -1..=1) => self.tail,
            Point(x, 0) => new_head - Point(x.signum(), 0),
            Point(0, y) => new_head - Point(0, y.signum()),
            Point(x, y) => {
                let mv = Point(x.signum(), y.signum());
                self.tail + mv
            }
        };

        // println!(
        //     "head {new_head:?} {new_tail:?} -- distance {distance:?} tail {tt:?} -> {new_tail:?}",
        //     tt = self.tail
        // );

        RopeSegment {
            head: new_head,
            tail: new_tail,
        }
    }
}

#[derive(Debug, Clone)]
struct Rope(Vec<RopeSegment>);
impl Rope {
    fn new(len: usize) -> Self {
        Rope(vec![RopeSegment::default(); len])
    }

    fn move_dir(self, dir: Dir) -> Rope {
        let mut d = dir;
        let mut segments = self.0;
        for seg in &mut segments {
            let prior = *seg.tail;
            seg.move_dir(dir)
        }

        todo!()
    }
}

fn part1(instructions: &[Instruction]) -> usize {
    let mut history = HashSet::new();
    let mut rope = RopeSegment::default();

    history.insert(rope.tail);
    for instruction in instructions {
        for _ in 0..instruction.1 {
            rope = rope.move_dir(instruction.0);
            history.insert(rope.tail);
        }
    }

    history.len()
}

fn read_file(file_name: &str) -> String {
    let mut contents = String::new();
    File::open(file_name)
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();
    contents
}

fn parse_input(inputs: &str) -> anyhow::Result<Vec<Instruction>> {
    inputs.lines().map(Instruction::try_from).collect()
}

fn main() -> anyhow::Result<()> {
    let instructions = parse_input(&read_file("input.txt"))?;

    let part1_res = part1(&instructions);
    println!("part 1 result = {part1_res}");

    // let part2_res = part2(&grid);
    // println!("part 2 result = {part2_res}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2
    "};

    #[test]
    fn parse_inputs_succeeds() {
        parse_input(TEST_INPUT).unwrap();
    }

    #[test]
    fn part1_correct() {
        let instructions = parse_input(TEST_INPUT).unwrap();
        let res = part1(&instructions);
        assert_eq!(res, 13);
    }

    // #[test]
    // fn scenic_score_correct() {
    //     let grid = parse_input(TEST_INPUT).unwrap();
    //     assert_eq!(grid.scenic_score_from_tree(2, 1), 4);
    //     assert_eq!(grid.scenic_score_from_tree(2, 3), 8);
    // }

    // #[test]
    // fn part2_correct() {
    //     let grid = parse_input(TEST_INPUT).unwrap();
    //     let res = part2(&grid);
    //     assert_eq!(res, 8);
    // }
}
