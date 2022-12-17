use anyhow::anyhow;
use common::*;
use indoc::indoc;
use nalgebra::{
    dmatrix, Const, DMatrix, DVector, Dynamic, OMatrix, RowVector, RowVector3, SMatrix, U7,
};
use std::{
    any::Any,
    fmt::{Display, Write},
};

use lazy_static::lazy_static;

const COLUMNS: usize = 7;
type ConstCols = Const<COLUMNS>;
type ProblemMatrix = OMatrix<i32, Dynamic, ConstCols>;
type RockMatrix = DMatrix<i32>;

lazy_static! {
    static ref ROCKS: [RockMatrix; 5] = [
        dmatrix![
            1,1,1,1;
        ],
        dmatrix![
            0,1,0;
            1,1,1;
            0,1,0;
        ],
        dmatrix![
            0,0,1;
            0,0,1;
            1,1,1;
        ],
        dmatrix![
            1;
            1;
            1;
            1;
        ],
        dmatrix![
            1,1;
            1,1;
        ]
    ];
}

pub const TEST_INPUT: &str = indoc! {">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"};

#[derive(Debug, Copy, Clone, PartialEq)]
enum Jet {
    L,
    R,
}

type JetPattern = Vec<Jet>;

fn parse_input(input: &str) -> AnyResult<JetPattern> {
    input
        .chars()
        .map(|c| match c {
            '<' => Ok(Jet::L),
            '>' => Ok(Jet::R),
            _ => Err(anyhow!("unrecognised character")),
        })
        .collect()
}

struct Problem {
    matrix: ProblemMatrix,
    highest_occupied_row: usize,
}
impl Problem {
    fn new(initial_rows: usize) -> Self {
        let matrix = ProblemMatrix::zeros(initial_rows);
        let highest_occupied_row = initial_rows;
        Problem {
            matrix,
            highest_occupied_row,
        }
    }

    fn double_height(mut self) -> Self {
        let current_rows = self.matrix.nrows();
        self.matrix = self.matrix.insert_rows(0, current_rows, 0);
        self.highest_occupied_row += current_rows;
        self
    }

    fn initial_position(&self, rock: &RockMatrix) -> Option<(usize, usize)> {
        let rock_height = rock.nrows();
        let highest = self.highest_occupied_row;
        match highest.checked_sub(rock_height + 3) {
            Some(row) => Some((row, 2)),
            None => None,
        }
    }

    fn try_move(
        &self,
        pos: (usize, usize),
        rows: isize,
        cols: isize,
        rock_buffer: &mut RockMatrix,
        rock: &RockMatrix,
    ) -> Option<(usize, usize)> {
        let mut r = pos.0 as isize;
        let mut c = pos.1 as isize;

        r += rows;
        c += cols;

        if r < 0 || c < 0 {
            return None;
        }

        if r as usize + rock.nrows() > self.matrix.nrows() {
            return None;
        }

        if c as usize + rock.ncols() > self.matrix.ncols() {
            return None;
        }

        let r = r as usize;
        let c = c as usize;

        // check collision by adding rock to the correct slice in the matrix
        rock_buffer.copy_from(rock);
        let rock_dims = (rock.nrows(), rock.ncols());
        let sub_matrix = self.matrix.slice((r, c), rock_dims);
        *rock_buffer += sub_matrix;
        if rock_buffer.iter().any(|&x| x > 1) {
            return None;
        }

        // acceptable move
        Some((r as usize, c as usize))
    }

