use common::{read_file, AnyResult};

use day22::*;

fn main() -> AnyResult<()> {
    let input = part1::parse_input(&read_file("day22/input.txt")?)?;

    println!("part1 result: {}", part1::run(&input));

    Ok(())
}
