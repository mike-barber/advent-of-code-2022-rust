use common::*;
use day19::parser::parse_input;



fn main() -> AnyResult<()> {
    let blueprints = parse_input(&read_file("day19/input.txt")?)?;
    println!("{blueprints:?}");
        
    Ok(())
}

#[cfg(test)]
mod tests {
   
}
