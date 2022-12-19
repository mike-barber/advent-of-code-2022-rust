use common::*;
use day19::parser::parse_input;
use nalgebra::Vector4;




fn main() -> AnyResult<()> {
    let blueprints = parse_input(&read_file("day19/input.txt")?)?;
    println!("{blueprints:?}");
        
    Ok(())
}

#[cfg(test)]
mod tests {
   
}
