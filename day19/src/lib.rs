pub mod parser;

use arrayvec::ArrayVec;
use indoc::indoc;
use nalgebra::Vector4;
use priority_queue::PriorityQueue;
use rayon::prelude::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use rustc_hash::FxHashSet;
use std::{hash::Hash, str::FromStr, time::Instant};
use Mineral::*;

pub const TEST_INPUT: &str = indoc! {"
    Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
    Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
"};

const TIME_MAX_PART1: usize = 24;
const TIME_MAX_PART2: usize = 32;

#[derive(Debug, Clone, Copy)]
pub enum Mineral {
    Ore = 0,
    Clay = 1,
    Obsidian = 2,
    Geode = 3,
}
impl FromStr for Mineral {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ore" => Ok(Ore),
            "clay" => Ok(Clay),
            "obsidian" => Ok(Obsidian),
            "geode" => Ok(Geode),
            _ => Err(anyhow::anyhow!("unknown resource type")),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Blueprint {
    pub id: i32,
    pub ore_robot: Cost,
    pub clay_robot: Cost,
    pub obsidian_robot: Cost,
    pub geode_robot: Cost,
}
impl Blueprint {
    pub fn to_spec(&self, max_time: usize) -> BlueprintSpec {
        let ore_robot = self.ore_robot.to_spec();
        let clay_robot = self.clay_robot.to_spec();
        let obsidian_robot = self.obsidian_robot.to_spec();
        let geode_robot = self.geode_robot.to_spec();

        // calculate maximum possible consumption rate, given that we can
        // only build a single robot each minute: this is the max of resources
        // required across all robots, and dictates the maximum number of
        // useful robots of each type.
        let mut maximum_useful_robots = Vector4::zeros();
        for i in 0..3 {
            maximum_useful_robots[i] = ore_robot[i]
                .max(clay_robot[i])
                .max(obsidian_robot[i])
                .max(geode_robot[i]);
        }
        // no limit on geode-producing robots
        maximum_useful_robots[3] = i32::MAX;

        //dbg!(&maximum_useful_robots);

        BlueprintSpec {
            max_time,
            ore_robot,
            clay_robot,
            obsidian_robot,
            geode_robot,
            maximum_useful_robots,
        }
    }
}

#[derive(Debug, Clone, Default, Copy)]
pub struct Cost {
    pub ore: i32,
    pub clay: i32,
    pub obsidian: i32,
}
impl Cost {
    pub fn to_spec(&self) -> Vector4<i32> {
        Vector4::new(self.ore, self.clay, self.obsidian, 0)
    }
}

#[derive(Debug, Clone, Default, Copy, PartialEq, PartialOrd, Hash)]
pub struct BlueprintSpec {
    pub max_time: usize,
    pub ore_robot: Vector4<i32>,
    pub clay_robot: Vector4<i32>,
    pub obsidian_robot: Vector4<i32>,
    pub geode_robot: Vector4<i32>,
    pub maximum_useful_robots: Vector4<i32>,
}
impl BlueprintSpec {
    pub fn required_resources(&self, robot: Mineral) -> &Vector4<i32> {
        match robot {
            Ore => &self.ore_robot,
            Clay => &self.clay_robot,
            Obsidian => &self.obsidian_robot,
            Geode => &self.geode_robot,
        }
    }
}

#[derive(Debug, Clone, Default, Copy, PartialEq, PartialOrd, Hash, Eq)]
pub struct State {
    time: usize,
    resources: Vector4<i32>,
    robots: Vector4<i32>,
}
impl State {
    pub fn advance(&self) -> State {
        let resources = self.resources + self.robots;
        let time = self.time + 1;
        State {
            resources,
            time,
            ..*self
        }
    }

    pub fn new() -> State {
        State {
            time: 0,
            resources: Vector4::new(0, 0, 0, 0),
            robots: Vector4::new(1, 0, 0, 0),
        }
    }
}

pub fn try_advance_with(spec: &BlueprintSpec, state: &State, new_robot: Mineral) -> Option<State> {
    let resources_required = spec.required_resources(new_robot);
    let resources_after = state.resources - resources_required;

    // insufficient resources - not a possible move
    if resources_after.min() < 0 {
        return None;
    }

    // increment robot
    let mut robots_after = state.robots;
    robots_after[new_robot as usize] += 1;

    // update resources and advance state
    let mut new_state = State {
        resources: resources_after,
        ..*state
    }
    .advance();

    // then finally update the number of robots _last_, after advancing
    // the state; the order is important here.
    new_state.robots = robots_after;

    Some(new_state)
}

pub type PossibleStates = ArrayVec<State, 5>;

pub fn possible_states_from(spec: &BlueprintSpec, state: &State) -> PossibleStates {
    let mut possible = ArrayVec::new();

    // try building Geode robot; and this is greedy -- only consider other alternatives
    // if we can't do this.
    if let Some(s) = try_advance_with(spec, state, Geode) {
        possible.push(s);
    } else {
        for robot in [Obsidian, Clay, Ore] {
            // check that we're below the limit for the number of robots of this type:
            // there's no point building more than required.
            let maximum_useful = spec.maximum_useful_robots[robot as usize];
            if state.robots[robot as usize] >= maximum_useful {
                continue;
            }

            // otherwise consider building one if possible
            if let Some(s) = try_advance_with(spec, state, robot) {
                possible.push(s);
            }
        }

        // add time advance with no new robots
        possible.push(state.advance());
    }

    possible
}

/// Very basic bound condition - estimate the maximum theoretically possible
/// number of geodes producible at the end of the simulation from this State,
/// assuming that we add a new geode factory on every iteration (regardless of
/// the number of actual other factories present). Think `s = ut + 1/2at^2`
pub fn simple_max_potential_geodes(state: &State, max_time: usize) -> i32 {
    let remaining_time = max_time - state.time;
    let geodes = state.resources[Geode as usize];
    let robots = state.robots[Geode as usize];

    let u = robots;
    let t = remaining_time as i32;
    let manufactured = u * t + t * (t - 1) / 2;

    geodes + manufactured
}

/// Original recursive DFS I used to solve the problem. I've replaced it with a faster
/// priority-queue based version below.
#[allow(dead_code)]
pub fn explore_dfs_max_original(
    spec: &BlueprintSpec,
    state: &State,
    global_best: &mut Option<State>,
) {
    for next_state in possible_states_from(spec, state) {
        // skip if we're almost at termination and have no geode factories
        if next_state.time == spec.max_time - 1 && next_state.robots[Geode as usize] == 0 {
            continue;
        }

        // termination and update global best -- step before final time step; no point exploring
        // from here, since it is always suboptimal to create a new factory at this stage.
        if next_state.time == spec.max_time - 1 {
            // advance to final state
            let final_state = next_state.advance();
            if let Some(existing_best) = global_best {
                if final_state.resources[Geode as usize] > existing_best.resources[Geode as usize] {
                    *existing_best = final_state;
                }
            } else {
                global_best.replace(final_state);
            }

            continue;
        }

        // check if next state _could_ be better than our existing global best; skip if not
        if let Some(existing_best) = global_best {
            let geodes = existing_best.resources[Geode as usize];
            let potential = simple_max_potential_geodes(&next_state, spec.max_time);
            if potential <= geodes {
                continue;
            }
        }

        // not complete yet; recurse
        explore_dfs_max_original(spec, &next_state, global_best);
    }
}

/// Fairly simple DFS using a priority queue based on an estimate
/// for the maximum possible geodes from a given state; this is
/// also used as an upper bound to reject states that cannot possibly
/// improve on the existing best.
fn explore_prio_max(spec: &BlueprintSpec) -> Option<State> {
    let mut global_best = None;

    let mut visited: FxHashSet<State> = FxHashSet::default();
    let mut queue: PriorityQueue<State, i32> = PriorityQueue::new();

    let initial = State::new();
    queue.push(
        initial,
        simple_max_potential_geodes(&initial, spec.max_time),
    );
    while let Some((v, _prio)) = queue.pop() {
        // terminal node
        if v.time == spec.max_time {
            // update best
            let best = global_best.get_or_insert(v);
            if v.resources[Geode as usize] > best.resources[Geode as usize] {
                global_best.replace(v);
            }
            continue;
        }

        // explore from here
        for w in possible_states_from(spec, &v) {
            if !visited.contains(&w) {
                visited.insert(w);

                // check if next state _could_ be better than our existing global best; skip if not
                let max_potential = simple_max_potential_geodes(&w, spec.max_time);
                if let Some(best) = global_best {
                    let geodes = best.resources[Geode as usize];
                    let potential = max_potential;
                    if potential <= geodes {
                        continue;
                    }
                }

                // otherwise add to queue for exploration
                queue.push(w, max_potential);
            }
        }
    }

    global_best
}

pub fn part1(blueprints: &[Blueprint]) -> i32 {
    let sum: i32 = blueprints
        .par_iter()
        .map(|bp| {
            let spec = bp.to_spec(TIME_MAX_PART1);
            let best = explore_prio_max(&spec);

            let id = bp.id;
            let geodes = best.map(|b| b.resources[Geode as usize]).unwrap_or(0);
            println!("part1 id {id} with {geodes} geodes");

            id * geodes
        })
        .sum();

    sum
}

pub fn part2(blueprints: &[Blueprint]) -> i32 {
    let product: i32 = blueprints
        .par_iter()
        .take(3)
        .map(|bp| {
            let t0 = Instant::now();

            let spec = bp.to_spec(TIME_MAX_PART2);
            let best = explore_prio_max(&spec);

            let id = bp.id;
            let geodes = best.unwrap().resources[Geode as usize];
            let elapsed = Instant::now() - t0;
            println!("part2 id {id} with {geodes} geodes after {elapsed:#?}");

            geodes
        })
        .product();

    product
}

#[cfg(test)]
mod tests {
    use crate::{parser::parse_input, *};

    fn blueprints() -> Vec<Blueprint> {
        parse_input(TEST_INPUT).unwrap()
    }

    #[test]
    #[ignore = "original DFS algorithm takes too long to run in debug mode"]
    fn part1_blueprint1_original_correct() {
        let spec = blueprints()[0].to_spec(TIME_MAX_PART1);
        let mut best = None;
        explore_dfs_max_original(&spec, &State::new(), &mut best);
        let best = best.unwrap();
        assert_eq!(best.resources[Geode as usize], 9);
        assert_eq!(best.time, TIME_MAX_PART1);
    }

    #[test]
    fn part1_blueprint1_prio_correct() {
        let spec = blueprints()[0].to_spec(TIME_MAX_PART1);
        let best = explore_prio_max(&spec).unwrap();
        assert_eq!(best.resources[Geode as usize], 9);
        assert_eq!(best.time, TIME_MAX_PART1);
    }

    #[test]
    #[ignore = "original DFS algorithm takes too long to run in debug mode"]
    fn part1_blueprint2_original_correct() {
        let spec = blueprints()[1].to_spec(TIME_MAX_PART1);
        let mut best = None;
        explore_dfs_max_original(&spec, &State::new(), &mut best);
        let best = best.unwrap();
        assert_eq!(best.resources[Geode as usize], 12);
        assert_eq!(best.time, TIME_MAX_PART1);
    }

    #[test]
    fn part1_blueprint2_prio_correct() {
        let spec = blueprints()[1].to_spec(TIME_MAX_PART1);
        let best = explore_prio_max(&spec).unwrap();
        assert_eq!(best.resources[Geode as usize], 12);
        assert_eq!(best.time, TIME_MAX_PART1);
    }

    #[test]
    fn part1_correct() {
        let blueprints = parse_input(TEST_INPUT).unwrap();
        let res = part1(&blueprints);
        assert_eq!(res, 33);
    }

    #[test]
    fn part2_blueprint1_correct() {
        let spec = blueprints()[0].to_spec(TIME_MAX_PART2);
        let best = explore_prio_max(&spec).unwrap();
        assert_eq!(best.resources[Geode as usize], 56);
        assert_eq!(best.time, TIME_MAX_PART2);
    }

    #[test]
    fn part2_blueprint2_correct() {
        let spec = blueprints()[1].to_spec(TIME_MAX_PART2);
        let best = explore_prio_max(&spec).unwrap();
        assert_eq!(best.resources[Geode as usize], 62);
        assert_eq!(best.time, TIME_MAX_PART2);
    }
}
