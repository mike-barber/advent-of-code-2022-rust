use std::{cmp::Ordering, ops::Rem};

use common::{read_file, AnyResult};
use itertools::Itertools;

const PART2_KEY: i64 = 811589153;

fn parse_input(input: &str) -> AnyResult<Vec<i64>> {
    let vals: Result<Vec<_>, _> = input.lines().map(str::parse::<i64>).collect();
    Ok(vals?)
}

fn mix_once(array: &[i64]) -> Vec<i64> {
    let moves: Vec<_> = array.iter().copied().enumerate().collect();
    let mut array: Vec<_> = array.iter().copied().enumerate().collect();

    fn format_array(arr: &[(usize, i64)]) -> String {
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
            (current_index as i64 + m).rem(array.len() as i64)
        } else {
            (current_index as i64 + m - 1).rem_euclid(array.len() as i64)
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

fn calculate_permutations_old(moves: &[i64]) -> Vec<usize> {
    let mut positions: Vec<usize> = (0..moves.len()).collect();
    println!("{:?}", permute(moves, &positions));
    for (idx, mv) in moves.iter().enumerate() {
        let curr_index = positions.iter().position(|&p| p == idx).unwrap();
        let dest_index = (curr_index as i64 + mv).rem_euclid(moves.len() as i64) as usize;

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

fn calculate_permutations(moves: &[i64]) -> Vec<usize> {
    let len = moves.len();
    let mut positions: Vec<usize> = (0..moves.len()).collect();
    // println!("{:?}", permute(moves, &positions));
    for (idx, mv) in moves.iter().enumerate() {
        let curr_index = positions.iter().position(|&p| p == idx).unwrap();

        // println!("mv {mv}");
        if *mv > 0 {
            for i in 0..*mv {
                let a = (curr_index as i64 + i).rem_euclid(len as i64) as usize;
                let b = (curr_index as i64 + i + 1).rem_euclid(len as i64) as usize;
                positions.swap(a, b);
                // println!("  {i} {:?}", permute(moves, &positions));
            }
        }

        if *mv < 0 {
            for i in (*mv..0).rev() {
                let a = (curr_index as i64 + i).rem_euclid(len as i64) as usize;
                let b = (curr_index as i64 + i + 1).rem_euclid(len as i64) as usize;
                positions.swap(a, b);
                // println!("  {i} {:?}", permute(moves, &positions));
            }
        }

        // println!("{:?}", permute(moves, &positions));
    }
    positions
}

fn calculate_permutations_fast_old(moves: &[i64], echo: bool) -> Vec<usize> {
    let len = moves.len();
    let mut positions: Vec<usize> = (0..moves.len()).collect();

    if echo {
        println!(
            "indices {:?} arr {:?}",
            positions,
            permute(moves, &positions)
        );
    }
    for (orig_idx, mv) in moves.iter().enumerate() {
        if echo {
            println!();
            if *mv == 0 {
                println!("skipped mv==0");
                // no move for zero
                continue;
            }
        }

        let curr_idx = positions.iter().position(|&p| p == orig_idx).unwrap();

        let tgt_index = match mv {
            mv if *mv > 0 => (curr_idx as i64 + *mv + 1).rem_euclid(len as i64) as usize,
            mv if *mv < 0 => (curr_idx as i64 + *mv).rem_euclid(len  as i64) as usize,
            _ => curr_idx,
        };
        let tgt_pos = positions[tgt_index];

        // skip if we're placing back at the same place
        if tgt_pos == orig_idx {
            if echo {
                println!("skipping place back in same location");
            }
            continue;
        }

        if echo {
            println!(
                "move by {mv}: [ix={curr_idx} = {}] --> [before ix={tgt_pos} = {}]",
                moves[positions[curr_idx]], moves[tgt_pos]
            );
        }

        // not efficient
        positions.remove(curr_idx);

        if echo {
            println!(
                "  indices {:?} arr {:?}",
                positions,
                permute(moves, &positions)
            );
        }

        let insert_idx = positions.iter().position(|&p| p == tgt_pos).unwrap();
        positions.insert(insert_idx, orig_idx);

        if echo {
            println!(
                "  indices {:?} arr {:?}",
                positions,
                permute(moves, &positions)
            );
        }
    }
    positions
}

#[derive(Debug,Clone,Copy)]
struct Pos(usize);

fn calculate_permutations_fast(moves: &[i64], echo: bool) -> Vec<usize> {
    let len = moves.len();
    let mut positions: Vec<usize> = (0..moves.len()).collect();

    if echo {
        println!(
            "indices {:?} arr {:?}",
            positions,
            permute(moves, &positions)
        );
    }
    for (curr_pos, mv) in moves.iter().enumerate() {
        if *mv == 0 {
            continue;
        }
        
        // find index for current position
        let curr_idx = positions.iter().position(|p| *p==curr_pos).unwrap();

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
        if echo {
            println!("insert at {insert_idx} that has position {}", positions[insert_idx]);
        }
        positions.insert(insert_idx, curr_pos);
        if echo {
            println!(
                "indices {:?} arr {:?}",
                positions,
                permute(moves, &positions)
            );
        }
    }
    positions
}

fn permute(array: &[i64], positions: &[usize]) -> Vec<i64> {
    positions.iter().map(|ix| array[*ix]).collect()
}

fn mix_once_permute(array: &[i64]) -> Vec<i64> {
    let positions = calculate_permutations(array);
    permute(&array, &positions)
}

fn part1(array: &[i64]) -> i64 {
    //let mixed = mix_once(array);
    let mixed = mix_once_permute(array);

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

fn part1_alt(array: &[i64]) -> i64 {
    //let mixed = mix_once(array);
    //let mixed = mix_once_permute(array);
    let positions = calculate_permutations_fast(array, false);
    let mixed = permute(array, &positions);

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

fn main() -> AnyResult<()> {
    let input = parse_input(&read_file("day20/input.txt")?)?;

    println!("expecting: 13289");
    println!("part1 result = {}", part1(&input));
    println!("part1_alt result = {}", part1_alt(&input));

    scratch();

    Ok(())
}

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

fn scratch() {
    let mut moves = [0; 6];
    println!("positive moves --------");
    for m in 0..=6 * 6 {
        moves[2] = m;
        let perm = calculate_permutations(&moves);
        let alt = calculate_permutations_fast(&moves, false);
        println!("{m}: {perm:?} {alt:?} {}", check_eq_rotate(&perm, &alt));
    }
    println!("negative moves --------");
    for m in 0..=6 * 6 {
        moves[2] = -m;
        let perm = calculate_permutations(&moves);
        let alt = calculate_permutations_fast(&moves, false);
        println!("{m}: {perm:?} {alt:?} {}", check_eq_rotate(&perm, &alt));
    }

    // let example = [1, 2, -3, 3, -2, 0, 4];
    // calculate_permutations_fast(&example, true);
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
        assert_eq!(res, [1, 2, -3, 4, 0, 3, -2]);
    }

    #[test]
    fn permute_mix_correct() {
        let input = parse_input(TEST_INPUT).unwrap();
        let res = mix_once_permute(&input);
        assert_eq_rotate(&res, &[1, 2, -3, 4, 0, 3, -2]);
    }

    #[test]
    fn calculate_permutations_fast_correct() {
        let input = parse_input(TEST_INPUT).unwrap();
        let positions = calculate_permutations_fast(&input, true);
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
    fn part2_correct() {
        let mut input = parse_input(TEST_INPUT).unwrap();
        input.iter_mut().for_each(|v| *v *= PART2_KEY);
    }
}
