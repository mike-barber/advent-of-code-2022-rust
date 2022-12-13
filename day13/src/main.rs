use std::{fs::File, io::Read};

use day13::*;

fn read_file(file_name: &str) -> String {
    let mut contents = String::new();
    File::open(file_name)
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();
    contents
}

fn parse_input(inputs: &str) -> anyhow::Result<Problem> {
    let lines: Vec<_> = inputs.lines().collect();
    let groups = lines.split(|l| l.is_empty());

    let mut pairs = vec![];
    for g in groups {
        let v1 = parser::parse(g.get(0).ok_anyhow()?)?;
        let v2 = parser::parse(g.get(1).ok_anyhow()?)?;
        pairs.push(Pair(v1,v2))
    }
    Ok(Problem{pairs})
}

fn main() {}

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

    // #[test]
    // fn part1_correct() {
    //     let problem = parse_input(TEST_INPUT).unwrap();
    //     let solution = part1(&problem).unwrap();
    //     assert_eq!(solution, 31);
    // }

    // #[test]
    // fn part2_correct() {
    //     let problem = parse_input(TEST_INPUT).unwrap();
    //     let solution = part2(&problem);
    //     assert_eq!(solution, 29);
    // }
}
