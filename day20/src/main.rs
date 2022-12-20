use common::AnyResult;
use itertools::Itertools;


fn parse_input(input: &str) -> AnyResult<Vec<i32>> {
    let vals: Result<Vec<_>,_> = input.lines().map(str::parse::<i32>).collect();
    Ok(vals?)
}


fn main() {
    
}


#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        1
        2
        -3
        3
        -2
        0
        4
    "};

    #[test]
    fn parse_input_correct() {
        parse_input(TEST_INPUT).unwrap();
    }
}
