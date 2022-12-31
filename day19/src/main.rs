use std::time::Instant;

use common::*;
use day19::{parser::parse_input, part1, part2};

fn main() -> AnyResult<()> {
    let blueprints = parse_input(&read_file("day19/input.txt")?)?;

    let t1 = Instant::now();
    println!("note: this will take a while to run especially in debug mode; we expect 1834");
    println!("part1 result = {}", part1(&blueprints));
    println!("complete in {:#?}", Instant::now() - t1);

    let t2 = Instant::now();
    println!();
    println!("note: this will take a while to run especially in debug mode; we expect 2240");
    println!("part2 result = {}", part2(&blueprints));
    println!("complete in {:#?}", Instant::now() - t2);

    Ok(())
}
