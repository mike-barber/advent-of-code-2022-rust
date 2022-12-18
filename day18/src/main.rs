use nalgebra::Vector3;
use ndarray::prelude::*;
use ndarray::Array1;
use ndarray::Shape;

type Pos = Array1<i32>;

fn parse_input2(input: &str) -> Vec<Pos> {
    input
        .lines()
        .map(|l| {
            let a = l.split(',').map(str::parse::<i32>).flatten();
            Array1::from_iter(a)
        })
        .collect()
}

fn max_dims(vals: &[Pos]) -> Option<Pos> {
    let mut iter = vals.iter();

    let first = iter.next()?;
    let acc = first.clone();
    let res = iter.fold(acc, |mut acc, v| {
        acc.zip_mut_with(v, |a, b| {
            let av = *a;
            let bv = *b;
            *a = av.max(bv);
        });
        acc
    });

    Some(res)
}

fn to_addr(pos: &Pos) -> Ix3 {
    Dim([pos[0] as usize, pos[1] as usize, pos[2] as usize])
}

fn count_neighbours(space: &Array3<i32>, pos: &Pos) {

}

fn part1(points: &[Pos]) -> Option<usize> {
    // create space matrix
    let extents = max_dims(points)?;
    let shape = [
        extents[0] as usize,
        extents[1] as usize,
        extents[2] as usize,
    ];
    let mut space: Array3<i32> = Array3::zeros(shape);

    // place all the points
    for p in points.iter() {
        let ix = to_addr(p);
        space[ix] = 1;
    }




    todo!();
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        2,2,2
        1,2,2
        3,2,2
        2,1,2
        2,3,2
        2,2,1
        2,2,3
        2,2,4
        2,2,6
        1,2,5
        3,2,5
        2,1,5
        2,3,5
    "};

    #[test]
    fn parse_input_correct2() {
        let input = parse_input2(TEST_INPUT);
        for i in &input {
            println!("{i}");
        }
        assert_eq!(input.len(), 13);
    }
}
