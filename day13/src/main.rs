use std::{cmp::Ordering, fs::File, io::Read};

use day13::*;

fn read_file(file_name: &str) -> anyhow::Result<String> {
    let mut contents = String::new();
    File::open(file_name)?.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse_input(inputs: &str) -> anyhow::Result<Problem> {
    let lines: Vec<_> = inputs.lines().collect();
    let groups = lines.split(|l| l.is_empty());

    let mut pairs = vec![];
    for g in groups {
        let v1 = parser::parse(g.get(0).ok_anyhow()?)?;
        let v2 = parser::parse(g.get(1).ok_anyhow()?)?;
        pairs.push(Pair(v1, v2))
    }
    Ok(Problem { pairs })
}

fn part1(problem: &Problem) -> anyhow::Result<usize> {
    let mut indices = vec![];
    for (i, pair) in problem.pairs.iter().enumerate() {
        let cmp = pair.0.partial_cmp(&pair.1).ok_anyhow()?;
        if cmp == Ordering::Less {
            indices.push(i + 1)
        }
    }

    Ok(indices.iter().sum())
}

fn part2(problem: &Problem) -> anyhow::Result<usize> {
    // get all packets
    let mut all_packets: Vec<_> = problem
        .pairs
        .iter()
        .cloned()
        .flat_map(|p| [p.0, p.1])
        .collect();

    // add dividers
    let divider_2: Value = vec![vec![2.into()].into()].into();
    let divider_6: Value = vec![vec![6.into()].into()].into();
    all_packets.push(divider_2.clone());
    all_packets.push(divider_6.clone());

    // sort
    all_packets.sort();

    // find indices of the divider packets
    let index_divider_2 = all_packets
        .iter()
        .position(|p| p == &divider_2)
        .ok_anyhow()?;
    let index_divider_6 = all_packets
        .iter()
        .position(|p| p == &divider_6)
        .ok_anyhow()?;

    Ok((index_divider_2 + 1) * (index_divider_6 + 1))
}

fn main() -> anyhow::Result<()> {
    let problem = parse_input(&read_file("input.txt")?)?;

    println!("part1 result: {}", part1(&problem)?);
    println!("part1 result: {}", part2(&problem)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        [1,1,3,1,1]
        [1,1,5,1,1]
        
        [[1],[2,3,4]]
        [[1],4]
        
        [9]
        [[8,7,6]]
        
        [[4,4],4,4]
        [[4,4],4,4,4]
        
        [7,7,7,7]
        [7,7,7]
        
        []
        [3]
        
        [[[]]]
        [[]]
        
        [1,[2,[3,[4,[5,6,7]]]],8,9]
        [1,[2,[3,[4,[5,6,0]]]],8,9]
    "};

    #[test]
    fn parse_inputs_succeeds() {
        parse_input(TEST_INPUT).unwrap();
    }

    #[test]
    fn part1_correct() {
        let problem = parse_input(TEST_INPUT).unwrap();
        let solution = part1(&problem).unwrap();
        assert_eq!(solution, 13);
    }

    #[test]
    fn part2_correct() {
        let problem = parse_input(TEST_INPUT).unwrap();
        let solution = part2(&problem).unwrap();
        assert_eq!(solution, 140);
    }
}
