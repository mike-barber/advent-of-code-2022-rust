use common::*;
use day19::{parser::parse_input, part1, part2};

fn main() -> AnyResult<()> {
    let blueprints = parse_input(&read_file("day19/input.txt")?)?;
        
    println!("note: this will take a while to run; we expect 1834");
    println!("part1 result = {}", part1(&blueprints));
    
    println!();
    println!("note: this will take a while to run; we expect 2240");
    println!("part2 result = {}", part2(&blueprints));
    
    Ok(())
}
