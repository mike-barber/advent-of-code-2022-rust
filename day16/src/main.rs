use anyhow::bail;
use arrayvec::ArrayVec;
use itertools::{iproduct, Itertools};
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use common::OptionAnyhow;

type AnyResult<T> = anyhow::Result<T>;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Code([char; 2]);
impl Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0[0], self.0[1])
    }
}
impl std::fmt::Debug for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Valve {
    code: Code,
    rate: i32,
    connects_to: Vec<Code>,
}

#[derive(Debug)]
struct Problem {
    valves: HashMap<Code, Valve>,
    start: Code,
    num_valves_with_flow: usize,
}

fn parse_code(code: &str) -> AnyResult<Code> {
    if code.len() != 2 {
        bail!("wrong number of chars for a Code: {}", code);
    }
    let mut chars = code.chars();
    Ok(Code([chars.next().unwrap(), chars.next().unwrap()]))
}

fn parse_input(input: &str) -> AnyResult<Problem> {
    let re = Regex::new(
        r#"Valve ([A-Z]+) has flow rate=(\d+); tunnel[s]? lead[s]? to valve[s]? ([A-Z, ]*)$"#,
    )?;

    //let mut first_code = ;

    let valves: AnyResult<HashMap<Code, Valve>> = input
        .lines()
        .map(|l| {
            let cap = re.captures(l).ok_anyhow()?;

            let code = parse_code(cap.get(1).ok_anyhow()?.as_str())?;
            let rate = cap.get(2).ok_anyhow()?.as_str().parse()?;

            // if first_code.is_none() {
            //     first_code = Some(code);
            // }

            let connects_to: Result<Vec<_>, _> = cap
                .get(3)
                .ok_anyhow()?
                .as_str()
                .split(',')
                .map(|c| parse_code(c.trim()))
                .collect();
            let connects_to = connects_to?;

            Ok((
                code,
                Valve {
                    code,
                    rate,
                    connects_to,
                },
            ))
        })
        .collect();

    let valves = valves?;
    let num_valves_with_flow = valves.values().filter(|v| v.rate > 0).count();

    Ok(Problem {
        valves: valves,
        start: parse_code("AA")?,
        num_valves_with_flow,
    })
}

fn check_all_bidirectional(problem: &Problem) -> AnyResult<()> {
    let valves = &problem.valves;
    for (code, valve) in valves {
        for connected in valve.connects_to.iter() {
            let other = valves.get(connected).ok_anyhow()?;
            if !other.connects_to.contains(&code) {
                bail!("Valve {connected} does not connect back to {code}");
            }
        }
    }

    Ok(())
}

const MAX_TIME: i32 = 30;

fn time_flow_from(valve: &Valve, opened_minute: i32) -> i32 {
    let total_minutes_open = MAX_TIME - opened_minute;
    valve.rate * total_minutes_open
}

fn option_max(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    match (a, b) {
        (None, None) => None,
        (None, Some(b)) => Some(b),
        (Some(a), None) => Some(a),
        (Some(a), Some(b)) => Some(a.max(b)),
    }
}

// basic DFS
fn explore_most_flow(
    problem: &Problem,
    at: &Valve,
    enabled: &HashSet<Code>,
    prior_node: Option<Code>,
    prior_time: i32,
    prior_flow: i32,
) -> Option<i32> {
    // we're at max time; nothing further we can do from here
    if prior_time == MAX_TIME {
        return Some(prior_flow);
    }

    // everything turned on; nothing we can do from here
    if enabled.len() == problem.num_valves_with_flow {
        return Some(prior_flow);
    }

    // two options at this valve
    // 1. skip over it, then consider move (by calling back to here)
    // 2. open valve then move on (if it has a non-zero flow rate)
    let mut best = None;

    // consider opening valve
    if at.rate > 0 && !enabled.contains(&at.code) {
        let mut now_enabled = enabled.clone();
        now_enabled.insert(at.code);

        let now_time = prior_time + 1;
        let now_flow = prior_flow + time_flow_from(at, now_time);
        let sub_best = explore_most_flow(problem, at, &now_enabled, None, now_time, now_flow);
        best = option_max(best, sub_best);
    }

    for next_code in at.connects_to.iter() {
        // don't go straight back to previous node - skip it
        if Some(*next_code) == prior_node {
            continue;
        }

        // consider moving to next code
        let now_time = prior_time + 1;
        let next_valve = problem.valves.get(next_code).unwrap();
        let sub_best = explore_most_flow(
            problem,
            next_valve,
            enabled,
            Some(at.code),
            now_time,
            prior_flow,
        );
        best = option_max(best, sub_best);
    }

    best
}

