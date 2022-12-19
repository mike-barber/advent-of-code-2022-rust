pub mod parser;

use std::str::FromStr;
use Mineral::*;



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