use std::{cmp::Ordering, ops::Rem};

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

            // this only seems to be required to make the test pass; we still get the right
            // answer for part1 without it.
            if curr_index as i32 + mv < 0 {
                positions.rotate_left(1);
            }
        }

        // println!("{:?}", permute(moves, &positions));
    }
    positions
}

fn calculate_permutations_fast(moves: &[i32]) -> Vec<usize> {
    let len = moves.len();
    let mut positions: Vec<usize> = (0..moves.len()).collect();

    println!(
        "indices {:?} arr {:?}",
        positions,
        permute(moves, &positions)
    );
    for (orig_idx, mv) in moves.iter().enumerate() {
        println!();
        if *mv == 0 {
            println!("skipped mv==0");
            // no move for zero
            continue;
        }

        let curr_idx = positions.iter().position(|&p| p == orig_idx).unwrap();

        let tgt_index = match mv {
            mv if *mv > 0 => (curr_idx as i32 + *mv + 1).rem_euclid(len as i32) as usize,
            mv if *mv < 0 => (curr_idx as i32 + *mv).rem_euclid(len as i32) as usize,
            _ => curr_idx,
        };
        let tgt_pos = positions[tgt_index];

        println!(
            "move by {mv}: [ix={curr_idx} = {}] --> [before ix={tgt_pos} = {}]",
            moves[positions[curr_idx]], moves[tgt_pos]
        );

        // not efficient
        positions.remove(curr_idx);

        println!(
            "  indices {:?} arr {:?}",
            positions,
            permute(moves, &positions)
        );

        let insert_idx = positions.iter().position(|&p| p == tgt_pos).unwrap();
        if insert_idx == 0 {
            // if it's at the start, add to the _end_ as per the rules
            positions.push(orig_idx);
        } else {
            // otherwise insert before the designated element
            positions.insert(insert_idx, orig_idx);
        }

        println!(
            "  indices {:?} arr {:?}",
            positions,
            permute(moves, &positions)
        );
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

fn part1_alt(array: &[i32]) -> i32 {
    //let mixed = mix_once(array);
    //let mixed = mix_once_permute(array);
    let positions = calculate_permutations_fast(array);
    let mixed = permute(array, &positions);

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

fn scratch() {
    let mut moves = [0; 6];
    println!("positive moves --------");
    for m in 0..=6 * 6 {
        moves[2] = m;
        let perm = calculate_permutations(&moves);
        println!("{m}: {perm:?}");
    }
    println!("negative moves --------");
    for m in 0..=6 * 6 {
        moves[2] = -m;
        let perm = calculate_permutations(&moves);
        println!("{m}: {perm:?}");
    }

    let example = [1, 2, -3, 3, -2, 0, 4];
    calculate_permutations_fast(&example);
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
        assert_eq!(res, [1, 2, -3, 4, 0, 3, -2]);
    }

    #[test]
    fn calculate_permutations_fast_correct() {
        let input = parse_input(TEST_INPUT).unwrap();
        let positions = calculate_permutations_fast(&input);
        let res = permute(&input, &positions);
        assert_eq!(res, [1, 2, -3, 4, 0, 3, -2]);
    }

    #[test]
    fn part1_correct() {
        let input = parse_input(TEST_INPUT).unwrap();
        let res = part1(&input);
        assert_eq!(res, 3);
    }
}
