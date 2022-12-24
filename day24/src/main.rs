use std::fmt::Display;

use common::*;

#[derive(Debug,Clone)]
struct Problem{

}
impl Display for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}



fn parse_input(input: &str) -> AnyResult<Problem> {


    todo!()
}


fn main() -> AnyResult<()> {
    let input = read_file("src/day24.txt")?;


    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT_BASIC: &str = indoc! {"
        #.#####
        #.....#
        #>....#
        #.....#
        #...v.#
        #.....#
        #####.#
    "};

    const TEST_INPUT_COMPLEX: &str = indoc! {"
        #.######
        #>>.<^<#
        #.<..<<#
        #>v.><>#
        #<^v^^>#
        ######.#
    "};

    #[test]
    fn parse_input_correct() {
        let problem = parse_input(TEST_INPUT_COMPLEX).unwrap();
        dbg!(&problem);
        println!("{}", problem);
    }

    // #[test]
    // fn step_once_check_small() {
    //     let mut problem = parse_input(TEST_INPUT_SMALL).unwrap();
    //     println!("{problem}");
    //     for i in 1..=3 {
    //         let count = problem.step_once();
    //         println!("i: {i}, moved: {count}");
    //         println!("{problem}");
    //     }
    //     let expected = indoc! {"
    //     "};
    //     assert_eq!(problem.to_string(), expected);
    // }

    // #[test]
    // fn step_once_check_larger() {
    //     let mut problem = parse_input(TEST_INPUT).unwrap();
    //     println!("{problem}");
    //     for i in 1..=10 {
    //         let count = problem.step_once();
    //         println!("i: {i}, moved: {count}");
    //         println!("{problem}");
    //     }

    //     let expected = indoc! {"
    //     "};
    //     assert_eq!(problem.to_string(), expected);
    //     assert_eq!(problem.count_empty_blocks(), 110);
    // }

    // #[test]
    // fn part2_correct() {
    //     let res = part2(TEST_INPUT).unwrap();
    //     assert_eq!(res, 20);
    // }
}
