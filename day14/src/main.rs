use std::{fs::File, io::Read};

use day14::GridSquare::*;
use day14::*;

struct Problem {
    grid: Grid<GridSquare>,
    sand_origin: Point,
}

fn read_file(file_name: &str) -> anyhow::Result<String> {
    let mut contents = String::new();
    File::open(file_name)?.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse_rocks(line: &str) -> anyhow::Result<Vec<Point>> {
    let points = line
        .split(" -> ")
        .map(|seg| {
            let mut seg_iter = seg.split(",");
            let x = seg_iter.next().ok_anyhow()?.parse()?;
            let y = seg_iter.next().ok_anyhow()?.parse()?;
            Ok(Point(x, y))
        })
        .collect();

    points
}

fn parse_input(test_input: &str) -> anyhow::Result<Problem> {
    let lines = test_input.lines().collect::<Vec<_>>();

    let rocks: anyhow::Result<Vec<_>> = lines.iter().map(|line| parse_rocks(line)).collect();
    let rocks = rocks?;
    
    // determine dimensions and create grid
    let max_x = rocks
        .iter()
        .flatten()
        .map(|x| x.0)
        .max()
        .ok_anyhow()?;
    let max_y = rocks
        .iter()
        .flatten()
        .map(|x| x.0)
        .max()
        .ok_anyhow()?;
    let mut grid = Grid::new(max_x as usize + 1, max_y as usize + 1, Blank);

    // populate grid with the rocks
    for rock in rocks {
        for pair in rock.windows(2) {
            if let [a,b] = pair {
                let dir = (*b - *a).signum();
                let mut cur = *a;
                while cur != *b {
                    let entry = grid.get_mut(&cur).ok_anyhow()?;
                    *entry = Rock;
                    cur = cur + dir;
                }
            }
        }
    }

    let sand_origin = Point(500,0);
    Ok(Problem{ 
        grid,
        sand_origin
    })
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        498,4 -> 498,6 -> 496,6
        503,4 -> 502,4 -> 502,9 -> 494,9
    "};

    #[test]
    fn parse_inputs_succeeds() {
        parse_input(TEST_INPUT).unwrap();
    }
}
