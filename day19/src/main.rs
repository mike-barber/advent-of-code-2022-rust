use std::str::FromStr;

use anyhow::bail;
use common::*;
use regex::Regex;

use RobotType::*;

enum RobotType {
    Ore,
    Clay,
    Obsidian,
    Geode
}
impl FromStr for RobotType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ore" => Ok(Ore),
            "clay" => Ok(Clay),
            "obsidian" => Ok(Obsidian),
            "geode" => Ok(Geode),
            _ => Err(anyhow::anyhow!("unknown resource type"))
        }
    }
}


#[derive(Debug, Clone)]
struct Blueprint {
    id: i32,
    ore_robot: Cost,
    clay_robot: Cost,
    obsidian_robot: Cost,
    geode_robot: Cost,
}

#[derive(Debug, Clone, Default)]
struct Cost {
    ore: i32,
    clay: i32,
    obsidian: i32,
}

fn parse_input(input: &str) -> AnyResult<Vec<Blueprint>> {
    let re_blueprint = Regex::new(r#"Blueprint (\d+)"#)?;
    let re_robot = Regex::new(r#"Each (\w+) robot"#)?;
    let re_costs = Regex::new(r#"(?:(\d+) (\w+))"#)?;

    let mut blueprints = vec![];
    for line in input.lines() {
        let (blueprint_str, robots_str) = line.split_once(":").unwrap();
        let match_blueprint = re_blueprint.captures(blueprint_str).unwrap();
        println!("{match_blueprint:?}");

        for robot_spec_str in robots_str
            .split(".")
            .map(str::trim)
            .filter(|s| !s.is_empty())
        {
            let (robot_str, costs_str) = robot_spec_str.split_once("costs").unwrap();
            let match_robot = re_robot.captures(robot_str).unwrap();
            println!("{match_robot:?}");

            let mut cost = Cost::default();
            for match_cost in re_costs.captures_iter(costs_str) {
                let qty: i32 = match_cost.get(1).ok_anyhow()?.as_str().parse()?;
                let res_type: RobotType = match_cost.get(2).ok_anyhow()?.as_str().parse()?;

                println!("{match_cost:?}");
                match res_type {
                    Ore => { cost.ore = qty },
                    Clay => { cost.clay = qty },
                    Obsidian => {cost.obsidian = qty},
                    Geode => bail!("invalid resource type"),
                }
            }
            dbg!(&cost);
        }
    }

    Ok(blueprints)
}

fn main() -> AnyResult<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
        Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
    "};

    #[test]
    fn parse_input_correct2() {
        let input = parse_input(TEST_INPUT).unwrap();
        for i in &input {
            println!("{i:?}");
        }
    }

    // #[test]
    // fn part1_correct() {
    //     let input = parse_input(TEST_INPUT);
    //     let res = part1(&input).unwrap();
    //     assert_eq!(res, 64);
    // }

    // #[test]
    // fn part2_correct() {
    //     let input = parse_input(TEST_INPUT);
    //     let res = part2(&input).unwrap();
    //     assert_eq!(res, 58);
    // }
}
