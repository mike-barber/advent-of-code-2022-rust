use std::{fs::File, io::Read, ops::RangeInclusive};

type Range = RangeInclusive<i32>;

#[derive(Debug, Clone)]
struct AssignmentPair(Range, Range);

fn parse_range(s: &str) -> Range {
    let mut iter = s.split('-');
    let start = iter.next().unwrap().parse().unwrap();
    let end = iter.next().unwrap().parse().unwrap();
    Range::new(start, end)
}

fn read_file(file_name: &str) -> String {
    let mut contents = String::new();
    File::open(file_name)
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();
    contents
}

fn parse_input(input: &str) -> Vec<AssignmentPair> {
    input
        .lines()
        .map(|l| {
            let mut iter = l.split(',');
            let rng1 = parse_range(iter.next().unwrap());
            let rng2 = parse_range(iter.next().unwrap());
            AssignmentPair(rng1, rng2)
        })
        .collect()
}

fn either_is_subset(pair: &AssignmentPair) -> bool {
    is_subset_of(&pair.0, &pair.1) || is_subset_of(&pair.1, &pair.0)
}

fn is_subset_of(a: &Range, b: &Range) -> bool {
    a.start() >= b.start() && a.end() <= b.end()
}

fn is_any_overlap(pair: &AssignmentPair) -> bool {
    pair.0.contains(pair.1.start())
        || pair.0.contains(pair.1.end())
        || pair.1.contains(pair.0.start())
        || pair.1.contains(pair.0.end())
}

fn part1(pairs: &[AssignmentPair]) -> usize {
    pairs.iter().filter(|p| either_is_subset(p)).count()
}

fn part2(pairs: &[AssignmentPair]) -> usize {
    pairs.iter().filter(|p| is_any_overlap(p)).count()
}

fn main() {
    let contents = read_file("input1.txt");
    let input = parse_input(&contents);

    let part1_solution = part1(&input);
    println!("day4 / part1: {part1_solution}");

    let part2_solution = part2(&input);
    println!("day4 / part2: {part2_solution}");
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::*;

    const TEST_INPUT: &str = indoc! {"
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8
    "};

    #[test]
    fn part1_correct() {
        let input = parse_input(TEST_INPUT);
        let res = part1(&input);
        assert_eq!(res, 2);
    }

    #[test]
    fn part2_correct() {
        let input = parse_input(TEST_INPUT);
        let res = part2(&input);
        assert_eq!(res, 4);
    }
}