fn part1(problem: &Problem) -> Option<i32> {
    let start = problem.valves.get(&problem.start).unwrap();
    explore_most_flow(problem, start, &HashSet::default(), None, 0, 0)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Move {
    TurnOn,
    Code(Code),
}

fn possible_moves(
    problem: &Problem,
    at: &Valve,
    enabled: &ArrayVec<Code, 20>,
    prior_node: Option<Code>,
) -> ArrayVec<Move, 5> {
    let mut moves = ArrayVec::new();

    if at.rate > 0 && !enabled.contains(&at.code) {
        moves.push(Move::TurnOn);
    }

    for next_code in at.connects_to.iter() {
        if Some(*next_code) == prior_node {
            continue;
        }
        moves.push(Move::Code(*next_code));
    }

    moves
}

// Basic DFS with dual players
fn explore_most_flow_dual(
    problem: &Problem,
    at: [&Valve; 2],
    enabled: &ArrayVec<Code, 20>,
    prior_node: [Option<Code>; 2],
    prior_time: i32,
    prior_flow: i32,
) -> Option<i32> {
    // we're at max time; nothing further we can do from here
    if prior_time == MAX_TIME {
        return Some(prior_flow);
    }

    // everything turned on; nothing we can do from here
    if enabled.len() == problem.num_valves_with_flow {
        return Some(prior_flow);
    }

    // two options at this valve
    // 1. skip over it, then consider move (by calling back to here)
    // 2. open valve then move on (if it has a non-zero flow rate)
    let mut best = None;

    let own_moves = possible_moves(problem, at[0], enabled, prior_node[0]);
    let ele_moves = possible_moves(problem, at[1], enabled, prior_node[1]);

    for (own, ele) in iproduct!(own_moves.iter(), ele_moves.iter().rev()) {
        
        // skip case where both turn on same valve
        if at[0] == at[1] && own == &Move::TurnOn && ele == &Move::TurnOn {
            println!("Skipping dual turn on at {:?} {:?}", at[0], at[1]);
            continue;
        }

        let now_time = prior_time + 1;
        let mut now_at = at;
        let mut now_flow = prior_flow;
        let mut now_prior_node = [None; 2];
        let mut now_enabled = enabled.clone();

        match own {
            Move::TurnOn => {
                now_enabled.push(at[0].code);
                now_flow += time_flow_from(at[0], now_time)
            }
            Move::Code(c) => {
                now_at[0] = problem.valves.get(c).unwrap();
                now_prior_node[0] = Some(at[0].code);
            }
        }

        match ele {
            Move::TurnOn => {
                now_enabled.push(at[1].code);
                now_flow += time_flow_from(at[1], now_time)
            }
            Move::Code(c) => {
                now_at[1] = problem.valves.get(c).unwrap();
                now_prior_node[1] = Some(at[1].code);
            }
        }

        let sub_best = explore_most_flow_dual(
            problem,
            now_at,
            &now_enabled,
            now_prior_node,
            now_time,
            now_flow,
        );
        best = option_max(best, sub_best);
    }

    best
}

fn part2(problem: &Problem) -> Option<i32> {
    let start = problem.valves.get(&problem.start).unwrap();
    explore_most_flow_dual(
        problem,
        [start, start],
        &ArrayVec::default(),
        [None, None],
        0,
        0,
    )
}

fn main() -> anyhow::Result<()> {
    let input_string = common::read_file("day16/input.txt")?;
    let problem = parse_input(&input_string)?;
    check_all_bidirectional(&problem)?;

    // for v in problem.valves.values() {
    //     println!("{:?}", v);
    // }

    println!("Note: should NOT be 1854 - it is too high (from starting at wrong node)");
    println!("Note: should be 1741");
    println!("part1 result: {}", part1(&problem).ok_anyhow()?);

    println!("part2 result: {}", part2(&problem).ok_anyhow()?);

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
        let problem = parse_input(TEST_INPUT).unwrap();
        check_all_bidirectional(&problem).unwrap();
        println!("{problem:?}");
    }

    #[test]
    fn part1_correct() {
        let problem = parse_input(TEST_INPUT).unwrap();
        let res = part1(&problem).unwrap();
        assert_eq!(res, 1651);
    }

    #[test]
    fn part2_correct() {
        let problem = parse_input(TEST_INPUT).unwrap();
        let res = part2(&problem).unwrap();
        assert_eq!(res, 1707);
    }
}
