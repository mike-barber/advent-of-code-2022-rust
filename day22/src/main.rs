use common::{read_file, AnyResult};

use day22::{
    part2::{Connection, Edge, Position, Topology},
    *,
};
use Direction::*;

fn part2_topology() -> AnyResult<Topology> {
    let connections = [
        Connection::new(Edge::new(1, U), Edge::new(6, L), false),
        Connection::new(Edge::new(1, D), Edge::new(3, U), false),
        Connection::new(Edge::new(1, L), Edge::new(4, L), true),
        Connection::new(Edge::new(1, R), Edge::new(2, L), false),
        Connection::new(Edge::new(2, U), Edge::new(6, D), false),
        Connection::new(Edge::new(2, D), Edge::new(3, R), false),
        Connection::new(Edge::new(2, R), Edge::new(5, R), true),
        Connection::new(Edge::new(3, D), Edge::new(5, U), false),
        Connection::new(Edge::new(3, L), Edge::new(4, U), false),
        Connection::new(Edge::new(4, D), Edge::new(6, U), false),
        Connection::new(Edge::new(4, R), Edge::new(5, L), false),
        Connection::new(Edge::new(5, D), Edge::new(6, R), false),
    ];
    Topology::new(&connections)
}

fn main() -> AnyResult<()> {
    let input = read_file("day22/input.txt")?;

    let part1_problem = part1::parse_input(&input)?;
    println!("part1 result: {}", part1::run(&part1_problem));

    let topology = part2_topology()?;
    let part2_problem = part2::parse_input(&input, 50, topology)?;

    // part2_problem.faces.iter().enumerate().for_each(|(i, f)| {
    //     println!("{i}");
    //     println!("{f}");
    // });

    // check cycles
    for dir in [U, D, L, R] {
        let init_pos = Position {
            face: 0,
            r: 0,
            c: 0,
            dir,
        };
        let mut curr_pos = init_pos;
        for _ in 1..=(50 * 4) {
            curr_pos = part2_problem.next_position(curr_pos);
        }
        assert_eq!(init_pos, curr_pos);
    }

    println!("part2 result: {}", part2_problem.run());
    println!("note: expect 95291 when refactoring");

    Ok(())
}
