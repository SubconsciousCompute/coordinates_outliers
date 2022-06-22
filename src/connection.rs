//! We represent a directed edge `A->B`, we use the coordinates to compute the hashes but not the
//! weight, with weight being the number of times that path is taken
//!
//! We store coordinates in string as `floats` are not comparable or hashable in a reliable way
//! Note: We round off the floats in `Point` definition itself

use std::cell::Cell;
use std::hash::{Hash, Hasher};

/// A connection represents `(x1, y1) -> (x2, y2)` and how many times it was taken
#[derive(Debug, Clone)]
pub struct Connection(pub(crate) String, pub(crate) String, pub(crate) Cell<usize>);

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
