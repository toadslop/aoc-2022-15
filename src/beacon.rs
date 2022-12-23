use std::ops::Deref;

use crate::point::Point;

#[derive(Debug, PartialEq)]
pub struct Beacon(Point);

impl Beacon {
    pub fn new(point: Point) -> Self {
        Self(point)
    }
}

impl Deref for Beacon {
    type Target = Point;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
