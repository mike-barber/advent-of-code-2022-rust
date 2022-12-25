use anyhow::anyhow;
use common::*;
use indoc::indoc;
use itertools::Itertools;
use nalgebra::{dmatrix, Const, DMatrix, Dynamic, OMatrix};
use std::fmt::{Display, Write};

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

type JetPattern = Vec<JetIndex>;
type JetIndex = (usize, Jet);

fn parse_input(input: &str) -> AnyResult<JetPattern> {
    input
        .char_indices()
        .map(|(i, c)| match c {
            '<' => Ok((i, Jet::L)),
            '>' => Ok((i, Jet::R)),
            _ => Err(anyhow!("unrecognised character")),
        })
        .collect()
}

struct Problem {
    matrix: ProblemMatrix,
    highest_occupied_row: usize,
    current_jet_index: usize,
}
impl Problem {
    fn new(initial_rows: usize) -> Self {
        let matrix = ProblemMatrix::zeros(initial_rows);
        let highest_occupied_row = initial_rows;
        Problem {
            matrix,
            highest_occupied_row,
            current_jet_index: 0,
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
        highest.checked_sub(rock_height + 3).map(|row| (row, 2))
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

    fn drop_rock(
        mut self,
        rock: &RockMatrix,
        jet_pattern: &mut impl Iterator<Item = JetIndex>,
    ) -> Self {
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
        let (jet_index, final_loc) = loop {
            // respond to jet on current row
            let (jet_index, jet) = jet_pattern.next().unwrap();
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
                break (jet_index, current_loc);
            }
        };

        // place rock
        let mut sub_matrix = self.matrix.slice_mut(final_loc, rock_dims);
        sub_matrix += rock;
        self.highest_occupied_row = self.highest_occupied_row.min(current_loc.0);
        self.current_jet_index = jet_index;

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
    let jet_pattern = parse_input(&read_file("day17/input.txt")?)?;
    println!("pattern length: {}", jet_pattern.len());

    println!("part 1 result = {}", part1(&jet_pattern));
    println!("part 2 result = {}", part2(&jet_pattern));

    Ok(())
}

#[allow(dead_code)]
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

fn part1(jet_pattern: &[JetIndex]) -> usize {
    let mut jets_iter = jet_pattern.iter().cycle().copied();
    let mut problem = Problem::new(8);
    for rock in ROCKS.iter().cycle().take(2022) {
        problem = problem.drop_rock(rock, &mut jets_iter);
    }
    problem.tower_height()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct State {
    rock_mod: usize,
    jet_mod: usize,
    top_row_byte: u8,
}

#[derive(Debug, Clone)]
struct Cycle {
    at_rock: usize,
    num_rocks: usize,
    height_gain: usize,
}

#[derive(Debug, Clone, Default)]
struct History {
    states: Vec<State>,
    heights: Vec<usize>,
}
impl History {
    fn push(&mut self, state: State, height: usize) {
        self.states.push(state);
        self.heights.push(height);
    }

    fn len(&self) -> usize {
        self.states.len()
    }

    // find the second position, skipping first
    fn positions_earlier_state(&self) -> impl Iterator<Item = usize> + '_ {
        let state = self.states.last();
        let positions = self
            .states
            .iter()
            .positions(move |a| Some(a) == state)
            .rev()
            .skip(1);
        positions
    }

    fn find_cycle(&self) -> Option<Cycle> {
        let current_pos = self.len() - 1;
        for earlier_pos in self.positions_earlier_state() {
            let length = current_pos - earlier_pos;

            // exit condition -- insufficient history for this length
            // of cycle
            if self.len() < length + length {
                break;
            }

            let a = &self.states[self.len() - length..self.len()];
            let b = &self.states[self.len() - length - length..self.len() - length];
            if a == b {
                let height_delta =
                    self.heights[self.len() - 1] - self.heights[self.len() - 1 - length];
                return Some(Cycle {
                    at_rock: current_pos,
                    num_rocks: length,
                    height_gain: height_delta,
                });
            }
        }
        None
    }
}

fn part2(jet_pattern: &[JetIndex]) -> usize {
    const TARGET_ROCKS: usize = 1000000000000;

    let mut jets_iter = jet_pattern.iter().cycle().copied();
    let mut problem = Problem::new(8);
    let mut history = History::default();

    let mut found_cycle = None;
    for (rock_mod, rock) in ROCKS.iter().enumerate().cycle() {
        problem = problem.drop_rock(rock, &mut jets_iter);

        // record history
        let state = State {
            rock_mod,
            jet_mod: problem.current_jet_index,
            top_row_byte: row_as_byte(problem.highest_occupied_row, &problem.matrix),
        };
        let height = problem.tower_height();
        history.push(state, height);

        // detect cycle at final rock in the ROCKS array
        if rock_mod == ROCKS.len() - 1 {
            if let Some(cycle) = history.find_cycle() {
                found_cycle = Some(cycle);
                break;
            }
        }
    }

    let found_cycle = found_cycle.unwrap();
    println!("found cycle: {found_cycle:?}");

    // now we know that the pattern repeats ad-infinitum, so we can skip
    // all the intermediate steps and work out the final height.
    let rocks_remaining = TARGET_ROCKS - found_cycle.at_rock;
    let cycles = rocks_remaining / found_cycle.num_rocks;
    let remainder = rocks_remaining % found_cycle.num_rocks;

    // continue running the problem over the remaining rocks, adding height
    // from the remainder rocks
    for rock in ROCKS.iter().cycle().take(remainder) {
        problem = problem.drop_rock(rock, &mut jets_iter);
    }

    // last row in the cycle is the first row of the remainder, so
    // substract one for the overlap.
    let cycle_height = cycles * found_cycle.height_gain - 1;
    cycle_height + problem.tower_height()
}

fn row_as_byte(r: usize, matrix: &ProblemMatrix) -> u8 {
    let row = matrix.row(r);
    let mut byte = 0u8;
    for c in 0..row.len() {
        let x = row[c] as u8;
        byte |= x << c;
    }
    byte
}

#[cfg(test)]
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

    #[test]
    fn part2_correct() {
        let pattern = parse_input(TEST_INPUT).unwrap();
        let res = part2(&pattern);
        assert_eq!(res, 1514285714288);
    }
}
