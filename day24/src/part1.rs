use std::fmt::Debug;

use itertools::Itertools;
use priority_queue::PriorityQueue;
use rustc_hash::FxHashMap;

use crate::*;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct PosState {
    phase: usize,
    loc: Point,
}
impl PosState {
    fn new(phase: usize, loc: Point) -> Self {
        PosState { phase, loc }
    }
}

pub fn find_shortest_path(problem: &Problem) -> Option<i32> {
    const DIST_INIT: i32 = i32::MAX / 2;
    const PRIO_INIT: i32 = i32::MIN;

    let states = (0..problem.cycle_length)
        .map(|t| ProblemState::with_time(problem, t))
        .collect_vec();

    let mut dist: FxHashMap<PosState, i32> = FxHashMap::default();

    // initialise
    let start = PosState::new(0, problem.start);
    let mut queue = PriorityQueue::new();
    queue.push(start, 0);
    dist.insert(start, 0);

    while let Some((u, _prio)) = queue.pop() {
        let u_dist = dist.get(&u).copied().unwrap();

        let next_phase = problem.next_phase(u.phase);
        let next_state = &states[next_phase];

        let valid_moves = next_state.available_moves(u.loc);
        for v_point in valid_moves {
            let v = PosState::new(next_phase, v_point);

            // add point to dist map and queue if it's unseen
            let v_dist = dist.entry(v).or_insert_with(|| {
                queue.push(v, PRIO_INIT);
                DIST_INIT
            });

            // update shortest path to `v`
            if queue.get(&v).is_some() {
                // distance is to current node (u) + 1
                let alt = u_dist + 1;
                if alt < *v_dist {
                    // update distances to this node
                    *v_dist = alt;
                    queue.change_priority(&v, -alt);
                }
            }
        }
    }

    // print out all states
    // for dest_state in (0..problem.cycle_length).map(|phase| PosState::new(phase, problem.end)) {
    //     println!("state {dest_state:?} => {:?}", dist.get(&dest_state));
    // }

    // return shortest
    let minimum = (0..problem.cycle_length)
        .flat_map(|phase| dist.get(&PosState::new(phase, problem.end)))
        .min();
    minimum.copied()
}
