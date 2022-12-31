use anyhow::bail;
use common::*;
use regex::Regex;

use crate::{Blueprint, Cost, Mineral};
use Mineral::*;

pub fn parse_input(input: &str) -> AnyResult<Vec<Blueprint>> {
    let re_blueprint = Regex::new(r#"Blueprint (\d+)"#)?;
    let re_robot = Regex::new(r#"Each (\w+) robot"#)?;
    let re_costs = Regex::new(r#"(?:(\d+) (\w+))"#)?;

    let mut blueprints = vec![];
    for line in input.lines() {
        let (blueprint_str, robots_str) = line.split_once(':').ok_anyhow()?;
        let match_blueprint = re_blueprint.captures(blueprint_str).ok_anyhow()?;

        let id = match_blueprint.get(1).ok_anyhow()?.as_str().parse()?;
        let mut blueprint = Blueprint { id, ..Default::default() };

        for robot_spec_str in robots_str
            .split('.')
            .map(str::trim)
            .filter(|s| !s.is_empty())
        {
            let (robot_str, costs_str) = robot_spec_str.split_once("costs").ok_anyhow()?;
            let match_robot = re_robot.captures(robot_str).ok_anyhow()?;

            let mut cost = Cost::default();
            for match_cost in re_costs.captures_iter(costs_str) {
                let qty: i32 = match_cost.get(1).ok_anyhow()?.as_str().parse()?;
                let res_type: Mineral = match_cost.get(2).ok_anyhow()?.as_str().parse()?;

                match res_type {
                    Ore => cost.ore = qty,
                    Clay => cost.clay = qty,
                    Obsidian => cost.obsidian = qty,
                    Geode => bail!("invalid resource type"),
                }
            }

            let robot_type: Mineral = match_robot.get(1).ok_anyhow()?.as_str().parse()?;
            match robot_type {
                Ore => blueprint.ore_robot = cost,
                Clay => blueprint.clay_robot = cost,
                Obsidian => blueprint.obsidian_robot = cost,
                Geode => blueprint.geode_robot = cost,
            }
        }

        blueprints.push(blueprint);
    }
    Ok(blueprints)
}

#[cfg(test)]
mod tests {
    use crate::TEST_INPUT;

    use super::*;

    #[test]
    fn parse_input_correct2() {
        let input = parse_input(TEST_INPUT).unwrap();
        for i in &input {
            println!("{i:?}");
        }
    }
}
