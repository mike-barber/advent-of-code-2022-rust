use std::{collections::HashMap, fmt::Display};

use crate::{BlockType, Direction, Instruction, Map};
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

/// Definition of connections between edges, with `invert_coord` specifying
/// if the direction along the edge is inverted as we cross it. This was probably
/// a suboptimal design decision: it probably would have been better to specify
/// the vertices (A..H) for each square instead of the edges, since that would
/// encode the direction too.
#[derive(Debug, Clone, Copy)]
pub struct Connection {
    a: Edge,
    b: Edge,
    invert_coord: bool,
}

impl Connection {
    // create new connection
    pub fn new(a: Edge, b: Edge, invert_coord: bool) -> Self {
        Connection { a, b, invert_coord }
    }
}

#[derive(Debug, Clone)]
pub struct Topology(HashMap<Edge, (Edge, bool)>);
impl Topology {
    pub fn new(connections: &[Connection]) -> AnyResult<Self> {
        // check for invalid edges - we're expecting ONE edge listed
        let mut map = HashMap::new();
        for con in connections {
            if map.insert(con.a, (con.b, con.invert_coord)).is_some() {
                bail!("attempted to insert duplicate edge {}", con.a)
            }
            if map.insert(con.b, (con.a, con.invert_coord)).is_some() {
                bail!("attempted to insert duplicate edge {}", con.b)
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub face: usize,
    pub r: usize,
    pub c: usize,
    pub dir: Direction,
}
impl Position {
    fn turn_left(&self) -> Self {
        Position {
            dir: self.dir.left(),
            ..*self
        }
    }
    fn turn_right(&self) -> Self {
        Position {
            dir: self.dir.right(),
            ..*self
        }
    }
}

#[derive(Debug, Clone)]
pub struct Problem {
    pub edge_len: usize,
    pub faces: Vec<Map>,
    pub faces_top_left: Vec<(usize, usize)>,
    pub topology: Topology,
    pub instructions: Vec<Instruction>,
}
impl Problem {
    pub fn next_position(&self, pos: Position) -> Position {
        let (dr, dc) = pos.dir.delta();
        let row = pos.r as i32 + dr;
        let col = pos.c as i32 + dc;

        let dim = 0..self.edge_len as i32;

        if dim.contains(&row) && dim.contains(&col) {
            // stay on this face
            Position {
                r: row as usize,
                c: col as usize,
                ..pos
            }
        } else {
            // transition to next face edge (note that the faces are labelled 1-indexed)
            let (next_edge, invert) = self
                .topology
                .0
                .get(&Edge::new(pos.face + 1, pos.dir))
                .unwrap();
            let next_face = next_edge.0 - 1;
            let next_dir = next_edge.1.opposite();

            // this is our location on the existing edge, inverted if required
            let loc_on_existing_edge = match pos.dir {
                L | R => row,
                U | D => col,
            };

            let loc_on_existing_edge = match invert {
                false => loc_on_existing_edge,
                true => self.edge_len as i32 - 1 - loc_on_existing_edge,
            };

            // this is our translated new position on the next edge
            let (r, c) = match next_edge.1 {
                L => (loc_on_existing_edge, 0),
                R => (loc_on_existing_edge, self.edge_len as i32 - 1),
                U => (0, loc_on_existing_edge),
                D => (self.edge_len as i32 - 1, loc_on_existing_edge),
            };

            Position {
                face: next_face,
                r: r as usize,
                c: c as usize,
                dir: next_dir,
            }
        }
    }

    fn block_type(&self, pos: Position) -> BlockType {
        let face = &self.faces[pos.face];
        face[(pos.r, pos.c)]
    }

    pub fn run(&self) -> usize {
        // initial position
        let mut pos = Position {
            face: 0,
            r: 0,
            c: 0,
            dir: R,
        };

        for inst in self.instructions.iter() {
            match inst {
                Instruction::Move(n) => {
                    for _ in 0..*n {
                        let new_pos = self.next_position(pos);
                        match self.block_type(new_pos) {
                            BlockType::Wall => break,
                            BlockType::Open => pos = new_pos,
                            BlockType::Empty => panic!("unexpected block type for part2"),
                        }
                    }
                }
                Instruction::TurnLeft => pos = pos.turn_left(),
                Instruction::TurnRight => pos = pos.turn_right(),
            }
        }

        // get the origin row and column for this face
        let face_num = pos.face;
        let (face_origin_r, face_origin_c) = self.faces_top_left[face_num];
        println!("face origin: {face_origin_r}, {face_origin_c}");

        let score = 1000 * (pos.r + face_origin_r + 1)
            + 4 * (pos.c + face_origin_c + 1)
            + match pos.dir {
                // Facing is 0 for right (>), 1 for down (v), 2 for left (<), and 3 for up (^)
                R => 0,
                D => 1,
                L => 2,
                U => 3,
            };
        println!("score: {score}");
        score
    }
}

pub fn parse_block(
    lines: &[&str],
    start_idx: usize,
    edge_len: usize,
) -> Option<DMatrix<BlockType>> {
    let mut map = DMatrix::repeat(edge_len, edge_len, BlockType::Empty);
    for row in 0..edge_len {
        let line = lines[row];
        for (col, ch) in line.chars().skip(start_idx).take(edge_len).enumerate() {
            let block_type = match ch {
                ' ' => Empty,
                '.' => Open,
                '#' => Wall,
                _ => panic!("invalid map character: {}", ch),
            };
            map[(row, col)] = block_type;
        }
    }

    if map.iter().any(|v| *v == BlockType::Empty) {
        return None;
    }

    Some(map)
}

pub fn parse_input(input: &str, edge_len: usize, topology: Topology) -> AnyResult<Problem> {
    let lines: Vec<_> = input.lines().collect();

    let mut sections = lines.split(|l| l.is_empty());

    let map_input = sections.next().ok_anyhow()?;
    let inst_input = sections.next().ok_anyhow()?;

    let total_rows = map_input.len();
    let total_cols = map_input
        .iter()
        .map(|l| l.chars().count())
        .max()
        .ok_anyhow()?;

    let block_rows = total_rows / edge_len;
    let block_cols = total_cols / edge_len;

    let mut faces: Vec<DMatrix<BlockType>> = vec![];
    let mut faces_top_left = vec![];
    for br in 0..block_rows {
        let lines = &map_input[edge_len * br..edge_len * (br + 1)];
        for bc in 0..block_cols {
            if let Some(face) = parse_block(lines, edge_len * bc, edge_len) {
                faces.push(face);
                faces_top_left.push((edge_len * br, edge_len * bc));
            }
        }
    }

    if faces.len() != 6 {
        bail!("expected 6 faces, and have {}", faces.len());
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

    Ok(Problem {
        edge_len,
        faces,
        faces_top_left,
        topology,
        instructions,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::TEST_INPUT;

    fn create_test_topology() -> Topology {
        let connections = [
            Connection::new(Edge::new(4, U), Edge::new(1, D), false),
            Connection::new(Edge::new(4, D), Edge::new(5, U), false),
            Connection::new(Edge::new(4, L), Edge::new(3, R), false),
            Connection::new(Edge::new(4, R), Edge::new(6, U), false),
            Connection::new(Edge::new(3, U), Edge::new(1, L), false),
            Connection::new(Edge::new(3, D), Edge::new(5, L), false),
            Connection::new(Edge::new(3, L), Edge::new(2, R), false),
            Connection::new(Edge::new(2, U), Edge::new(1, U), false),
            Connection::new(Edge::new(2, D), Edge::new(5, D), false),
            Connection::new(Edge::new(2, L), Edge::new(6, D), false),
            Connection::new(Edge::new(1, R), Edge::new(6, R), false),
            Connection::new(Edge::new(5, R), Edge::new(6, L), false),
        ];
        Topology::new(&connections).unwrap()
    }

    #[test]
    fn create_topology() {
        create_test_topology();
    }

    #[test]
    fn parse_input_correct() {
        let problem = parse_input(TEST_INPUT, 4, create_test_topology()).unwrap();
        problem.faces.iter().enumerate().for_each(|(i, f)| {
            println!("{i}");
            println!("{f}");
        });
    }

    #[test]
    fn part2_correct() {
        let problem = parse_input(TEST_INPUT, 4, create_test_topology()).unwrap();
        let res = problem.run();
        assert_eq!(res, 5031);
    }
}
