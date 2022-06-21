//! Simple lib to find outliers or path taken less frequently than others

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
    /// We round off the location to upto 5 decimal places
    pub fn new(x: f64, y: f64) -> Self {
        Point {
            x: f64::trunc(x * 100000.0) / 100000.0,
            y: f64::trunc(y * 100000.0) / 100000.0,
        }
    }
}

/// Pretty display for coordinates, also used to make hashes
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:5} {:5}", self.x, self.y)
    }
}

/// This represents and directed edge A->B, we use the coordinates to compute the hashes but not the
/// weight, with weight being the number of times that path is taken
///
/// We store coordinates in string as `floats` are not comparable or hashable in a reliable way
/// Note: We round off the floats in `Point` definition itself
#[derive(Debug, Clone)]
pub struct Connection(String, String, Cell<usize>);

/// Eq for connection
impl Eq for Connection {}

/// Any way to compare a node, Eg: (1.22222, 4.12345) == (1.22222, 4.12345) == (4.12345, 1.22222)
impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 || self.0 == other.1 && self.1 == other.0
    }
}

/// This will make it easier to find the right connections
impl PartialEq<String> for Connection {
    fn eq(&self, other: &String) -> bool {
        self.0 == *other || self.1 == *other
    }
}

/// Custom hash for `Connection`
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
        let mut graph = Graph(HashSet::<Connection>::new());

        let mut frequencies: HashMap<String, u32> = HashMap::new();
        let mut x = Vec::with_capacity(capacity);
        let mut y = Vec::with_capacity(capacity);

        *frequencies.entry(points[0].to_string()).or_insert(0) += 1;
        x.push(points[0].x);
        y.push(points[0].y);

        for k in 1..points.len() {
            x.push(points[k].x);
            y.push(points[k].y);
            graph.push(&Connection(
                (points[k - 1].x * 100000.0).to_string()
                    + "-"
                    + &*(points[k - 1].y * 100000.0).to_string(),
                (points[k].x * 100000.0).to_string() + "-" + &*(points[k].y * 100000.0).to_string(),
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
