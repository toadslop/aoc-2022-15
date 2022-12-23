use std::{error::Error, fmt::Display};

use crate::{beacon::Beacon, point::Point, sensor::Sensor};

#[derive(Debug)]
pub struct Area {
    center: Point,
    distance_to_edge: isize,
}

impl Area {
    pub fn new(center: Point, distance_to_edge: isize) -> Self {
        Self {
            center,
            distance_to_edge,
        }
    }

    pub fn from_beacon_and_sensor(sensor: Sensor, beacon: Beacon) -> Self {
        let distance_to_edge = Self::calculate_manhattan_distance(&sensor, &beacon);
        Self {
            center: *sensor,
            distance_to_edge,
        }
    }

    pub fn calculate_manhattan_distance(point1: &Point, point2: &Point) -> isize {
        let x1 = point1.get_x();
        let y1 = point1.get_y();
        let x2 = point2.get_x();
        let y2 = point2.get_y();

        (x1 - x2).abs() + (y1 - y2).abs()
    }

    pub fn get_extremities_on_row(&self, row_index: isize) -> (Point, Point) {
        let (point1, point2) = if row_index > self.center.get_y() {
            (
                self.get_missing_x_1(row_index),
                self.get_missing_x_2(row_index),
            )
        } else {
            (
                self.get_missing_x_3(row_index),
                self.get_missing_x_4(row_index),
            )
        };

        (point1, point2)
    }

    fn get_missing_x_1(&self, row_index: isize) -> Point {
        let x1 = self.center.get_x();
        let y1 = self.center.get_y();
        let y2 = row_index;
        let d = self.distance_to_edge;
        let x2 = x1 - y1 + y2 - d;

        Point::new(x2, y2)
    }

    fn get_missing_x_2(&self, row_index: isize) -> Point {
        let x1 = self.center.get_x();
        let y1 = self.center.get_y();
        let y2 = row_index;
        let d = self.distance_to_edge;
        let x2 = d + x1 + y1 - y2;

        Point::new(x2, y2)
    }

    fn get_missing_x_3(&self, row_index: isize) -> Point {
        let x1 = self.center.get_x();
        let y1 = self.center.get_y();
        let y2 = row_index;
        let d = self.distance_to_edge;
        let x2 = x1 + y1 - d - y2;

        Point::new(x2, y2)
    }

    fn get_missing_x_4(&self, row_index: isize) -> Point {
        let x1 = self.center.get_x();
        let y1 = self.center.get_y();
        let y2 = row_index;
        let d = self.distance_to_edge;
        let x2 = x1 - y1 + d + y2;

        Point::new(x2, y2)
    }
}

#[derive(Debug)]
pub struct OutOfBoundsError {
    index: isize,
}

impl OutOfBoundsError {
    pub fn new(index: isize) -> Self {
        Self { index }
    }
}

impl Display for OutOfBoundsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Index {} is out of bounds", self.index,)
    }
}

impl Error for OutOfBoundsError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

#[cfg(test)]
mod test {
    use crate::point::Point;

    use super::Area;

    #[test]
    fn gets_correct_bounds_on_row_greater_than_origin() {
        let area = Area::new(Point::new(5, 5), 3);
        let (actual1, actual2) = area.get_extremities_on_row(6);
        let expected1 = Point::new(3, 6);
        let expected2 = Point::new(7, 6);

        assert_eq!(actual1, expected1);
        assert_eq!(actual2, expected2)
    }

    #[test]
    fn gets_correct_bounds_on_row_less_than_origin() {
        let area = Area::new(Point::new(5, 5), 3);
        let (actual1, actual2) = area.get_extremities_on_row(4);
        let expected1 = Point::new(3, 4);
        let expected2 = Point::new(7, 4);

        assert_eq!(actual1, expected1);
        assert_eq!(actual2, expected2)
    }
}
