//! A simple point in 2D plane of type (x,y)

use std::fmt;

/// Custom point type, location is usually in float numeric
#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub(crate) x: f64,
    pub(crate) y: f64,
}

impl Point {
    /// We round off the location to upto `3` decimal places
    pub fn new(x: f64, y: f64) -> Self {
        Point {
            x: f64::trunc(x * 1000.0) / 1000.0,
            y: f64::trunc(y * 1000.0) / 1000.0,
        }
    }
}

/// Pretty display for coordinates, also used to make hashes
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:5} {:5}", self.x, self.y)
    }
}
