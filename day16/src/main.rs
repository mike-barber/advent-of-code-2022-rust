use anyhow::bail;
use regex::Regex;
use std::{ops::{Add, Sub}, collections::{HashSet, HashMap}};

use common::OptionAnyhow;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Code<'a>(&'a str);

#[derive(Debug, Clone, PartialEq, Eq)]
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

const MAX_TIME: i32 = 30;

fn total_flow(valve: &Valve, opened_minute: i32) -> i32 {
    let total_minutes_open = MAX_TIME - opened_minute;
    valve.rate * total_minutes_open
}

fn option_max(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    match (a,b) {
        (None, None) => None,
        (None, Some(b)) => Some(b),
        (Some(a), None) => Some(a),
        (Some(a), Some(b)) => Some(a.max(b))
    }
}

// basic DFS
fn explore_most_flow(valves: &HashMap<Code, &Valve>, at: &Valve, prior_seen: &Vec<Code>, prior_time: i32, prior_flow: i32) -> Option<i32> {

    if prior_time == MAX_TIME {
        return Some(prior_flow)
    }

    let mut seen = prior_seen.clone();
    seen.push(at.code);

    let mut best = None;
    for next in at.connects_to.iter().filter(|v| !prior_seen.contains(*v)) {

        let next_valve = valves.get(next).unwrap();

        // consider opening valve
        if at.rate > 0 && prior_time < MAX_TIME - 1{
            let time = prior_time + 1;
            let flow = prior_flow + total_flow(at, time);

            if time < MAX_TIME {

            }
        }

        // consider skipping opening valve and moving
        if prior_time < MAX_TIME {
            let time = prior_time + 1;
            let flow = prior_flow;

            let explored_flow = explore_most_flow(valves, next_valve, &seen, time, flow);
            best = option_max(best, explored_flow);
        } 
    }

    best
}


fn main() -> anyhow::Result<()> {
    let input_string = common::read_file("day16/input.txt")?;
    let input = parse_input(&input_string)?;
    println!("{:#?}", input);

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
