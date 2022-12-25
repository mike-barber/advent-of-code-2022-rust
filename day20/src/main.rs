use std::ops::Rem;

use common::{read_file, AnyResult};
use itertools::Itertools;

const PART2_KEY: i64 = 811589153;

fn parse_input(input: &str) -> AnyResult<Vec<i64>> {
    let vals: Result<Vec<_>, _> = input.lines().map(str::parse::<i64>).collect();
    Ok(vals?)
}

fn positions_for<T>(moves: &[T]) -> Vec<usize> {
    (0..moves.len()).collect()
}

fn calculate_permutations(positions: &mut Vec<usize>, moves: &[i64]) {
    let len = moves.len();
    for (curr_pos, mv) in moves.iter().enumerate() {
        // skip no-move cases
        if *mv == 0 {
            continue;
        }

        // find index for current position
        let curr_idx = positions.iter().position(|p| *p == curr_pos).unwrap();

        // next element index
        let mut next_idx = (curr_idx + 1).rem(len);

        // remove the current element
        positions.remove(curr_idx);

        // bump next idx left if it's to the right of the removed index
        if next_idx >= curr_idx {
            next_idx -= 1;
        }

        // find insert location in the array without the original element
        let insert_idx = (next_idx as i64 + *mv).rem_euclid(positions.len() as i64) as usize;
        positions.insert(insert_idx, curr_pos);
    }
}

fn permute(array: &[i64], positions: &[usize]) -> Vec<i64> {
    positions.iter().map(|ix| array[*ix]).collect()
}

fn calculate_result(mixed: Vec<i64>) -> i64 {
    // note: position of zero is the start; it doesn't matter what overall
    // rotation the array has, as we're starting the cycle here.
    let pos_zero = mixed.iter().position(|a| *a == 0).unwrap();
    let mut sum = 0;
    for idx in [1000, 2000, 3000] {
        let val = mixed.iter().cycle().skip(pos_zero).nth(idx).unwrap();
        dbg!(val);
        sum += val;
    }
    sum
}

fn part1(array: &[i64]) -> i64 {
    let mut positions = positions_for(array);
    calculate_permutations(&mut positions, array);
    let mixed = permute(array, &positions);

    calculate_result(mixed)
}

fn part2(array: &[i64]) -> i64 {
    let moves = array.iter().map(|v| v * PART2_KEY).collect_vec();

    let mut positions = positions_for(&moves);
    for i in 1..=10 {
        println!("permutation iteration {i}");
        calculate_permutations(&mut positions, &moves);
    }

    let mixed = permute(&moves, &positions);

    calculate_result(mixed)
}

fn main() -> AnyResult<()> {
    let input = parse_input(&read_file("day20/input.txt")?)?;

    println!("part1 result = {}", part1(&input));

    println!();
    println!("part2 result = {}", part2(&input));

    Ok(())
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

    // Assert that arrays are equal for any rotation of the right
    // array; i.e. the elements need to be in the correct (cyclical) order,
    // but not necessarily starting at the same location.
    fn assert_eq_rotate<T>(left: &[T], right: &[T])
    where
        T: PartialEq + std::fmt::Debug + Clone,
    {
        if !check_eq_rotate(left, right) {
            assert_eq!(left, right);
        }
    }

    fn check_eq_rotate<T>(left: &[T], right: &[T]) -> bool
    where
        T: PartialEq + std::fmt::Debug + Clone,
    {
        let mut right = right.iter().cloned().collect_vec();
        for _ in 0..left.len() {
            if left == right.as_slice() {
                return true;
            }
            right.rotate_left(1);
        }
        return false;
    }

    #[test]
    fn parse_input_correct() {
        parse_input(TEST_INPUT).unwrap();
    }

    #[test]
    fn calculate_permutations_fast_correct() {
        let input = parse_input(TEST_INPUT).unwrap();
        let mut positions = positions_for(&input);
        calculate_permutations(&mut positions, &input);
        let res = permute(&input, &positions);
        assert_eq_rotate(&res, &[1, 2, -3, 4, 0, 3, -2]);
    }

    #[test]
    fn part1_correct() {
        let input = parse_input(TEST_INPUT).unwrap();
        let res = part1(&input);
        assert_eq!(res, 3);
    }

    #[test]
    fn part2_mix_correct() {
        let mut moves = parse_input(TEST_INPUT).unwrap();
        moves.iter_mut().for_each(|v| *v *= PART2_KEY);
        let mut positions = positions_for(&moves);
        for i in 1..=10 {
            calculate_permutations(&mut positions, &moves);
            let permuted = permute(&moves, &positions);
            println!("iteration {i} => {permuted:?}");
        }

        let permuted = permute(&moves, &positions);
        assert_eq_rotate(
            &permuted,
            &[
                0,
                -2434767459,
                1623178306,
                3246356612,
                -1623178306,
                2434767459,
                811589153,
            ],
        );
    }

    #[test]
    fn part2_correct() {
        let moves = parse_input(TEST_INPUT).unwrap();
        let res = part2(&moves);
        assert_eq!(res, 1623178306);
    }
}
