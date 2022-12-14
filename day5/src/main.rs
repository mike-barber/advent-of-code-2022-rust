use regex::Regex;

type Stack = Vec<char>;

#[derive(Debug, Clone)]
struct Instruction {
    count: usize,
    source: usize,
    dest: usize,
}

fn parse_input(input: &str) -> (Vec<Stack>, Vec<Instruction>) {
    let input_lines: Vec<_> = input.lines().collect();
    let mut it = input_lines.split(|l| l.is_empty());
    let input_stacks = it.next().unwrap();
    let input_instructions = it.next().unwrap();

    let stacks = parse_stacks(input_stacks);
    let instructions = parse_instructions(input_instructions);

    (stacks, instructions)
}

/// Quick and dirty implementation - reverse the lines, then use the first
/// line to determine how many stacks we have. Then iterate through the lines
/// in reverse picking out chars at specific locations (since it is layed out
/// spatially rather than delimited). Missing values are `None` are are not
/// added to stacks.
fn parse_stacks(input: &[&str]) -> Vec<Stack> {
    let mut lines_iter = input.iter().rev();
    let num_stacks = lines_iter
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .max()
        .unwrap();

    let mut stacks = vec![vec![]; num_stacks];
    for l in input.iter().rev().skip(1) {
        let lc: Vec<_> = l.chars().collect();
        #[allow(clippy::needless_range_loop)]
        for i in 0..num_stacks {
            let location = 1 + i * 4;
            let ch_maybe = lc.get(location).and_then(|c| match c {
                ' ' => None,
                c => Some(c),
            });

            if let Some(c) = ch_maybe {
                stacks[i].push(*c);
            }
        }
    }

    stacks
}

fn parse_instructions(input: &[&str]) -> Vec<Instruction> {
    // move 1 from 2 to 1
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    let mut instructions = vec![];
    for l in input {
        let cap = re.captures(l).unwrap();
        instructions.push(Instruction {
            count: cap[1].parse().unwrap(),
            source: cap[2].parse::<usize>().unwrap() - 1,
            dest: cap[3].parse::<usize>().unwrap() - 1,
        })
    }
    instructions
}

// part 1 - old crane -- move crates one at a time
fn part1_apply_instruction(stacks: &mut [Stack], instruction: &Instruction) {
    for _ in 0..instruction.count {
        let c = stacks[instruction.source].pop().unwrap();
        stacks[instruction.dest].push(c);
    }
}

fn part1(input: &str) -> String {
    let (mut stacks, instructions) = parse_input(input);
    for inst in instructions.iter() {
        part1_apply_instruction(&mut stacks, inst);
    }
    stacks.iter().map(|st| st.last().unwrap()).collect()
}

// part 2 - new crane -- move stack of crates all at once, so the order
// is preserved on placing them on the top of another stack
fn part2_apply_instruction(stacks: &mut [Stack], instruction: &Instruction) {
    let mut collected = vec![];
    for _ in 0..instruction.count {
        let c = stacks[instruction.source].pop().unwrap();
        collected.push(c);
    }

    for c in collected.iter().rev() {
        stacks[instruction.dest].push(*c);
    }
}

fn part2(input: &str) -> String {
    let (mut stacks, instructions) = parse_input(input);
    for inst in instructions.iter() {
        part2_apply_instruction(&mut stacks, inst);
    }
    stacks.iter().map(|st| st.last().unwrap()).collect()
}

fn main() -> anyhow::Result<()>{
    let contents = common::read_file("input1.txt")?;

    let part1_solution = part1(&contents);
    println!("day5 / part1: {part1_solution}");

    let part2_solution = part2(&contents);
    println!("day5 / part2: {part2_solution}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
            [D]
        [N] [C]
        [Z] [M] [P]
         1   2   3 
        
        move 1 from 2 to 1
        move 3 from 1 to 3
        move 2 from 2 to 1
        move 1 from 1 to 2
    "};

    #[test]
    fn part1_correct() {
        let res = part1(TEST_INPUT);
        assert_eq!(res, "CMZ");
    }

    #[test]
    fn part2_correct() {
        let res = part2(TEST_INPUT);
        assert_eq!(res, "MCD");
    }
}
