use anyhow::bail;
use regex::Regex;
use std::ops::{Add, Sub};

use common::OptionAnyhow;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Code<'a>(&'a str);

#[derive(Debug, Clone)]
struct Valve<'a> {
    code: Code<'a>,
    rate: i32,
    connects_to: Vec<Code<'a>>,
}

fn parse_code(code: &str) -> anyhow::Result<Code> {
    if code.len() != 2 {
        bail!("wrong number of chars for a Code: {}", code);
    }
    Ok(Code(code))
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Valve>> {
    let re = Regex::new(r#"Valve ([A-Z]+) has flow rate=(\d+); tunnel[s]? lead[s]? to valve[s]? ([A-Z, ]*)$"#)?;

    input
        .lines()
        .map(|l| {
            let cap = re.captures(l).ok_anyhow()?;

            let code = parse_code(cap.get(1).ok_anyhow()?.as_str())?;
            let rate = cap.get(2).ok_anyhow()?.as_str().parse()?;

            let connects_to: Result<Vec<_>, _> = cap
                .get(3)
                .ok_anyhow()?
                .as_str()
                .split(',')
                .map(|c| parse_code(c.trim()))
                .collect();
            let connects_to = connects_to?;

            Ok(Valve {
                code,
                rate,
                connects_to,
            })
        })
        .collect()
}

fn main() -> anyhow::Result<()> {
    let input = parse_input(&common::read_file("day16/input.txt")?)?;

    // println!("part1 result: {}", part1(&input, 2000000));
    // println!("part2 result: {}", part2(&input, 0, 4000000).ok_anyhow()?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        Valve BB has flow rate=13; tunnels lead to valves CC, AA
        Valve CC has flow rate=2; tunnels lead to valves DD, BB
        Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
        Valve EE has flow rate=3; tunnels lead to valves FF, DD
        Valve FF has flow rate=0; tunnels lead to valves EE, GG
        Valve GG has flow rate=0; tunnels lead to valves FF, HH
        Valve HH has flow rate=22; tunnel leads to valve GG
        Valve II has flow rate=0; tunnels lead to valves AA, JJ
        Valve JJ has flow rate=21; tunnel leads to valve II
    "};

    #[test]
    fn parse_inputs_succeeds() {
        let valves = parse_input(TEST_INPUT).unwrap();
        println!("{valves:?}");
    }

    // #[test]
    // fn part1_alt_correct() {
    //     let measurements = parse_input(TEST_INPUT).unwrap();
    //     let res = part1(&measurements, 10);
    //     assert_eq!(res, 26);
    // }

    // #[test]
    // fn part2_correct() {
    //     let measurements = parse_input(TEST_INPUT).unwrap();
    //     let res = part2(&measurements, 0, 20).unwrap();
    //     assert_eq!(res, 56000011);
    // }
}
