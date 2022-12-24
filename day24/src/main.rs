use common::*;
use day24::{parse_input, part1, part2, ProblemState};

fn main() -> AnyResult<()> {
    let input = read_file("day24/input.txt")?;

    let problem = parse_input(&input)?;

    for t in 0..3 {
        let state = ProblemState::with_time(&problem, t);
        println!("time = {t}");
        println!("{state}");
    }

    let init = ProblemState::with_time(&problem, 0);
    for cycle in 0..3 {
        let t = cycle * problem.cycle_length();
        let state = ProblemState::with_time(&problem, t);
        println!("cycle {cycle} time = {t}");
        println!("{state}");

        assert_eq!(init.to_string(), state.to_string());
    }

    let part1_result = part1::find_shortest_path(&problem);
    println!("part1 result: {part1_result:?}");

    let part2_result = part2::find_shortest_path(&problem);
    println!("part2 result: {part2_result:?}");

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
    }

    #[test]
    fn run_cycles_basic() {
        let problem = parse_input(TEST_INPUT_BASIC).unwrap();
        println!("cycles: {}", problem.cycle_length());
        for t in 0..6 {
            let state = ProblemState::with_time(&problem, t);
            println!("time {t}");
            println!("{state}");
            println!();
        }
    }

    fn test_cycles(input: &str) {
        let problem = parse_input(input).unwrap();

        let init = ProblemState::with_time(&problem, 0);
        let init_str = init.to_string();

        // assert that we keep cycling successfully
        for c in 0..5 {
            let state = ProblemState::with_time(&problem, c * problem.cycle_length());
            let state_str = state.to_string();
            assert_eq!(init_str, state_str);
        }
    }

    #[test]
    fn run_cycles_simple() {
        test_cycles(TEST_INPUT_COMPLEX);
    }

    #[test]
    fn run_cycles_complex() {
        test_cycles(TEST_INPUT_COMPLEX);
    }

    #[test]
    fn part1_find_shortest_path_correct() {
        let problem = parse_input(TEST_INPUT_COMPLEX).unwrap();
        let minimum = part1::find_shortest_path(&problem);
        assert_eq!(minimum, Some(18));
    }

    #[test]
    fn part2_find_shortest_path_correct() {
        let problem = parse_input(TEST_INPUT_COMPLEX).unwrap();
        let minimum = part2::find_shortest_path(&problem);
        assert_eq!(minimum, Some(54));
    }
}
