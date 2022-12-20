use std::collections::VecDeque;

use common::AnyResult;
use itertools::Itertools;

fn parse_input(input: &str) -> AnyResult<Vec<i32>> {
    let vals: Result<Vec<_>,_> = input.lines().map(str::parse::<i32>).collect();
    Ok(vals?)
}

fn mix_once(array: &[i32]) -> Vec<i32> {
    let moves: Vec<_> = array.iter().copied().collect();
    let mut array: Vec<i32> = array.iter().copied().collect();
    
    println!("init: {array:?}");

    let mut next_index = 0_usize;
    for m in moves {
        let target_index = (next_index as i32 + m) as usize % array.len();
        
        if target_index > next_index {
            let range = next_index..=target_index;
            let slice  = &mut array[range.clone()];
            println!("  1 before {slice:?}");
            slice.rotate_left(1);
            next_index += 1;
            if range.contains(&next_index) {
                next_index -= 1;
            }
            println!("  1 after {slice:?}");
        }
        
        if next_index > target_index {
            let slice  = &mut array[target_index..=next_index];
            println!("  2 before {slice:?}");
            slice.rotate_right(1);
            next_index += 2;
            println!("  2 after {slice:?}");
        }

        println!("move {m} tgt {target_index} next {next_index} array {array:?}");
    }

    array
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

    #[test]
    fn basic_mix_correct() {
        let input = parse_input(TEST_INPUT).unwrap();
        let res = mix_once(&input);
        dbg!(res);
    }
}
