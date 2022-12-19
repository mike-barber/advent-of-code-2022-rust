use common::*;
use day19::{parser::parse_input, part1};
//use nalgebra::Vector4;




fn main() -> AnyResult<()> {
    let blueprints = parse_input(&read_file("day19/input.txt")?)?;
    println!("{blueprints:?}");
        
    println!("note: this will take a while to run; we expect 1834");
    println!("part1 result = {}", part1(&blueprints));
    Ok(())
}

#[cfg(test)]
mod tests {
   
}
