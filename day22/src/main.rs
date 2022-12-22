use std::fmt::{Display, Write};

use anyhow::bail;
use common::{AnyResult, OptionAnyhow};
use nalgebra::DMatrix;
use regex::Regex;
use BlockType::*;

#[derive(Debug, Clone)]
enum Instruction {
    Move(u32),
    TurnLeft,
    TurnRight,
}

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
enum BlockType {
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

type Map = DMatrix<BlockType>;

#[derive(Debug, Clone)]
struct Problem {
    map: Map,
    instructions: Vec<Instruction>,
}

fn parse_input(input: &str) -> AnyResult<Problem> {
    let lines: Vec<_> = input.lines().collect();

    let mut sections = lines.split(|l| l.is_empty());

    let map_input = sections.next().ok_anyhow()?;
    let inst_input = sections.next().ok_anyhow()?;

    let rows = map_input.len();
    let cols = map_input
        .iter()
        .map(|l| l.chars().count())
        .max()
        .ok_anyhow()?;

    let mut map = DMatrix::repeat(rows, cols, BlockType::Empty);
    for row in 0..rows {
        let line = map_input[row];
        for (col, ch) in line.chars().enumerate() {
            let block_type = match ch {
                ' ' => Empty,
                '.' => Open,
                '#' => Wall,
                _ => bail!("unexpected map character: {}", ch),
            };
            map[(row, col)] = block_type;
        }
    }

    let mut instructions = vec![];
    let re = Regex::new(r#"(\d+)|(R|L)"#)?;
    for c in re.captures_iter(inst_input.first().ok_anyhow()?) {
        if let Some(count) = c.get(1) {
            instructions.push(Instruction::Move(count.as_str().parse()?))
        }
        if let Some(turn) = c.get(2) {
            let dir = match turn.as_str() {
                "L" => Instruction::TurnLeft,
                "R" => Instruction::TurnRight,
                x => bail!("invalid instruction: {}", x),
            };
            instructions.push(dir);
        }
    }

    Ok(Problem { map, instructions })
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
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

    #[test]
    fn parse_input_correct() {
        let problem = parse_input(TEST_INPUT).unwrap();
        dbg!(&problem);
        println!("{}", problem.map);
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
