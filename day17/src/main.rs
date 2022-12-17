use anyhow::anyhow;
use common::*;
use indoc::indoc;
use nalgebra::{
    dmatrix, Const, DMatrix, DVector, Dynamic, OMatrix, RowVector, RowVector3, SMatrix, U7,
};
use std::any::Any;

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
        match highest.checked_sub(rock_height) {
            Some(row) => Some((row, 2)),
            None => None,
        }
    }

    fn drop_rock(mut self, rock: &RockMatrix, jet_pattern: &JetPattern) -> Self {
        let rock_dims = (rock.nrows(), rock.ncols());

        // create space for new rock and get the intial position
        let initial = loop {
            if let Some(pos) = self.initial_position(rock) {
                break pos;
            } else {
                self = self.double_height();
            }
        };

        // find lowest location to place rock without a conflict
        let mut rock_sum = rock.clone();
        let mut current_loc = initial;
        for (row_offset, jet) in (0..).zip(jet_pattern.iter().cycle()) {
            
            // respond to jet on current row
            let col = match (jet, current_loc.1) {
                (Jet::L, c) if c > 0 => c-1,
                (Jet::R, c) if c < COLUMNS - rock_dims.1 => c + 1,
                (_,c) => c,
            };

            // move down 1
            let loc = (initial.0 + row_offset, col);

            let bottom_row = loc.0 + rock_dims.0;
            if bottom_row >= self.matrix.nrows() {
                println!("hit bottom with loc: {:?}", loc);
                break;
            }

            let sub_matrix = self.matrix.slice_mut(loc, rock_dims);
            rock_sum.copy_from(rock);
            rock_sum += sub_matrix;

            let conflict = rock_sum.iter().any(|v| *v > 1);
            if conflict {
                println!("conflict found at row {row_offset}");
                println!("{}", rock_sum);
                break;
            }

            // update placement location from prior
            current_loc = loc;
        }

        // place rock
        {
            let mut sub_matrix = self.matrix.slice_mut(current_loc, rock_dims);
            sub_matrix += rock;
            self.highest_occupied_row = current_loc.0;
        }
        println!("placed rock: {}", self.matrix);

        self
    }
}

fn main() -> AnyResult<()> {
    let pattern = parse_input(TEST_INPUT)?;
    let mut problem = Problem::new(8);

    for r in ROCKS.iter() {
        problem = problem.drop_rock(r, &pattern);
    }

    Ok(())
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
}
