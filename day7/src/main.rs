use std::{fs::File, io::Read, env::current_exe};

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
    sub_dirs: Vec<Dir>
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

    fn size(&self) -> usize {
        let sub_dir_size: usize = self.sub_dirs.iter().map(|s| s.size()).sum();
        let self_size: usize = self.files.iter().map(|f| f.1).sum();
        sub_dir_size + self_size
    }
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
       
    input.lines().map(|line| {
        if let Some(caps) = re_cd.captures(line) {
            let arg = &caps[1];
            match arg {
                "/" => CdRoot,
                ".." => CdUp,
                s => CdInto(s.into())
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
    }).collect()
}


fn explore_dir(terminal_iter: &mut impl Iterator<Item = Term>, dir_name: String) -> Dir {
    let mut dir = Dir::new(dir_name);
    while let Some(term) = terminal_iter.next() {
        match term {
            CdUp => return dir,
            CdInto(d) => {
                let sub_dir = explore_dir(terminal_iter, d);
                dir.push_dir(sub_dir);
            },
            Ls => {},
            Term::Entry(Entry::File(name, size)) => dir.push_file(name, size),
            Term::Entry(Entry::Dir(_)) => {},
            CdRoot => panic!("cd to root not supported"),
        }
    }
    dir
}


fn part1(inputs: impl Iterator<Item = Term>) -> usize {
    //let mut directory_sizes : Vec<(String, usize)> = Vec::new();
    let mut input_iter = inputs.into_iter();
    assert_eq!(input_iter.next(), Some(CdRoot));

    let root_node = explore_dir(&mut input_iter, "/".into());

    println!("{:#?}", root_node);

    0
}



fn main() {
    let entries = parse_input(&read_file("input.txt"));
    println!("{entries:#?}");
}


#[cfg(test)]
mod tests {
    use indoc::indoc;
    use crate::*;

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


}