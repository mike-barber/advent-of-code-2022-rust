pub mod parser;

use arrayvec::ArrayVec;
use indoc::indoc;
use nalgebra::Vector4;
use std::{arch::x86_64::_MM_GET_ROUNDING_MODE, str::FromStr};
use Mineral::*;

pub const TEST_INPUT: &str = indoc! {"
    Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
    Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
"};

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
    pub fn to_spec(&self) -> BlueprintSpec {
        BlueprintSpec {
            ore_robot: self.ore_robot.to_spec(),
            clay_robot: self.clay_robot.to_spec(),
            obsidian_robot: self.obsidian_robot.to_spec(),
            geode_robot: self.geode_robot.to_spec(),
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
    pub ore_robot: Vector4<i32>,
    pub clay_robot: Vector4<i32>,
    pub obsidian_robot: Vector4<i32>,
    pub geode_robot: Vector4<i32>,
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

#[derive(Debug, Clone, Default, Copy, PartialEq, PartialOrd, Hash)]
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

    let mut new_state = State {
        resources: resources_after,
        ..*state
    }
    .advance();
    new_state.robots = robots_after;

    Some(new_state)
}

pub type PossibleStates = ArrayVec<State, 5>;

pub fn possible_states_from(spec: &BlueprintSpec, state: &State) -> PossibleStates {
    let mut possible = ArrayVec::new();

    // try robots; highest-value first
    for robot in [Geode, Obsidian, Clay, Ore] {
        if let Some(s) = try_advance_with(spec, state, robot) {
            possible.push(s);
        }
    }

    // add time advance with no new robots
    possible.push(state.advance());
    possible
}

const TIME_MAX: usize = 24;

/// Very basic DP condition - estimate the maximum theoretically possible
/// number of geodes producible at the end of the simulation from this State, 
/// assuming that we add a new geode factory on every iteration (regardless of 
/// the number of actual other factories present). Think `s = ut + 1/2at^2`
pub fn simple_max_potential_geodes(state: &State) -> i32 {
    let remaining_time = TIME_MAX - state.time;
    let geodes = state.resources[Geode as usize];
    let robots = state.robots[Geode as usize];

    let u = robots;
    let t = remaining_time as i32;
    let manufactured = u * t + t * (t - 1) / 2;

    geodes + manufactured
}

pub fn explore_dfs_max(spec: &BlueprintSpec, state: &State, global_best: &mut Option<State>) {
    for next_state in possible_states_from(spec, state) {
        // termination and update global best
        if next_state.time == TIME_MAX {
            if let Some(existing_best) = global_best {
                if next_state.resources[Geode as usize] > existing_best.resources[Geode as usize] {
                    *existing_best = next_state;
                }
            } else {
                global_best.replace(next_state);
            }

            continue;
        } 

        // check if next state _could_ be better than our existing global best; skip if not
        if let Some(existing_best) = global_best{
            let geodes = existing_best.resources[Geode as usize];
            let potential = simple_max_potential_geodes(&next_state);
            if potential <= geodes {
                continue;
            }
        }

        // not complete yet; recurse
        explore_dfs_max(spec, &next_state, global_best);
    }
}

#[cfg(test)]
mod tests {
    use crate::{parser::parse_input, *};

    fn blueprints() -> Vec<Blueprint> {
        parse_input(TEST_INPUT).unwrap()
    }

    #[test]
    fn blueprint1_correct() {
        let spec = blueprints()[0].to_spec();
        let mut best = None;
        explore_dfs_max(&spec, &State::new(), &mut best);
        let best = best.unwrap();
        assert_eq!(best.resources[Geode as usize], 9);
        assert_eq!(best.time, 24);
    }

    #[test]
    fn blueprint2_correct() {
        let spec = blueprints()[1].to_spec();
        let mut best = None;
        explore_dfs_max(&spec, &State::new(), &mut best);
        let best = best.unwrap();
        assert_eq!(best.resources[Geode as usize], 12);
        assert_eq!(best.time, 24);
    }
}
