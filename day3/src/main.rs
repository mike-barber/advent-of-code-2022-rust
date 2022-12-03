use anyhow::{anyhow, bail};
use std::{fs::File, io::Read};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Hash)]
struct Item(char);

impl TryFrom<char> for Item {
    type Error = anyhow::Error;

    fn try_from(value: char) -> anyhow::Result<Item> {
        match value {
            'a'..='z' => Ok(Item(value)),
            'A'..='Z' => Ok(Item(value)),
            _ => bail!("invalid item type"),
        }
    }
}

impl Item {
    pub fn inner(&self) -> char {
        self.0
    }

    pub fn priority(&self) -> i32 {
        match self.inner() {
            'a'..='z' => 1 + (self.inner() as i32 - 'a' as i32),
            'A'..='Z' => 27 + (self.inner() as i32 - 'A' as i32),
            _ => {
                panic!("invalid char - this invariant should have been ensured by the constructor")
            }
        }
    }
}

fn split_compartments(rucksack: &str) -> (&str, &str) {
    let len = rucksack.len();
    let middle = len / 2;
    rucksack.split_at(middle)
}

fn parse_items(items_str: &str) -> anyhow::Result<Vec<Item>> {
    items_str.chars().map(Item::try_from).collect()
}

fn common_item(comp1: &[Item], comp2: &[Item]) -> Option<Item> {
    // just do the dumb linear search for N^2 complexity
    for i1 in comp1.iter().copied() {
        for i2 in comp2.iter().copied() {
            if i1 == i2 {
                return Some(i1);
            }
        }
    }

    // no common items
    None
}

fn part1(input: &str) -> anyhow::Result<i32> {
    let rucksacks = input.lines();

    let mut common_items = Vec::new();
    for r in rucksacks {
        let (c1, c2) = split_compartments(r);

        let items1 = parse_items(c1)?;
        let items2 = parse_items(c2)?;
        let found_common =
            common_item(&items1, &items2).ok_or_else(|| anyhow!("common item not found"))?;

        common_items.push(found_common);
    }

    let sum: i32 = common_items.iter().map(Item::priority).sum();
    Ok(sum)
}

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    File::open("input1.txt")?.read_to_string(&mut input)?;

    println!("Part 1 result: {:?}", part1(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw
    "};

    const RUCKSACK: &str = "vJrwpWtwJgWrhcsFMMfFFhFp";
    const COMPARTMENT1: &str = "vJrwpWtwJgWr";
    const COMPARTMENT2: &str = "hcsFMMfFFhFp";

    #[test]
    fn correct_priority() {
        assert_eq!(1, Item::try_from('a').unwrap().priority());
        assert_eq!(26, Item::try_from('z').unwrap().priority());
        assert_eq!(27, Item::try_from('A').unwrap().priority());
        assert_eq!(52, Item::try_from('Z').unwrap().priority());
    }

    #[test]
    fn split_correct() {
        let (comp1, comp2) = split_compartments(RUCKSACK);
        assert_eq!(comp1, COMPARTMENT1);
        assert_eq!(comp2, COMPARTMENT2)
    }

    #[test]
    fn common_item_correct() {
        let comp1 = parse_items(COMPARTMENT1).unwrap();
        let comp2 = parse_items(COMPARTMENT2).unwrap();

        let found_common = common_item(&comp1, &comp2);
        let expected = Some(Item::try_from('p').unwrap());
        assert_eq!(found_common, expected);
    }

    #[test]
    fn part1_correct() {
        let value = part1(TEST_INPUT).unwrap();
        assert_eq!(value, 157);
    }
}
