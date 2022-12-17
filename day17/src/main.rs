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
