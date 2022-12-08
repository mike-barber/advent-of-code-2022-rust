use std::{fs::File, io::Read};

fn read_file(file_name: &str) -> String {
    let mut contents = String::new();
    File::open(file_name)
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();
    contents
}

// This is quite inefficient: O(N^2) on the size
// of the slice. The problem is so small that a
// more complex algorithm won't yield any benefit.
fn all_different(characters: &[char]) -> bool {
    let mut chr = characters;
    while let Some((ch, remainder)) = chr.split_first() {
        if remainder.iter().any(|r| r == ch) {
            return false;
        }
        chr = remainder;
    }
    true
}

fn find_marker(line: &str, window_size: usize) -> usize {
    let chars: Vec<_> = line.chars().collect();
    let (idx, _) = chars
        .windows(window_size)
        .enumerate()
        .find(|(_, w)| all_different(w))
        .expect("pattern not found");
    idx + window_size
}

fn part1(input: &str) -> usize {
    find_marker(input, 4)
}

fn part2(input: &str) -> usize {
    find_marker(input, 14)
}

fn main() {
    let contents = read_file("input1.txt");

    let part1_solution = part1(&contents);
    println!("day6 / part1: {part1_solution}");

    let part2_solution = part2(&contents);
    println!("day6 / part2: {part2_solution}");
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn part1_examples_correct() {
        assert_eq!(part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
    }

    #[test]
    fn part2_examples_correct() {
        assert_eq!(part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
    }
}
