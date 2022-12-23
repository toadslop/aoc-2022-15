use std::{collections::HashSet, env, fs};

use area::Area;
use parsing::parse_line;
use point::Point;

mod area;
mod beacon;
mod parsing;
mod point;
mod sensor;

fn main() {
    let args: Vec<_> = env::args().collect();
    let input = load_from_file(&args);
    let row_idx = args
        .get(2)
        .expect("Expected a row idx but got None")
        .parse::<isize>()
        .expect("Expected the provided row idx to be parsable as isize but it wasn't");

    let count = get_count_positions_that_cannot_be_beacons(input.trim(), row_idx);
    println!("POSITION COUNT: {count}");
}

pub fn load_from_file(args: &[String]) -> String {
    let path = args.get(1).expect("No inpute file provided");
    fs::read_to_string(path).expect("Could not read the input file")
}

fn get_count_positions_that_cannot_be_beacons(input: &str, row_idx: isize) -> usize {
    let mut positions = HashSet::new();
    let mut beacon_positions = HashSet::new();
    let max_x = isize::MIN;
    let min_x = isize::MAX;
    for line in input.trim().lines() {
        let (sensor, beacon) = parse_line(line);

        if beacon.get_y() == row_idx {
            beacon_positions.insert(*beacon);
            positions.remove(&*beacon);
        }

        let area = Area::from_beacon_and_sensor(sensor, beacon);
        let (left_extremity, right_extremity) = area.get_extremities_on_row(row_idx);
        if !positions.contains(&left_extremity) || !positions.contains(&right_extremity) {
            for point in Point::into_x_iter(left_extremity..right_extremity) {
                if !beacon_positions.contains(&point) {
                    positions.insert(point);
                }
            }
        }
    }

    positions.len()
}

#[cfg(test)]
mod test {
    use crate::get_count_positions_that_cannot_be_beacons;

    #[test]
    fn gets_correct_count() {
        let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
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
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
        let actual = get_count_positions_that_cannot_be_beacons(input, 10);
        let expected = 26;

        assert_eq!(actual, expected)
    }
}
