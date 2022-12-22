pub mod part1;
pub mod part2;

use std::fmt::{Display, Write};

use nalgebra::DMatrix;
use BlockType::*;
use Direction::*;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    R,
    D,
    L,
    U,
}
impl Direction {
    fn left(&self) -> Self {
        match self {
            R => U,
            D => R,
            L => D,
            U => L,
        }
    }
    fn right(&self) -> Self {
        match self {
            R => D,
            D => L,
            L => U,
            U => R,
        }
    }
    // returns row and column
    fn delta(&self) -> (i32, i32) {
        match self {
            R => (0, 1),
            D => (1, 0),
            L => (0, -1),
            U => (-1, 0),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Move(u32),
    TurnLeft,
    TurnRight,
}

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub enum BlockType {
    Empty,
    Open,
    Wall,
}
impl Display for BlockType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = match self {
            Empty => ' ',
            Open => '.',
            Wall => '#',
        };
        f.write_char(ch)
    }
}

pub type Map = DMatrix<BlockType>;
pub type Pos = (usize, usize);


#[cfg(test)]
mod tests {
    use indoc::indoc;

    pub const TEST_INPUT: &str = indoc! {"
            ...#
            .#..
            #...
            ....
        ...#.......#
        ........#...
        ..#....#....
        ..........#.
            ...#....
            .....#..
            .#......
            ......#.

        10R5L5R10L4R5L5
    "};
}