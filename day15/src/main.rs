use std::{
    collections::HashSet,
    default,
    ops::{Add, RangeInclusive, Sub},
};

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
        manhattan_length(self.x, self.y)
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

fn manhattan_length(x: i64, y: i64) -> i64 {
    x.abs() + y.abs()
}

#[derive(Debug, Clone)]
struct Measurement {
    sensor: Point,
    beacon: Point,
    distance: i64,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct Range(i64, i64);
impl Range {
    fn try_merge(&self, other: Range) -> Option<Range> {
        let left = self.0 - 1;
        let right = self.0 + 1;
        if other.0 <= right || other.1 >= left {
            Some(Range(self.0.min(other.0), self.1.max(other.1)))
        } else {
            None
        }
    }

    fn contains(&self, value: i64) -> bool {
        value >= self.0 && value <= self.1
    }

    fn new(left: i64, right: i64) -> Range {
        Range(left, right)
    }

    fn width(&self) -> i64 {
        self.1 - self.0
    }
}

#[derive(Default)]
struct Cover(Vec<Range>);

impl Cover {
    fn push_range(&mut self, range: Range) {
        let ranges = &mut self.0;

        // merge with any existing ranges
        let mut already_merged = false;
        for r in ranges.iter_mut() {
            if let Some(merged) = r.try_merge(range) {
                *r = merged;
                already_merged = true;
                break;
            }
        }

        // early exit if we didn't merge with any other range
        if !already_merged {
            ranges.push(range);
            return;
        }

        // if we've merged, then we need to consider further merges
        loop {
            let mut merge_occurred = false;
            for i in 0..ranges.len() - 1 {
                for j in 1..ranges.len() {
                    if let Some(merge) = ranges[i].try_merge(ranges[j]) {
                        ranges[i] = merge;
                        ranges[j] = merge;
                        merge_occurred = true;
                    }
                }
            }

            // we've merged everything we can
            if !merge_occurred {
                break;
            }
        }

        // finally, remove duplicates (although this could be more efficient)
        ranges.sort_unstable();
        ranges.dedup();
    }
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Measurement>> {
    let re = Regex::new(
        r#"Sensor at x=([+-]?\d+), y=([+-]?\d+): closest beacon is at x=([+-]?\d+), y=([+-]?\d+)"#,
    )?;

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

fn part1(measurements: &[Measurement], reference_row: i64) -> usize {
    let mut line_covered: HashSet<i64> = HashSet::new();
    for m in measurements {
        let x = m.sensor.x;
        let y = m.sensor.y;
        for dx in 0.. {
            let dist = manhattan_length(dx, y - reference_row);
            if dist > m.distance {
                break;
            } else {
                line_covered.insert(x - dx);
                line_covered.insert(x + dx);
            }
        }
    }

    // exclude beacons on this line
    for m in measurements.iter().filter(|m| m.beacon.y == reference_row) {
        line_covered.remove(&m.beacon.x);
    }

    // let mut coords: Vec<_> = line_covered.iter().collect();
    // coords.sort();
    // println!("coords: {coords:?}");
    line_covered.len()
}

fn part1_alt(measurements: &[Measurement], reference_row: i64) -> usize {
    let mut line_covered = Cover::default();
    for m in measurements {
        let x = m.sensor.x;
        let y = m.sensor.y;
        let dist_y = (y - reference_row).abs();
        let dx = m.distance - dist_y;
        if dx >= 0 {
            let range = Range::new(x - dx, x + dx);
            line_covered.push_range(range);
        }
    }

    // exclude beacons on this line
    let mut exclude_count = 0;
    for m in measurements.iter().filter(|m| m.beacon.y == reference_row) {
        for range in &line_covered.0 {
            if range.contains(m.beacon.x) {
                exclude_count += 1;
            }
        }
    }

    // total cover - beacon
    line_covered.0.iter().map(|r| r.width() as usize).sum::<usize>() - exclude_count
}

fn main() -> anyhow::Result<()> {
    let input = parse_input(&common::read_file("input.txt")?)?;

    println!("part1 result: {}", part1(&input, 2000000));
    println!("part1 alt result: {}", part1_alt(&input, 2000000));

    Ok(())
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

    #[test]
    fn part1_correct() {
        let input = parse_input(TEST_INPUT).unwrap();
        let res = part1(&input, 10);
        assert_eq!(res, 26);
    }

    #[test]
    fn part1_alt_correct() {
        let input = parse_input(TEST_INPUT).unwrap();
        let res = part1_alt(&input, 10);
        assert_eq!(res, 26);
    }
}
