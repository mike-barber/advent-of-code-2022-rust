use itertools::Itertools;
use nalgebra::Vector3;

type Pos = Vector3<i32>;

fn parse_input(input: &str) -> Vec<Pos> {
    input
        .lines()
        .map(|l| {
            let a = l.split(',').map(str::parse::<i32>).flatten();
            Vector3::from_iterator(a)
        })
        .collect()
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::*;

    const TEST_INPUT: &str = indoc! {"
        2,2,2
        1,2,2
        3,2,2
        2,1,2
        2,3,2
        2,2,1
        2,2,3
        2,2,4
        2,2,6
        1,2,5
        3,2,5
        2,1,5
        2,3,5
    "};

    #[test]
    fn parse_input_correct() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(input.len(), 13);
    }
}
