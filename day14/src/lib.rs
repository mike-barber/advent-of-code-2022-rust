use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, Default, Hash, Eq, PartialEq)]
pub struct Point(pub isize, pub isize);
impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
}
impl Point {
    pub fn signum(&self) -> Point {
        Point(self.0.signum(), self.1.signum())
    }
}



#[derive(Debug, Clone, Copy)]
pub enum Dir {
    U,
    D,
    L,
    R,
}
impl From<Dir> for Point {
    fn from(dir: Dir) -> Self {
        match dir {
            Dir::U => Point(0, 1),
            Dir::D => Point(0, -1),
            Dir::L => Point(-1, 0),
            Dir::R => Point(1, 0),
        }
    }
}

#[derive(Debug)]
pub struct Grid<T> {
    values: Vec<T>,
    width: usize,
    height: usize,
}
impl<T: Clone> Grid<T> {
    pub fn new(width: usize, height: usize, fill_value: T) -> Self {
        Grid {
            width,
            height,
            values: vec![fill_value; 0],
        }
    }
}
impl<T> Grid<T> {
    pub fn get(&self, point: &Point) -> Option<&T> {
        if self.contains(point) {
            let ix = point.0 + point.1 * self.width as isize;
            self.values.get(ix as usize)
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, point: &Point) -> Option<&mut T> {
        if self.contains(point) {
            let ix = point.0 + point.1 * self.width as isize;
            self.values.get_mut(ix as usize)
        } else {
            None
        }
    }

    pub fn contains(&self, point: &Point) -> bool {
        (0..self.width as isize).contains(&point.0) && (0..self.height as isize).contains(&point.1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GridSquare {
    Blank,
    Rock,
    Sand,
}




// TODO: put this in a common location
pub trait OptionAnyhow<T> {
    fn ok_anyhow(self) -> anyhow::Result<T>;
}
impl<T> OptionAnyhow<T> for Option<T> {
    fn ok_anyhow(self) -> anyhow::Result<T> {
        self.ok_or_else(|| anyhow::anyhow!("expected Some value"))
    }
}
