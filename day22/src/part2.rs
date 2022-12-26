use std::{
    cmp::Ordering,
    collections::HashMap,
    fmt::{write, Display},
};

use crate::{BlockType, Direction, Instruction, Map, Pos};
use anyhow::bail;
use common::*;
use nalgebra::DMatrix;
use regex::Regex;

use BlockType::*;
use Direction::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Edge(usize, Direction);
impl Edge {
    pub fn new(face: usize, direction: Direction) -> Self {
        Edge(face, direction)
    }
}
impl Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Connection(Edge, Edge);

impl Connection {
    // create new connection with canonical ordering
    pub fn new(a: Edge, b: Edge) -> Self {
        let (a, b) = match a.cmp(&b) {
            Ordering::Less => (a, b),
            Ordering::Equal => (a, b),
            Ordering::Greater => (b, a),
        };
        Connection(a, b)
    }

    // // check if edge is contained in the connection
    // pub fn contains(&self, a: Edge) -> bool {
    //     self.0 == a || self.1 == a
    // }

    // // get other edge for this connection, given one of
    // // the edges
    // pub fn connection_to(&self, near: Edge) -> Option<Edge> {
    //     if near == self.0 {
    //         Some(self.1)
    //     } else if near == self.1 {
    //         Some(self.0)
    //     } else {
    //         None
    //     }
    // }

    pub fn first(&self) -> Edge {
        self.0
    }

    pub fn second(&self) -> Edge {
        self.1
    }
}

pub struct Topology(HashMap<Edge, Edge>);
impl Topology {
    pub fn new(connections: &[Connection]) -> AnyResult<Self> {
        // check for invalid edges - we're expecting ONE edge listed
        let mut map = HashMap::new();
        for con in connections {
            //println!("map {map:?} + connection {con:?}");

            if let Some(_) = map.insert(con.first(), con.second()) {
                bail!("attempted to insert duplicate edge {}", con.first())
            }
            if let Some(_) = map.insert(con.second(), con.first()) {
                bail!("attempted to insert duplicate edge {}", con.second())
            }
        }

        // assert that we have 12 edges
        if map.len() != 24 {
            bail!(
                "expected 24 edges in connection map, and have {}",
                map.len()
            );
        }

        Ok(Self(map))
    }
}

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

pub fn parse_input(input: &str, topology: &Topology) -> AnyResult<Problem> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::TEST_INPUT;

    fn create_test_topology() -> Topology {
        let connections = [
            Connection::new(Edge::new(4, U), Edge::new(1, D)),
            Connection::new(Edge::new(4, D), Edge::new(5, U)),
            Connection::new(Edge::new(4, L), Edge::new(3, R)),
            Connection::new(Edge::new(4, R), Edge::new(6, U)),
            Connection::new(Edge::new(3, U), Edge::new(1, L)),
            Connection::new(Edge::new(3, D), Edge::new(5, L)),
            Connection::new(Edge::new(3, L), Edge::new(2, R)),
            Connection::new(Edge::new(2, U), Edge::new(1, U)),
            Connection::new(Edge::new(2, D), Edge::new(5, D)),
            Connection::new(Edge::new(2, L), Edge::new(6, D)),
            Connection::new(Edge::new(1, R), Edge::new(6, R)),
            Connection::new(Edge::new(5, R), Edge::new(6, L)),
        ];
        Topology::new(&connections).unwrap()
    }

    #[test]
    fn create_topology() {
        create_test_topology();
    }

    #[test]
    fn parse_input_correct() {
        let topology = create_test_topology();
        let problem = parse_input(TEST_INPUT, &topology).unwrap();
        dbg!(&problem);
        println!("{}", problem.map);
    }

    // #[test]
    // fn part2_correct() {
    //     let problem = parse_input(TEST_INPUT).unwrap();
    //     let res = run(&problem);
    //     assert_eq!(res, 6032);
    // }
}
