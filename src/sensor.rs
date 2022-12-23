use std::ops::Deref;

use crate::point::Point;

#[derive(Debug, PartialEq)]
pub struct Sensor(Point);

impl Sensor {
    pub fn new(point: Point) -> Self {
        Self(point)
    }
}

impl Deref for Sensor {
    type Target = Point;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
