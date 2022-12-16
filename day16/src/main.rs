use anyhow::bail;
use arrayvec::ArrayVec;
use itertools::iproduct;
use regex::Regex;
use std::{collections::HashMap, fmt::Display};

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
    permitted_time: i32,
}
impl Problem {
    fn time_flow_from(&self, valve_rate: i32, opened_minute: i32) -> i32 {
        let total_minutes_open = self.permitted_time - opened_minute;
        valve_rate * total_minutes_open
    }
}

fn parse_code(code: &str) -> AnyResult<Code> {
    if code.len() != 2 {
        bail!("wrong number of chars for a Code: {}", code);
    }
    let mut chars = code.chars();
    Ok(Code([chars.next().unwrap(), chars.next().unwrap()]))
}

fn parse_input(input: &str, permitted_time: i32) -> AnyResult<Problem> {
    let re = Regex::new(
        r#"Valve ([A-Z]+) has flow rate=(\d+); tunnel[s]? lead[s]? to valve[s]? ([A-Z, ]*)$"#,
    )?;

    let valves: AnyResult<HashMap<Code, Valve>> = input
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
        valves,
        start: parse_code("AA")?,
        num_valves_with_flow,
        permitted_time,
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

// basic DFS
fn explore_most_flow(
    problem: &Problem,
    global_best_found: &mut i32,
    at: &Valve,
    remaining_potential: i32,
    turned_on: &ArrayVec<Code, 20>,
    prior_node: Option<Code>,
    prior_time: i32,
    prior_flow: i32,
) -> i32 {
    // we're at max time; nothing further we can do from here
    if prior_time == problem.permitted_time {
        return prior_flow;
    }

    // everything turned on; nothing we can do from here
    if turned_on.len() == problem.num_valves_with_flow {
        return prior_flow;
    }

    // two options at this valve
    // 1. skip over it, then consider move (by calling back to here)
    // 2. open valve then move on (if it has a non-zero flow rate)
    for next_move in possible_moves(at, turned_on, prior_node) {
        let now_time = prior_time + 1;
        let mut now_remaining_potential = remaining_potential;
        let mut now_at = at;
        let mut now_flow = prior_flow;
        let mut now_prior_node = None;
        let mut now_enabled = turned_on.clone();

        match next_move {
            Move::TurnOn => {
                now_enabled.push(at.code);
                now_flow += problem.time_flow_from(at.rate, now_time);
                now_remaining_potential -= at.rate;
            }
            Move::Code(c) => {
                now_at = problem.valves.get(&c).unwrap();
                now_prior_node = Some(at.code);
            }
        }

        // DP part: skip expensive recursion if the remaining value at this point is below the current
        // best estimate we've found. The value we realise from recursion is strictly less than this value.
        let remaining_time_value = problem.time_flow_from(now_remaining_potential, now_time);
        let maximum_payoff = now_flow + remaining_time_value;
        if maximum_payoff < *global_best_found {
            continue;
        }

        let sub_best = explore_most_flow(
            problem,
            global_best_found,
            now_at,
            now_remaining_potential,
            &now_enabled,
            now_prior_node,
            now_time,
            now_flow,
        );

        *global_best_found = sub_best.max(*global_best_found);
    }

    *global_best_found
}

fn part1(problem: &Problem) -> i32 {
    let start = problem.valves.get(&problem.start).unwrap();
    let remaining_potential = problem.valves.values().map(|v| v.rate).sum();
    let mut best = 0;
    explore_most_flow(
        problem,
        &mut best,
        start,
        remaining_potential,
        &ArrayVec::default(),
        None,
        0,
        0,
    )
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Move {
    TurnOn,
    Code(Code),
}

fn possible_moves(
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
    global_best_found: &mut i32,
    at: [&Valve; 2],
    remaining_potential: i32,
    turned_on: &ArrayVec<Code, 20>,
    prior_node: [Option<Code>; 2],
    prior_time: i32,
    prior_flow: i32,
) -> i32 {
    // we're at max time; nothing further we can do from here
    if prior_time == problem.permitted_time {
        // println!("max time with flow: {prior_flow}");
        return prior_flow;
    }

    // everything turned on; nothing we can do from here
    if turned_on.len() == problem.num_valves_with_flow {
        // println!("everything on with flow: {prior_flow} at time {prior_time}");
        return prior_flow;
    }

    // two options at this valve
    // 1. skip over it, then consider move (by calling back to here)
    // 2. open valve then move on (if it has a non-zero flow rate)
    let own_moves = possible_moves(at[0], turned_on, prior_node[0]);
    let ele_moves = possible_moves(at[1], turned_on, prior_node[1]);

    for (own, ele) in iproduct!(own_moves.iter(), ele_moves.iter().rev()) {
        // skip case where both turn on same valve
        if at[0] == at[1] && own == &Move::TurnOn && ele == &Move::TurnOn {
            //println!("Skipping dual turn on at {:?} {:?}", at[0], at[1]);
            continue;
        }

        let now_time = prior_time + 1;
        let mut now_at = at;
        let mut now_remaining_potential = remaining_potential;
        let mut now_flow = prior_flow;
        let mut now_prior_node = [None; 2];
        let mut now_enabled = turned_on.clone();

        match own {
            Move::TurnOn => {
                now_enabled.push(at[0].code);
                now_flow += problem.time_flow_from(at[0].rate, now_time);
                now_remaining_potential -= at[0].rate;
            }
            Move::Code(c) => {
                now_at[0] = problem.valves.get(c).unwrap();
                now_prior_node[0] = Some(at[0].code);
            }
        }

        match ele {
            Move::TurnOn => {
                now_enabled.push(at[1].code);
                now_flow += problem.time_flow_from(at[1].rate, now_time);
                now_remaining_potential -= at[1].rate;
            }
            Move::Code(c) => {
                now_at[1] = problem.valves.get(c).unwrap();
                now_prior_node[1] = Some(at[1].code);
            }
        }

        // DP part: skip expensive recursion if the remaining value at this point is below the current
        // best estimate we've found. The value we realise from recursion is strictly less than this value.
        let remaining_time_value = problem.time_flow_from(now_remaining_potential, now_time);
        let maximum_payoff = now_flow + remaining_time_value;
        if maximum_payoff < *global_best_found {
            continue;
        }

        let sub_best = explore_most_flow_dual(
            problem,
            global_best_found,
            now_at,
            now_remaining_potential,
            &now_enabled,
            now_prior_node,
            now_time,
            now_flow,
        );

        *global_best_found = sub_best.max(*global_best_found);
    }

    *global_best_found
}

fn part2(problem: &Problem) -> i32 {
    let start = problem.valves.get(&problem.start).unwrap();
    let mut best_found = 0;
    let remaining_potential = problem.valves.values().map(|v| v.rate).sum();
    explore_most_flow_dual(
        problem,
        &mut &mut best_found,
        [start, start],
        remaining_potential,
        &ArrayVec::default(),
        [None, None],
        0,
        0,
    )
}

const TIME_PART1: i32 = 30;
const TIME_PART2: i32 = 26;

fn main() -> anyhow::Result<()> {
    let input_string = common::read_file("day16/input.txt")?;
    let problem_part1 = parse_input(&input_string, TIME_PART1)?;
    check_all_bidirectional(&problem_part1)?;

    // for v in problem.valves.values() {
    //     println!("{:?}", v);
    // }

    println!("Note: should NOT be 1854 - it is too high (from starting at wrong node)");
    println!("Note: should be 1741");
    println!("part1 result: {}", part1(&problem_part1));

    let problem_part2 = parse_input(&input_string, TIME_PART2)?;
    println!("part2 result: {}", part2(&problem_part2));

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
        let problem = parse_input(TEST_INPUT, TIME_PART1).unwrap();
        check_all_bidirectional(&problem).unwrap();
        println!("{problem:?}");
    }

    #[test]
    fn part1_correct() {
        let problem = parse_input(TEST_INPUT, TIME_PART1).unwrap();
        let res = part1(&problem);
        assert_eq!(res, 1651);
    }

    #[test]
    fn part2_correct() {
        let problem = parse_input(TEST_INPUT, TIME_PART2).unwrap();
        let res = part2(&problem);
        assert_eq!(res, 1707);
    }
}
