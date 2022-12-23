use lazy_static::lazy_static;
use regex::Regex;

use crate::{beacon::Beacon, point::Point, sensor::Sensor};

pub fn parse_line(line: &str) -> (Sensor, Beacon) {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^Sensor at (x=-?\d+, y=-?\d+): closest beacon is at (x=-?\d+, y=-?\d+)$")
                .unwrap();
    }

    let captures = RE.captures(line).expect("Expected the line to capture");

    let sensor_cap = captures.get(1).expect("Expected a catpure for the sensor");
    let beacon_cap = captures.get(2).expect("Expected a catpure for the beacon");

    let sensor_point = parse_point(sensor_cap.as_str());
    let beacon_point = parse_point(beacon_cap.as_str());

    (Sensor::new(sensor_point), Beacon::new(beacon_point))
}

fn parse_point(point_str: &str) -> Point {
    let mut parts = point_str.split(", ");
    let x_str = parts.next().expect("Expected an X part but got None");
    let y_str = parts.next().expect("Expected a Y part but got None");

    let x = x_str
        .split("x=")
        .nth(1)
        .expect("Expected value after x= but got None")
        .parse::<isize>()
        .expect("Expected the x-value to be parsable to isize but it wasn't");

    let y = y_str
        .split("y=")
        .nth(1)
        .expect("Expected value after y= but got None")
        .parse::<isize>()
        .expect("Expected the y-value to be parsable to isize but it wasn't");

    Point::new(x, y)
}

#[cfg(test)]
mod test {
    use crate::{beacon::Beacon, point::Point, sensor::Sensor};

    use super::{parse_line, parse_point};

    #[test]
    fn parses_a_point_from_input() {
        let input = "x=2, y=18";

        let actual = parse_point(input);
        let expected = Point::new(2, 18);

        assert_eq!(actual, expected)
    }

    #[test]
    fn parses_a_line() {
        let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15";

        let (actual_sensor, actual_beacon) = parse_line(input);
        let expected_sensor = Sensor::new(Point::new(2, 18));
        let expected_beacon = Beacon::new(Point::new(-2, 15));

        assert_eq!(actual_sensor, expected_sensor);
        assert_eq!(actual_beacon, expected_beacon);
    }
}
