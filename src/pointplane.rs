use crate::connection::Connection;
use crate::graph::Graph;
use crate::point::Point;
use simple_accumulator::SimpleAccumulator;
use std::cell::Cell;
use std::collections::{HashMap, HashSet};

/// A detailed struct containing summary of all the points, their frequencies and edge-weights
#[derive(Debug, Clone)]
pub struct PointPlane {
    /// We hash the `Points`(upto 5 decimal places) by converting them into `String` due to float numerical limitations
    points: HashMap<String, u32>,
    graph: Graph,
    accumulate_xaxis: SimpleAccumulator,
    accumulate_yaxis: SimpleAccumulator,
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
                (points[k - 1].x).to_string() + " " + &*(points[k - 1].y).to_string(),
                (points[k].x).to_string() + " " + &*(points[k].y).to_string(),
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

    /// Function to get the unique points in the plane
    pub fn get_points(&self) -> &HashMap<String, u32> {
        &self.points
    }

    /// Function to get the graph of points, contains `A->B`
    pub fn get_graph(&self) -> &Graph {
        &self.graph
    }

    /// Function to get the x-axis accumulator
    pub fn get_accumulate_xaxis(&self) -> &SimpleAccumulator {
        &self.accumulate_xaxis
    }

    /// Function to get the y-axis accumulator
    pub fn get_accumulate_yaxis(&self) -> &SimpleAccumulator {
        &self.accumulate_yaxis
    }

    /// Function to get the capacity of the plane (i.e. capacity of accumulator)
    pub fn get_capacity(&self) -> usize {
        self.capacity
    }

    /// Function to get the frequency of a point in the plane
    pub fn get_point_frequency(&self, point: &Point) -> u32 {
        *self.points.get(&point.to_string()).unwrap_or(&0u32)
    }

    /// Function to get the weight of an point in the plane (i.e. how many times it was used to traverse)
    pub fn get_point_weight(&self, point: &Point) -> usize {
        let mut weight = 0;
        for x in self.graph.0.iter() {
            if x.0 == point.to_string() || x.1 == point.to_string() {
                weight += x.2.get();
            }
        }
        weight
    }
}
