use std::ops::Add;

use common::*;
use itertools::Itertools;
use ndarray::prelude::*;
use ndarray::Array1;
use priority_queue::PriorityQueue;

type Pos = Array1<i32>;

const NEIGHBOUR_OFFSETS: [[isize; 3]; 6] = [
    [-1, 0, 0],
    [1, 0, 0],
    [0, -1, 0],
    [0, 1, 0],
    [0, 0, -1],
    [0, 0, 1],
];
const DIST_NOT_FOUND: i32 = i32::MAX / 2;

fn parse_input(input: &str) -> Vec<Pos> {
    input
        .lines()
        .map(|l| {
            let a = l.split(',').flat_map(str::parse::<i32>);
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

fn space_shape(space: &Array3<i32>) -> Ix3 {
    let shape = space.shape();
    Dim([shape[0], shape[1], shape[2]])
}

fn to_addr(pos: &Pos) -> Ix3 {
    Dim([pos[0] as usize, pos[1] as usize, pos[2] as usize])
}

fn add_address_checked(addr: Ix3, offset: [isize; 3], shape: Ix3) -> Option<Ix3> {
    fn add_signed(x: usize, y: isize) -> Option<usize> {
        let v = x as isize + y;
        if v < 0 {
            return None;
        }
        Some(v as usize)
    }

    let new_addr = [
        add_signed(addr[0], offset[0])?,
        add_signed(addr[1], offset[1])?,
        add_signed(addr[2], offset[2])?,
    ];

    if new_addr[0] >= shape[0] || new_addr[1] >= shape[1] || new_addr[2] >= shape[2] {
        return None;
    }

    Some(Dim(new_addr))
}

fn count_neighbours(space: &Array3<i32>, pos: &Pos) -> usize {
    let shape = space_shape(space);
    let mut neighbours = 0;
    for offset in &NEIGHBOUR_OFFSETS {
        if let Some(addr) = add_address_checked(to_addr(pos), *offset, shape) {
            if space[addr] == 1 {
                neighbours += 1;
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

/// Fill reachable space, then consider which points have faces onto the filled
/// region
fn part2(points: &[Pos]) -> Option<usize> {
    // create space matrix
    let extents = max_dims(points)?;

    // +1 for index; +2 for space around all the edges for filling
    let shape = [
        extents[0] as usize + 3,
        extents[1] as usize + 3,
        extents[2] as usize + 3,
    ];
    let mut space: Array3<i32> = Array3::zeros(shape);

    // move all the points so they're away from the edges
    let offset = array![1, 1, 1];
    let points = points.iter().map(|p| p.add(&offset)).collect_vec();

    // place all the points
    for p in points.iter() {
        let ix = to_addr(p);
        space[ix] = 1;
    }

    // fill reachable space
    let exterior_reachable = fill_reachable_space(&space);

    // find all open surfaces
    let mut surface_area = 0;
    for p in points.iter() {
        let open_faces = count_open_faces_to_filled(&space, &exterior_reachable, p);
        surface_area += open_faces;
    }

    Some(surface_area)
}

// essentially Dijkstra again
fn fill_reachable_space(space: &Array3<i32>) -> Array3<i32> {
    let shape = space_shape(space);
    let mut dist: Array3<i32> = Array3::zeros(shape);

    // initialise problem
    let mut q = PriorityQueue::new();
    for x in 0..shape[0] {
        for y in 0..shape[1] {
            for z in 0..shape[2] {
                let ix = Dim([x, y, z]);
                dist[ix] = DIST_NOT_FOUND;
                // queue priority is highest first
                q.push(ix, i32::MIN);
            }
        }
    }
    let start = Dim([0, 0, 0]);
    dist[start] = 0;
    q.change_priority(&start, 0);

    // update all the reachable nodes
    while let Some((u, _)) = q.pop() {
        for offset in &NEIGHBOUR_OFFSETS {
            // consider valid neighbours still in the queue
            let v = add_address_checked(u, *offset, shape);
            if v.is_none() {
                continue;
            }
            let v = v.unwrap();

            // skip nodes that are part of the lava lump
            if space[v] == 1 {
                continue;
            }

            // record distance
            if q.get(&v).is_some() {
                // distance is to current node (u) + 1
                let alt = dist[u] + 1;
                if alt < dist[v] {
                    // update distances to this node
                    dist[v] = alt;
                    q.change_priority(&v, -alt);
                }
            }
        }
    }

    // // print filled region
    // for z in 0..shape[2] {
    //     let plane = dist.slice(s![.., .., z]);
    //     println!("z = {z}");
    //     println!("{plane}");
    // }

    dist
}

fn count_open_faces_to_filled(
    space: &Array3<i32>,
    exterior_reachable: &Array3<i32>,
    pos: &Pos,
) -> usize {
    let shape = space_shape(space);

    let mut empty_faces = 0;
    for offset in NEIGHBOUR_OFFSETS {
        if let Some(addr) = add_address_checked(to_addr(pos), offset, shape) {
            if space[addr] == 0 && exterior_reachable[addr] < DIST_NOT_FOUND {
                empty_faces += 1;
            }
        }
    }
    empty_faces
}

fn main() -> anyhow::Result<()> {
    let points = parse_input(&read_file("day18/input.txt")?);

    println!("part1 result = {:?}", part1(&points));
    println!("part2 result = {:?}", part2(&points));

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

    #[test]
    fn part2_correct() {
        let input = parse_input(TEST_INPUT);
        let res = part2(&input).unwrap();
        assert_eq!(res, 58);
    }
}
