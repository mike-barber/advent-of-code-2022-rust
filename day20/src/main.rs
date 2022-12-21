use std::{cmp::Ordering, ops::Rem, thread::current};

use common::{read_file, AnyResult};
use itertools::Itertools;

fn parse_input(input: &str) -> AnyResult<Vec<i32>> {
    let vals: Result<Vec<_>, _> = input.lines().map(str::parse::<i32>).collect();
    Ok(vals?)
}

fn mix_once(array: &[i32]) -> Vec<i32> {
    let moves: Vec<_> = array.iter().copied().enumerate().collect();
    let mut array: Vec<_> = array.iter().copied().enumerate().collect();

    fn format_array(arr: &[(usize, i32)]) -> String {
        let mut s = "[".to_string();
        s.push_str(&arr.iter().map(|(_, v)| v.to_string()).join(" "));
        s.push(']');
        s
    }

    println!("init: {array:?}");

    for (original_index, m) in moves {
        // println!();
        // println!("{}: move idx {original_index} by {m}", format_array(&array));

        let current_index = array
            .iter()
            .position(|(i, _)| *i == original_index)
            .unwrap();

        let target_index = if m >= 0 {
            (current_index as i32 + m).rem(array.len() as i32)
        } else {
            (current_index as i32 + m - 1).rem_euclid(array.len() as i32)
        };
        let target_index = target_index as usize;

        if target_index > current_index {
            let range = current_index..=target_index;
            let slice = &mut array[range.clone()];
            // println!("  1 before {}", format_array(slice));
            slice.rotate_left(1);
            // println!("  1 after  {}", format_array(slice));
        }

        if current_index > target_index {
            let slice = &mut array[target_index + 1..=current_index];
            // println!("  2 before {}", format_array(slice));
            slice.rotate_right(1);
            // println!("  2 after  {}", format_array(slice));
        }

        // println!(
        //     "{}: moved {m} tgt {target_index} curr {current_index}",
        //     format_array(&array)
        // );
    }

    array.iter().map(|(_, v)| v).copied().collect()
}

fn calculate_permutations_old(moves: &[i32]) -> Vec<usize> {
    let mut positions: Vec<usize> = (0..moves.len()).collect();
    println!("{:?}", permute(moves, &positions));
    for (idx, mv) in moves.iter().enumerate() {
        let curr_index = positions.iter().position(|&p| p == idx).unwrap();
        let dest_index = (curr_index as i32 + mv).rem_euclid(moves.len() as i32) as usize;

        println!("move {mv}: {curr_index}->{dest_index}");
        let left = curr_index.min(dest_index);
        let right = curr_index.max(dest_index);

        match dest_index.cmp(&curr_index) {
            Ordering::Less => {
                let slice = &mut positions[left..=right];
                println!("{:?}", permute(moves, slice));
                slice.rotate_right(1);
                println!("{:?}", permute(moves, slice));
            }
            Ordering::Equal => {}
            Ordering::Greater => {
                let slice = &mut positions[left..=right];
                println!("{:?}", permute(moves, slice));
                slice.rotate_left(1);
                println!("{:?}", permute(moves, slice));
            }
        }

        println!("{:?}", permute(moves, &positions));
    }
    positions
}

fn calculate_permutations(moves: &[i32]) -> Vec<usize> {
    let len = moves.len();
    let mut positions: Vec<usize> = (0..moves.len()).collect();
    // println!("{:?}", permute(moves, &positions));
    for (idx, mv) in moves.iter().enumerate() {
        let curr_index = positions.iter().position(|&p| p == idx).unwrap();

        // println!("mv {mv}");
        if *mv > 0 {
            for i in 0..*mv {
                let a = (curr_index as i32 + i).rem_euclid(len as i32) as usize;
                let b = (curr_index as i32 + i + 1).rem_euclid(len as i32) as usize;
                positions.swap(a, b);
                // println!("  {i} {:?}", permute(moves, &positions));
            }
        }

        if *mv < 0 {
            for i in (*mv..0).rev() {
                let a = (curr_index as i32 + i).rem_euclid(len as i32) as usize;
                let b = (curr_index as i32 + i + 1).rem_euclid(len as i32) as usize;
                positions.swap(a, b);
                // println!("  {i} {:?}", permute(moves, &positions));
            }
            if curr_index as i32 + mv < 0 {
                positions.rotate_left(1);
            }
        }

        // println!("{:?}", permute(moves, &positions));
    }
    positions
}

