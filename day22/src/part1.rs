use crate::{BlockType, Direction, Instruction, Map, Pos};
use anyhow::bail;
use common::*;
use nalgebra::DMatrix;
use regex::Regex;

use BlockType::*;
use Direction::*;

#[derive(Debug, Clone)]
pub struct Problem {
    map: Map,
    instructions: Vec<Instruction>,
}
impl Problem {
    pub fn find_next(&self, pos: Pos, dir: Direction) -> (BlockType, Pos) {
        let (dr, dc) = dir.delta();
        let mut r = pos.0 as i32;
        let mut c = pos.1 as i32;
        loop {
            r = (r + dr).rem_euclid(self.map.nrows() as i32);
            c = (c + dc).rem_euclid(self.map.ncols() as i32);

            let new_pos = (r as usize, c as usize);
            let block = self.map[new_pos];
            if block != Empty {
                return (block, new_pos);
            }
        }
    }
}

pub fn parse_input(input: &str) -> AnyResult<Problem> {
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

pub fn run(problem: &Problem) -> usize {
    let mut dir = Direction::R;
    let col = problem.map.row(0).iter().position(|b| *b == Open).unwrap();
    let mut pos = (0, col);

    for inst in problem.instructions.iter() {
        match inst {
            Instruction::Move(n) => {
                for _ in 0..*n {
                    let (block, new_pos) = problem.find_next(pos, dir);
                    if block == BlockType::Wall {
                        break;
                    } else {
                        pos = new_pos;
                    }
                }
            }
            Instruction::TurnLeft => dir = dir.left(),
            Instruction::TurnRight => dir = dir.right(),
        }
    }

    println!("final position {pos:?} and direction {dir:?}");
    let score = 1000 * (pos.0 + 1)
        + 4 * (pos.1 + 1)
        + match dir {
            // Facing is 0 for right (>), 1 for down (v), 2 for left (<), and 3 for up (^)
            R => 0,
            D => 1,
            L => 2,
            U => 3,
        };
    score
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

    #[test]
    fn part1_correct() {
        let problem = parse_input(TEST_INPUT).unwrap();
        let res = run(&problem);
        assert_eq!(res, 6032);
    }
}
