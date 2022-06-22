//! We are making a directed graph of type `A->B`, it updates the edges if it already exists

use crate::connection::Connection;
use std::collections::HashSet;

/// The graph itself is simple, it is a `hashset` of `Connection`
// Can use Vec<HashSet<Connection>> for easy index
#[derive(Debug, Clone)]
pub struct Graph(pub(crate) HashSet<Connection>);

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
