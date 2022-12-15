use std::ops::{Add, Sub};

use common::OptionAnyhow;
use regex::Regex;

#[derive(Debug, Clone, Copy, Default, Hash, Eq, PartialEq)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}
impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn signum(self) -> Point {
        Point::new(self.x.signum(), self.y.signum())
    }

    pub fn manhattan_length(self) -> i64 {
        self.x.abs() + self.y.abs()
    }
}
impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

#[derive(Debug,Clone)]
struct Measurement {
    sensor: Point,
    beacon: Point,
    distance: i64,
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Measurement>> {
    let re = Regex::new(r#"Sensor at x=([+-]?\d+), y=([+-]?\d+): closest beacon is at x=([+-]?\d+), y=([+-]?\d+)"#)?;

    input
        .lines()
        .map(|l| {
            let cap = re.captures(l).ok_anyhow()?;

            let sensor_x = cap.get(1).ok_anyhow()?.as_str().parse()?;
            let sensor_y = cap.get(2).ok_anyhow()?.as_str().parse()?;
            let sensor = Point::new(sensor_x, sensor_y);

            let beacon_x = cap.get(3).ok_anyhow()?.as_str().parse()?;
            let beacon_y = cap.get(4).ok_anyhow()?.as_str().parse()?;
            let beacon = Point::new(beacon_x, beacon_y);

            let distance = (beacon - sensor).manhattan_length();

            Ok(Measurement {
                sensor,
                beacon,
                distance,
            })
        })
        .collect()
}

fn main() -> anyhow::Result<()> {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        Sensor at x=9, y=16: closest beacon is at x=10, y=16
        Sensor at x=13, y=2: closest beacon is at x=15, y=3
        Sensor at x=12, y=14: closest beacon is at x=10, y=16
        Sensor at x=10, y=20: closest beacon is at x=10, y=16
        Sensor at x=14, y=17: closest beacon is at x=10, y=16
        Sensor at x=8, y=7: closest beacon is at x=2, y=10
        Sensor at x=2, y=0: closest beacon is at x=2, y=10
        Sensor at x=0, y=11: closest beacon is at x=2, y=10
        Sensor at x=20, y=14: closest beacon is at x=25, y=17
        Sensor at x=17, y=20: closest beacon is at x=21, y=22
        Sensor at x=16, y=7: closest beacon is at x=15, y=3
        Sensor at x=14, y=3: closest beacon is at x=15, y=3
        Sensor at x=20, y=1: closest beacon is at x=15, y=3
    "};

    #[test]
    fn parse_inputs_succeeds() {
        let input = parse_input(TEST_INPUT).unwrap();
        println!("{input:?}");
    }
}
