//! Find relationships in a series of location assuming `[A,B,A]` implies `A->B->A` and `A,B` are of
//! type `Point`.
//!
//! Simple lib to find outliers or path taken less or more frequently than others.
//!
//! We store location in form of `Point(x: f64, y: f64)`, they are automatically rounded off to 3 decimal places when using
//! `new` method on `Point`. We assume `x,y` are `latitude and longitude` and don't need more than
//! [3 decimal places of precision](https://gis.stackexchange.com/questions/8650/measuring-accuracy-of-latitude-and-longitude#:~:text=The%20first%20decimal%20place%20is,one%20village%20from%20the%20next.).
//!
//! Relevant [xkcd](https://xkcd.com/2170/)
//!
//! ![location precision](https://imgs.xkcd.com/comics/coordinate_precision.png)
//!
//! Usage:
///
/// ```rust
/// use coordinates_outliers::{Point, PointPlane};
/// let a = Point::new(0.123, 0.123);
/// let b = Point::new(1.123, 1.123);
/// let c = Point::new(2.123, 2.123);
/// let d = Point::new(3.123, 3.123);
/// let e = Point::new(0.123, 0.123);
/// let f = Point::new(1.123, 1.123);
///
/// let points = vec![a, b, c, d, e, f];
///
/// let k = PointPlane::new(points, 100);
///
/// println!("{:#?}", k);
///
use simple_accumulator::SimpleAccumulator;
use std::cell::Cell;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::{Hash, Hasher};

/// Custom point type, location is usually in float numeric
#[derive(Debug, Copy, Clone)]
pub struct Point {
    x: f64,
    y: f64,
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

/// This represents and directed edge `A->B`, we use the coordinates to compute the hashes but not the
/// weight, with weight being the number of times that path is taken
///
/// We store coordinates in string as `floats` are not comparable or hashable in a reliable way
/// Note: We round off the floats in `Point` definition itself
#[derive(Debug, Clone)]
pub struct Connection(String, String, Cell<usize>);

/// Eq for connection
impl Eq for Connection {}

/// Way to compare an edge, Eg: (1.222, 4.123) == (1.222, 4.123) only
impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

/// This will make it easier to find the right connections. i.e. given `1.111` would match with
/// `(1.111, 2.222)` or `(2.222, 1.111)`
impl PartialEq<String> for Connection {
    fn eq(&self, other: &String) -> bool {
        self.0 == *other || self.1 == *other
    }
}

/// Custom hash for `Connection`, we use the String values of `Connection(String, String, Cell<usize>)`
/// to make hashes as `Cell<usize>` is used to keep track of frequency/weight of path aka how many times
/// it was traversed
impl Hash for Connection {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.0.hash(hasher);
        self.1.hash(hasher);
    }
}

/// The graph itself is simple, it is a `hashset` of `Connection`
// Can use Vec<HashSet<Connection>> for easy index
#[derive(Debug, Clone)]
pub struct Graph(HashSet<Connection>);

impl Graph {
    /// We insert a Node (aka Connection) and update the weight if it already exists
    pub fn push(&mut self, connection: &Connection) {
        self.0.insert(connection.clone());
        if let Some(x) = self.0.get(connection) {
            let val = x.2.get();
            x.2.set(val + 1);
        }
    }
}

/// A detailed struct containing summary of all the points, their frequencies and edge-weights
#[derive(Debug, Clone)]
pub struct PointPlane {
    /// We hash the `Points`(upto 5 decimal places) by converting them into `String` due to float numerical limitations
    pub points: HashMap<String, u32>,
    pub graph: Graph,
    pub accumulate_xaxis: SimpleAccumulator,
    pub accumulate_yaxis: SimpleAccumulator,
    capacity: usize,
}

impl PointPlane {
    /// Create new PointPlane
    pub fn new(points: Vec<Point>, capacity: usize) -> Self {
        let mut graph = Graph(HashSet::<Connection>::with_capacity(100));

        let mut frequencies: HashMap<String, u32> = HashMap::with_capacity(100);
        let mut x = Vec::with_capacity(capacity);
        let mut y = Vec::with_capacity(capacity);

        *frequencies.entry(points[0].to_string()).or_insert(0) += 1;
        x.push(points[0].x);
        y.push(points[0].y);

        for k in 1..points.len() {
            x.push(points[k].x);
            y.push(points[k].y);
            graph.push(&Connection(
                (points[k - 1].x).to_string() + "-" + &*(points[k - 1].y).to_string(),
                (points[k].x).to_string() + "-" + &*(points[k].y).to_string(),
                Cell::new(0),
            ));

            *frequencies.entry(points[k].to_string()).or_insert(0) += 1;
        }

        PointPlane {
            points: frequencies,
            graph,
            accumulate_xaxis: SimpleAccumulator::with_fixed_capacity(&x, capacity, true),
            accumulate_yaxis: SimpleAccumulator::with_fixed_capacity(&y, capacity, true),
            capacity,
        }
    }
}
