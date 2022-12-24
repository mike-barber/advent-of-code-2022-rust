use std::fmt::Debug;

use itertools::Itertools;
use priority_queue::PriorityQueue;
use rustc_hash::FxHashMap;

use crate::*;

/// `Regime` specifies where we are in the overall journey
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Regime {
    /// Initial outbound journey from start to end
    Initial,
    /// Return from end back to start to pick up the snacks
    ReturnToStart,
    /// Second and final journey from start to end
    Final,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct PosStateRegime {
    regime: Regime,
    phase: usize,
    loc: Point,
}
impl PosStateRegime {
    fn new(regime: Regime, phase: usize, loc: Point) -> Self {
        PosStateRegime { regime, phase, loc }
    }
}

pub fn find_shortest_path(problem: &Problem) -> Option<i32> {
    const DIST_INIT: i32 = i32::MAX / 2;
    const PRIO_INIT: i32 = i32::MIN;

    let states = (0..problem.cycle_length)
        .map(|t| ProblemState::with_time(problem, t))
        .collect_vec();

    let mut dist: FxHashMap<PosStateRegime, i32> = FxHashMap::default();

    // initialise
    let start = PosStateRegime::new(Regime::Initial, 0, problem.start);
    let mut queue = PriorityQueue::new();
    queue.push(start, 0);
    dist.insert(start, 0);

    while let Some((u, _prio)) = queue.pop() {
        let next_phase = problem.next_phase(u.phase);
        let next_state = &states[next_phase];

        let valid_moves = next_state.available_moves(u.loc);
        for v_point in valid_moves {
            // transition points
            let next_regime = match (u.regime, v_point) {
                (Regime::Initial, p) if p == problem.end => Regime::ReturnToStart,
                (Regime::ReturnToStart, p) if p == problem.start => Regime::Final,
                (r, _) => r,
            };
            let v = PosStateRegime::new(next_regime, next_phase, v_point);

            if !dist.contains_key(&v) {
                queue.push(v, PRIO_INIT);
                dist.insert(v, DIST_INIT);
            }

            if queue.get(&v).is_some() {
                // distance is to current node (u) + 1
                let alt = dist.get(&u).unwrap() + 1;
                if alt < *dist.get(&v).unwrap() {
                    // update distances to this node
                    *dist.get_mut(&v).unwrap() = alt;
                    queue.change_priority(&v, -alt);
                }
            }
        }
    }

    // print out all states
    // for dest_state in (0..problem.cycle_length)
    //     .map(|phase| PosStateRegime::new(Regime::Final, phase, problem.end))
    // {
    //     println!("state {dest_state:?} => {:?}", dist.get(&dest_state));
    // }

    // return shortest - from the `Final` regime, and we're looking for the `end` point
    // in any phase.
    let minimum = (0..problem.cycle_length)
        .flat_map(|phase| dist.get(&PosStateRegime::new(Regime::Final, phase, problem.end)))
        .min();
    minimum.copied()
}
