use common::{read_file, AnyResult};

use day22::{
    part2::{Connection, Edge, Topology},
    *,
};
use Direction::*;

fn part2_topology() -> AnyResult<Topology> {
    let connections = [
        Connection::new(Edge::new(1, U), Edge::new(6, L)),
        Connection::new(Edge::new(1, D), Edge::new(3, U)),
        Connection::new(Edge::new(1, L), Edge::new(4, L)),
        Connection::new(Edge::new(1, R), Edge::new(2, L)),
        Connection::new(Edge::new(2, U), Edge::new(6, D)),
        Connection::new(Edge::new(2, D), Edge::new(3, R)),
        Connection::new(Edge::new(2, R), Edge::new(5, R)),
        Connection::new(Edge::new(3, D), Edge::new(5, U)),
        Connection::new(Edge::new(3, L), Edge::new(4, U)),
        Connection::new(Edge::new(4, D), Edge::new(6, U)),
        Connection::new(Edge::new(4, R), Edge::new(5, L)),
        Connection::new(Edge::new(5, D), Edge::new(6, R)),
    ];
    Topology::new(&connections)
}

fn main() -> AnyResult<()> {
    let input = read_file("day22/input.txt")?;

    let part1_problem = part1::parse_input(&input)?;
    println!("part1 result: {}", part1::run(&part1_problem));

    let topology = part2_topology()?;
    let part2_problem = part2::parse_input(&input, 50, topology)?;
    println!("part2 result: {}", part2_problem.run());
    println!("note: 109077 is too high");

    Ok(())
}