    fn drop_rock(mut self, rock: &RockMatrix, jet_pattern: &mut impl Iterator<Item = Jet>) -> Self {
        let rock_dims = (rock.nrows(), rock.ncols());

        // create space for new rock and get the intial position
        let initial = loop {
            if let Some(pos) = self.initial_position(rock) {
                break pos;
            } else {
                self = self.double_height();
            }
        };

        // reusable buffer for rock collision tests
        let mut rock_buffer = rock.clone();

        // find lowest location to place rock without a conflict
        let mut current_loc = initial;
        let final_loc = loop {
            // respond to jet on current row
            let jet = jet_pattern.next().unwrap();
            let col_delta = match jet {
                Jet::L => -1,
                Jet::R => 1,
            };

            if let Some(loc) = self.try_move(current_loc, 0, col_delta, &mut rock_buffer, rock) {
                //println!("moved x by {jet:?}");
                current_loc = loc;
            }

            // move down 1
            if let Some(loc) = self.try_move(current_loc, 1, 0, &mut rock_buffer, rock) {
                //println!("moved down 1");
                current_loc = loc;
            } else {
                // stop - no further move possible
                //println!("conflict found at {current_loc:?}");
                break current_loc;
            }
        };

        // place rock
        let mut sub_matrix = self.matrix.slice_mut(final_loc, rock_dims);
        sub_matrix += rock;
        self.highest_occupied_row = self.highest_occupied_row.min(current_loc.0);

        self
    }

    fn tower_height(&self) -> usize {
        self.matrix.nrows() - self.highest_occupied_row
    }
}
impl Display for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..self.matrix.nrows() {
            for c in 0..self.matrix.ncols() {
                let ch = match self.matrix.get((r, c)).unwrap() {
                    0 => '.',
                    1 => '#',
                    _ => '?',
                };
                f.write_char(ch)?;
            }
            if r == self.highest_occupied_row {
                write!(f, " *")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() -> AnyResult<()> {
    demo();

    let jet_pattern = parse_input(&read_file("day17/input.txt")?)?;
    println!("part 1 result = {}", part1(&jet_pattern));

    Ok(())
}

fn demo() {
    let jet_pattern = parse_input(TEST_INPUT).unwrap();
    let mut problem = Problem::new(8);
    let mut jets_iter = jet_pattern.iter().cycle().copied();
    for rock in ROCKS.iter().cycle().take(10) {
        println!("---------------------------");
        println!("{rock}");
        problem = problem.drop_rock(rock, &mut jets_iter);
        println!("{problem}");
    }
}

fn part1(jet_pattern: &[Jet]) -> usize {
    let mut jets_iter = jet_pattern.iter().cycle().copied();
    let mut problem = Problem::new(8);
    for rock in ROCKS.iter().cycle().take(2022) {
        problem = problem.drop_rock(rock, &mut jets_iter);
    }
    problem.tower_height()
}

fn scratch() {
    let rock: SMatrix<i32, 3, 3> = SMatrix::from_rows(&[
        RowVector3::new(0, 1, 0),
        RowVector3::new(1, 1, 1),
        RowVector3::new(0, 1, 0),
    ]);

    let rock2 = dmatrix![
        0,1,0;
        1,1,1;
        0,1,0;
    ];

    println!("rock: {rock}");
    println!("rock: {rock2}");

    for rock in ROCKS.iter() {
        println!("{rock}");
    }

    {
        let mut problem_space: DMatrix<i32> = DMatrix::zeros(8, 7);
        println!("{}", problem_space);
        let mut sub_matrix = problem_space.slice_mut((2, 2), (3, 3));
        let rock = &ROCKS[1];
        sub_matrix.copy_from(rock);
        sub_matrix += rock;
        println!("{}", problem_space);
        let mut problem_space = problem_space.insert_rows(0, 8, 0);
        println!("{}", problem_space);
        println!("{}x{}", problem_space.nrows(), problem_space.ncols());
    }

    {
        let mut problem_space: OMatrix<i32, Dynamic, U7> = OMatrix::<i32, Dynamic, U7>::zeros(8);
        let mut sub_matrix = problem_space.slice_mut((2, 2), (3, 3));
        let rock = &ROCKS[1];
        sub_matrix.copy_from(rock);
        sub_matrix += rock;
        println!("{}", problem_space);
        let mut problem_space = problem_space.insert_rows(0, 8, 0);
        println!("{}", problem_space);
        println!("{}x{}", problem_space.nrows(), problem_space.ncols());
    }
}

//#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input_correct() {
        let pattern = parse_input(TEST_INPUT).unwrap();
        println!("{:?}", pattern);
    }

    #[test]
    fn part1_correct() {
        let pattern = parse_input(TEST_INPUT).unwrap();
        let res = part1(&pattern);
        assert_eq!(res, 3068);
    }
}
