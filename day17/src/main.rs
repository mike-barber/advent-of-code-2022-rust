use anyhow::anyhow;
use common::*;
use nalgebra::{dmatrix, DMatrix, DVector, Dynamic, OMatrix, RowVector, RowVector3, SMatrix};

use lazy_static::lazy_static;

lazy_static! {
    static ref ROCKS: [DMatrix<i32>; 5] = [
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

fn main() {
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
}


#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const TEST_INPUT: &str = indoc! {">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"};

    #[test]
    fn parse_input_correct() {
        let pattern = parse_input(TEST_INPUT).unwrap();
        println!("{:?}", pattern);
    }


}