fn permute(array: &[i32], positions: &[usize]) -> Vec<i32> {
    positions.iter().map(|ix| array[*ix]).collect()
}

fn mix_once_permute(array: &[i32]) -> Vec<i32> {
    let positions = calculate_permutations(array);
    permute(&array, &positions)
}

fn part1(array: &[i32]) -> i32 {
    //let mixed = mix_once(array);
    let mixed = mix_once_permute(array);

    let pos_zero = mixed.iter().position(|a| *a == 0).unwrap();

    let mut sum = 0;
    for idx in [1000, 2000, 3000] {
        let val = mixed.iter().cycle().skip(pos_zero).nth(idx).unwrap();
        dbg!(val);
        sum += val;
    }

    sum
}

fn main() -> AnyResult<()> {
    let input = parse_input(&read_file("day20/input.txt")?)?;

    println!("note: -2444 is not correct");
    println!("part1 result = {}", part1(&input));

    scratch();

    Ok(())
}

fn scratch() {
    let mut v = vec![0, 1, 2];
    println!("{v:?}");
    //v.rotate_left(1);
    v.rotate_right(1);
    println!("{v:?}");
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
    fn permute_positions_correct() {
        // no moves
        assert_eq!(
            calculate_permutations(&[0, 0, 0, 0, 0, 0]),
            [0, 1, 2, 3, 4, 5]
        );
        assert_eq!(
            calculate_permutations(&[0, 0, 6, 0, 0, 0]),
            [0, 1, 2, 3, 4, 5]
        );
        assert_eq!(
            calculate_permutations(&[0, 0, -6, 0, 0, 0]),
            [0, 1, 2, 3, 4, 5]
        );
        // negative moves
        assert_eq!(
            calculate_permutations(&[0, 0, -1, 0, 0, 0]),
            [0, 2, 1, 3, 4, 5]
        );
        assert_eq!(
            calculate_permutations(&[0, 0, -2, 0, 0, 0]),
            [2, 0, 1, 3, 4, 5]
        );
        assert_eq!(
            calculate_permutations(&[0, 0, -3, 0, 0, 0]),
            [0, 1, 3, 4, 5, 2]
        );
        assert_eq!(
            calculate_permutations(&[0, 0, -4, 0, 0, 0]),
            [0, 1, 3, 4, 2, 5]
        );
        assert_eq!(
            calculate_permutations(&[0, 0, -5, 0, 0, 0]),
            [0, 1, 3, 2, 4, 5]
        );
        // positive moves
        assert_eq!(
            calculate_permutations(&[0, 0, 1, 0, 0, 0]),
            [0, 1, 3, 2, 4, 5]
        );
        assert_eq!(
            calculate_permutations(&[0, 0, 2, 0, 0, 0]),
            [0, 1, 3, 4, 2, 5]
        );
        assert_eq!(
            calculate_permutations(&[0, 0, 3, 0, 0, 0]),
            [0, 1, 3, 4, 5, 2]
        );
        assert_eq!(
            calculate_permutations(&[0, 0, 4, 0, 0, 0]),
            [2, 0, 1, 3, 4, 5]
        );
        assert_eq!(
            calculate_permutations(&[0, 0, 5, 0, 0, 0]),
            [0, 2, 1, 3, 4, 5]
        );
    }

    #[test]
    fn parse_input_correct() {
        parse_input(TEST_INPUT).unwrap();
    }

    #[test]
    fn basic_mix_correct() {
        let input = parse_input(TEST_INPUT).unwrap();
        let res = mix_once(&input);
        assert_eq!(res, [1, 2, -3, 4, 0, 3, -2]);
    }

    #[test]
    fn permute_mix_correct() {
        let input = parse_input(TEST_INPUT).unwrap();
        let res = mix_once_permute(&input);
        assert_eq!(res, [1, 2, -3, 4, 0, 3, -2]);
    }

    #[test]
    fn part1_correct() {
        let input = parse_input(TEST_INPUT).unwrap();
        let res = part1(&input);
        assert_eq!(res, 3);
    }
}
