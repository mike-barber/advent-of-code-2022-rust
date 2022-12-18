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

fn add_address_checked(addr: Ix3, offset: [isize; 3], shape: [usize; 3]) -> Option<Ix3> {
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

fn count_open_faces_part2_simple(space: &Array3<i32>, pos: &Pos) -> usize {
    let mut empty_faces = 0;
    for dim in 0..3 {
        for offset in [-1, 1] {
            let mut test_pos = pos.clone();
            test_pos[dim] += offset;

            if let Some(addr) = to_addr_checked(&test_pos, space.shape()) {
                // if the cell is empty, check that it has at least one open
                // space (i.e. not completely enclosed)
                if space[addr] == 0 {
                    let space_neighbours = count_neighbours(space, &test_pos);
                    if space_neighbours < 6 {
                        empty_faces += 1;
                    }
                }
            } else {
                // also open space - edge of grid
                empty_faces += 1;
            }
        }
    }
    empty_faces
}

fn count_open_faces_part2_filled(
    space: &Array3<i32>,
    exterior_reachable: &Array3<i32>,
    pos: &Pos,
) -> usize {
    let shape = space.shape();
    let shape = [shape[0], shape[1], shape[2]];

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

// assuming air pockets are single, and not joined initially
fn part2_simple(points: &[Pos]) -> Option<usize> {
    // create space matrix
    let extents = max_dims(points)?;
    // create empty space around right and bottom edge for fill
    let shape = [
        extents[0] as usize + 2,
        extents[1] as usize + 2,
        extents[2] as usize + 2,
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
        let open_faces = count_open_faces_part2_simple(&space, p);
        surface_area += open_faces;
    }

    Some(surface_area)
}

// fill reachable space, then consider which points have faces to it
fn part2_filled(points: &[Pos]) -> Option<usize> {
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
        let open_faces = count_open_faces_part2_filled(&space, &exterior_reachable, p);
        surface_area += open_faces;
    }

    // print filled region
    for z in 0..shape[2] {
        let plane = exterior_reachable.slice(s![.., .., z]);
        println!("z = {z}");
        println!("{plane}");
    }

    Some(surface_area)
}

// essentially Dijkstra again
fn fill_reachable_space(space: &Array3<i32>) -> Array3<i32> {
    let shape = space.shape();
    let shape = [shape[0], shape[1], shape[2]];
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

    dist
}

fn main() -> anyhow::Result<()> {
    let points = parse_input(&read_file("day18/input.txt")?);

    println!("part1 result = {:?}", part1(&points));
    println!("part2 simple result = {:?}", part2_simple(&points));
    println!("part2 filled result = {:?}", part2_filled(&points));
    println!("note: 3402 is wrong -- it's too high!");
    println!("note: 2063 is wrong -- it's too low!");

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
    fn part2_simple_correct() {
        let input = parse_input(TEST_INPUT);
        let res = part2_simple(&input).unwrap();
        assert_eq!(res, 58);
    }

    #[test]
    fn part2_filled_correct() {
        let input = parse_input(TEST_INPUT);
        let res = part2_filled(&input).unwrap();
        assert_eq!(res, 58);
    }
}
