use std::{fs::File, io::Read, iter};

use regex::Regex;

use Term::*;

#[derive(Debug, Clone, PartialEq)]
enum Term {
    CdRoot,
    CdUp,
    CdInto(String),
    Ls,
    Entry(Entry),
}

#[derive(Debug, Clone, PartialEq)]
enum Entry {
    Dir(String),
    File(String, usize),
}

#[derive(Debug, Clone, PartialEq)]
struct Dir {
    name: String,
    files: Vec<(String, usize)>,
    sub_dirs: Vec<Dir>,
}

impl Dir {
    fn new(name: String) -> Self {
        Dir {
            name,
            files: Vec::new(),
            sub_dirs: Vec::new(),
        }
    }

    fn push_file(&mut self, name: String, size: usize) {
        self.files.push((name, size));
    }

    fn push_dir(&mut self, dir: Dir) -> &mut Dir {
        self.sub_dirs.push(dir);
        self.sub_dirs.last_mut().unwrap()
    }

    fn size_inclusive(&self) -> usize {
        let sub_dir_size: usize = self.sub_dirs.iter().map(|s| s.size_inclusive()).sum();
        let self_size: usize = self.files.iter().map(|f| f.1).sum();
        sub_dir_size + self_size
    }

    // fn size_self(&self) -> usize {
    //     self.files.iter().map(|f| f.1).sum()
    // }
}

fn read_file(file_name: &str) -> String {
    let mut contents = String::new();
    File::open(file_name)
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();
    contents
}

fn parse_input(input: &str) -> Vec<Term> {
    let re_cd = Regex::new(r#"\$ cd (/|\.\.|\w+)"#).unwrap();
    let re_ls = Regex::new(r#"\$ ls"#).unwrap();
    let re_dir = Regex::new(r#"dir (\w+)"#).unwrap();
    let re_file = Regex::new(r#"(\d+) (\w+)"#).unwrap();

    input
        .lines()
        .map(|line| {
            if let Some(caps) = re_cd.captures(line) {
                let arg = &caps[1];
                match arg {
                    "/" => CdRoot,
                    ".." => CdUp,
                    s => CdInto(s.into()),
                }
            } else if re_ls.is_match(line) {
                Ls
            } else if let Some(caps) = re_dir.captures(line) {
                Term::Entry(Entry::Dir(caps[1].into()))
            } else if let Some(caps) = re_file.captures(line) {
                Term::Entry(Entry::File(caps[2].into(), caps[1].parse().unwrap()))
            } else {
                panic!("cannot parse: {line}")
            }
        })
        .collect()
}

fn explore_dir(terminal_iter: &mut impl Iterator<Item = Term>, dir_name: String) -> Dir {
    let mut dir = Dir::new(dir_name);
    while let Some(term) = terminal_iter.next() {
        match term {
            CdUp => return dir,
            CdInto(d) => {
                let sub_dir = explore_dir(terminal_iter, d);
                dir.push_dir(sub_dir);
            }
            Ls => {}
            Term::Entry(Entry::File(name, size)) => dir.push_file(name, size),
            Term::Entry(Entry::Dir(_)) => {}
            CdRoot => panic!("cd to root not supported"),
        }
    }
    dir
}

fn part1(inputs: impl Iterator<Item = Term>) -> usize {
    let mut input_iter = inputs.into_iter();
    assert_eq!(input_iter.next(), Some(CdRoot));

    let root_dir = explore_dir(&mut input_iter, "/".into());

    fn add_sizes_under_at_most(dir: &Dir, max_size: usize) -> usize {
        let mut acc = 0;
        let size = dir.size_inclusive();
        if size <= max_size {
            acc += size;
        }
        for sub in dir.sub_dirs.iter() {
            acc += add_sizes_under_at_most(sub, max_size);
        }
        acc
    }

    add_sizes_under_at_most(&root_dir, 100_000)
}

fn part2(inputs: impl Iterator<Item = Term>) -> usize {
    let mut input_iter = inputs.into_iter();
    assert_eq!(input_iter.next(), Some(CdRoot));

    let root_dir = explore_dir(&mut input_iter, "/".into());

    let required_free_space = 30000000;
    let current_free_space = 70000000 - root_dir.size_inclusive();
    let minimum_amount_to_free = required_free_space - current_free_space;
    //let minimum_amount_to_free = 10;
    dbg!(minimum_amount_to_free);

    fn min_size_larger_than(dir: &Dir, minimum_amount_to_free: usize) -> Option<usize> {
        let min_size = dir.sub_dirs.iter()
            .filter_map(|sd| min_size_larger_than(sd, minimum_amount_to_free))
            .min();
    
        let own_size = dir.size_inclusive();
        let own_acceptable = if own_size >= minimum_amount_to_free {
            Some(own_size)
        } else {
            None
        };

        dbg!(min_size);
        dbg!(own_size);

        let res = match (min_size, own_acceptable) {
            (None, None) => None,
            (None, Some(y)) => Some(y),
            (Some(x), None) => Some(x),
            (Some(x), Some(y)) => Some(x.min(y)),
        };

        dbg!(res);
        res
    }

    min_size_larger_than(&root_dir, minimum_amount_to_free).unwrap()
}

fn main() {
    let entries = parse_input(&read_file("input.txt"));
    //println!("{entries:#?}");

    let part1_res = part1(entries.iter().cloned());
    println!("part 1 result = {part1_res}");

    let part2_res = part2(entries.iter().cloned());
    println!("part 2 result = {part2_res}");
}

#[cfg(test)]
mod tests {
    use crate::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
    $ cd /
    $ ls
    dir a
    14848514 b.txt
    8504156 c.dat
    dir d
    $ cd a
    $ ls
    dir e
    29116 f
    2557 g
    62596 h.lst
    $ cd e
    $ ls
    584 i
    $ cd ..
    $ cd ..
    $ cd d
    $ ls
    4060174 j
    8033020 d.log
    5626152 d.ext
    7214296 k
    "};

    #[test]
    fn parse_inputs_succeeds() {
        parse_input(TEST_INPUT);
    }

    #[test]
    fn part1_correct() {
        let inputs = parse_input(TEST_INPUT);
        assert_eq!(part1(inputs.into_iter()), 95437);
    }

    #[test]
    fn part2_correct() {
        let inputs = parse_input(TEST_INPUT);
        assert_eq!(part2(inputs.into_iter()), 24933642);
    }
}
