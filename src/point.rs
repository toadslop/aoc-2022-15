use std::{
    cmp::{max, min},
    fmt::Display,
    ops::Range,
};

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub struct Point {
    x: isize,
    y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn get_x(&self) -> isize {
        self.x
    }

    pub fn get_y(&self) -> isize {
        self.y
    }

    pub fn into_x_iter(range: Range<Point>) -> impl Iterator<Item = Point> {
        let x_start = min(range.start.x, range.end.x);
        let x_end = max(range.start.x, range.end.x);
        assert!(range.start.y == range.end.y);
        (x_start..=x_end).map(move |x| Self::new(x, range.start.y))
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}
