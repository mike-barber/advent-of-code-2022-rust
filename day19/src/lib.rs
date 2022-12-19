pub mod parser;

use indoc::indoc;
use std::str::FromStr;
use Mineral::*;

pub const TEST_INPUT: &str = indoc! {"
    Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
    Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
"};

pub enum Mineral {
    Ore,
    Clay,
    Obsidian,
    Geode,
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

#[derive(Debug, Clone, Default, Copy)]
pub struct Cost {
    pub ore: i32,
    pub clay: i32,
    pub obsidian: i32,
}
