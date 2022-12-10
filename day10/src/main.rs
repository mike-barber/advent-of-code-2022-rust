use anyhow::{anyhow, bail};
use std::{
    fs::File,
    io::Read,
};
use strum::EnumString;

#[derive(Debug, Clone, Copy, EnumString)]
enum Instruction {
    Add(i64),
    NoOp,
}
impl Instruction {
    fn cycles(&self) -> u64 {
        match self {
            Instruction::Add(_) => 2,
            Instruction::NoOp => 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Machine {
    clock: u64,
    register: i64,
}
impl Default for Machine {
    fn default() -> Self {
        Self {
            clock: 1,
            register: 1,
        }
    }
}

fn execute<F: Fn(u64) -> bool>(
    init: Machine,
    instructions: &[Instruction],
    clock_observe: F,
) -> Vec<Machine> {
    let mut machine = init;
    let mut observations = vec![];
    if clock_observe(machine.clock) {
        observations.push(machine);
    }
    for instruction in instructions {
        let cycles = instruction.cycles();
        for _ in 0..cycles-1 {
            machine.clock += 1;
            if clock_observe(machine.clock) {
                observations.push(machine);
            }
        }
        match instruction {
            Instruction::Add(x) => machine.register += x,
            Instruction::NoOp => {}
        }
        machine.clock += 1;
        if clock_observe(machine.clock) {
            observations.push(machine);
        }
    }
    observations
}

fn predicate_20_then_every_40(clock: u64) -> bool {
    match clock {
        20 => true,
        x if x > 20 && (x-20) % 40 == 0 => true,
        _ => false
    }
}

fn part1(instructions: &[Instruction]) -> i64 {
    let observations = execute(Machine::default(), instructions, predicate_20_then_every_40);
    println!("{observations:?}");
    observations
        .iter()
        .map(|m| m.clock as i64 * m.register)
        .sum()
}

fn part2(_instructions: &[Instruction]) -> i64 {
    todo!()
}

fn read_file(file_name: &str) -> String {
    let mut contents = String::new();
    File::open(file_name)
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();
    contents
}

fn parse_input(inputs: &str) -> anyhow::Result<Vec<Instruction>> {
    let mut instructions = vec![];
    for line in inputs.lines() {
        let mut split = line.split_whitespace();
        let code = split.next();
        let instruction = match code {
            Some("noop") => Instruction::NoOp,
            Some("addx") => {
                let argument = split.next().ok_or_else(|| anyhow!("no argument for add"))?;
                let argument = argument.parse()?;
                Instruction::Add(argument)
            }
            _ => bail!("unrecognised instruction {code:?}"),
        };
        instructions.push(instruction);
    }
    Ok(instructions)
}

fn main() -> anyhow::Result<()> {
    let instructions = parse_input(&read_file("input.txt"))?;
    println!("{instructions:?}");

    let part1_res = part1(&instructions);
    println!("part 1 result = {part1_res}");

    let part2_res = part2(&instructions);
    println!("part 2 result = {part2_res}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;
    use indoc::indoc;

    const BASIC_INPUT: &str = indoc! {"
        noop
        addx 3
        addx -5
    "};

    const TEST_INPUT: &str = include_str!("../input_test.txt");

    #[test]
    fn parse_inputs_succeeds() {
        parse_input(BASIC_INPUT).unwrap();
    }

    #[test]
    fn basic_input_execution() {
        let instructions = parse_input(BASIC_INPUT).unwrap();
        let observations = execute(Machine::default(), &instructions, |_| true);
        let register_vals: Vec<_> = observations.iter().map(|m| m.register).collect();
        // cycles
        // 1 -> 1 (During the first cycle, X is 1)
        // 2 -> 1 (During the second cycle, X is still 1)
        // 3 -> 1 (During the third cycle, X is still 1)
        // 4 -> 4 (During the fourth cycle, X is still 4)
        // 5 -> 4 (During the fifth cycle, X is still 4)
        // 6 -> -1 (After the fifth cycle, .. setting X to -1.)
        assert_eq!(register_vals, [1, 1, 1, 4, 4, -1])
    }

    #[test]
    fn test_input_execution_partial() {
        let instructions = parse_input(TEST_INPUT).unwrap();
        let observations = execute(Machine::default(), &instructions, predicate_20_then_every_40);
        assert_eq!(
            observations[0],
            Machine {
                clock: 20,
                register: 21
            }
        );
        assert_eq!(
            observations[1],
            Machine {
                clock: 60,
                register: 19
            }
        );
        assert_eq!(
            observations[2],
            Machine {
                clock: 100,
                register: 18
            }
        );
        assert_eq!(
            observations[3],
            Machine {
                clock: 140,
                register: 21
            }
        );
        assert_eq!(
            observations[4],
            Machine {
                clock: 180,
                register: 16
            }
        );
        assert_eq!(
            observations[5],
            Machine {
                clock: 220,
                register: 18
            }
        );
    }

    #[test]
    fn part1_correct() {
        let instructions = parse_input(TEST_INPUT).unwrap();
        let res = part1(&instructions);
        assert_eq!(res, 13140);
    }

    // #[test]
    // fn part2_correct() {
    //     let instructions = parse_input(TEST_INPUT_PART2).unwrap();
    //     let res = part2(&instructions);
    //     assert_eq!(res, 36);
    // }
}
