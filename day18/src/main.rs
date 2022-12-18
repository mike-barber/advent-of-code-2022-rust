use common::*;
use ndarray::prelude::*;
use ndarray::Array1;

type Pos = Array1<i32>;

fn parse_input(input: &str) -> Vec<Pos> {
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

fn to_addr_checked(pos: &Pos, shape: &[usize]) -> Option<Ix3> {
    if pos[0] < 0 || pos[1] < 0 || pos[2] < 0 {
        return None;
    }
    let pos_usize = [pos[0] as usize, pos[1] as usize, pos[2] as usize];
    for i in 0..3 {
        if pos_usize[i] >= shape[i] {
            return None;
        }
    }
    Some(Dim(pos_usize))
}

fn count_neighbours(space: &Array3<i32>, pos: &Pos) -> usize {
    let mut neighbours = 0;
    for dim in 0..3 {
        for ofs in [-1, 1] {
            let mut p = pos.clone();
            p[dim] += ofs;

            if let Some(addr) = to_addr_checked(&p, space.shape()) {
                if space[addr] == 1 {
                    neighbours += 1;
                }
            }
        }
    }
    neighbours
}

fn part1(points: &[Pos]) -> Option<usize> {
    // create space matrix
    let extents = max_dims(points)?;
    let shape = [
        extents[0] as usize + 1,
        extents[1] as usize + 1,
        extents[2] as usize + 1,
    ];
    let mut space: Array3<i32> = Array3::zeros(shape);

    // place all the points
    for p in points.iter() {
        let ix = to_addr(p);
        space[ix] = 1;
    }

    // find all open surfaces
    let mut surface_area = 0;
    for p in points.iter() {
        let neighbours = count_neighbours(&space, p);
        let open_faces = 6 - neighbours;
        surface_area += open_faces;
    }

    Some(surface_area)
}

fn main() -> anyhow::Result<()> {
    let points = parse_input(&read_file("day18/input.txt")?);

    println!("part1 result = {:?}", part1(&points));

    Ok(())
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
        let input = parse_input(TEST_INPUT);
        for i in &input {
            println!("{i}");
        }
        assert_eq!(input.len(), 13);
    }

    #[test]
    fn part1_correct() {
        let input = parse_input(TEST_INPUT);
        let res = part1(&input).unwrap();
        assert_eq!(res, 64);
    }
}
