//! Our main entry point into the library. Maintains the frequency of points, edges, X and Y axis stats
//! and capacity

use crate::connection::Connection;
use crate::graph::Graph;
use crate::point::Point;
use simple_accumulator::SimpleAccumulator;
use std::cell::Cell;
use std::collections::{HashMap, HashSet};

/// A detailed struct containing summary of all the points, their frequencies and edge-weights
#[derive(Debug, Clone)]
pub struct PointPlane {
    /// We hash the `Points`(upto 3 decimal places) by converting them into `String` due to float numerical limitations
    points: HashMap<String, usize>,
    graph: Graph,
    accumulate_xaxis: SimpleAccumulator,
    accumulate_yaxis: SimpleAccumulator,
    capacity: usize,
}

impl PointPlane {
    /// Create new PointPlane
    ///
    /// If `i < j` ==> `points[i] // A -> points[j] // B` ==> `A->B`
    pub fn new(points: Vec<Point>, capacity: usize) -> Self {
        let mut graph = Graph(HashSet::<Connection>::with_capacity(100));

        let mut frequencies: HashMap<String, usize> = HashMap::with_capacity(100);
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
    pub fn get_points(&self) -> &HashMap<String, usize> {
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
    pub fn get_point_frequency(&self, point: &Point) -> usize {
        *self.points.get(&point.to_string()).unwrap_or(&0usize)
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

    /// We add new points into the existing point place again assuming the `Vec` of `Point`s
    ///
    /// If `i < j` ==> `points[i] // A -> points[j] // B` ==> `A->B`
    pub fn push(&mut self, points: Vec<Point>) {
        *self.points.entry(points[0].to_string()).or_insert(0) += 1;
        self.accumulate_xaxis.push(points[0].x);
        self.accumulate_yaxis.push(points[0].y);

        for k in 1..points.len() {
            self.accumulate_xaxis.push(points[k].x);
            self.accumulate_yaxis.push(points[k].y);
            self.graph.push(&Connection(
                (points[k - 1].x).to_string() + " " + &*(points[k - 1].y).to_string(),
                (points[k].x).to_string() + " " + &*(points[k].y).to_string(),
                Cell::new(0),
            ));

            *self.points.entry(points[k].to_string()).or_insert(0) += 1;
        }
    }

    /// Remove values based how many time it occurs
    pub fn retain(&mut self, frequency: usize) {
        self.points
            .retain(|_, &mut point_frequency| point_frequency != frequency);

        self.graph.0.retain(|k| k.2.get() != frequency);
    }
}
