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
//!
//! ```rust
//! use coordinates_outliers::{Point, PointPlane};
//!
//! let a = Point::new(0.123, 0.123);
//! let b = Point::new(1.123, 1.123);
//! let c = Point::new(2.123, 2.123);
//! let d = Point::new(3.123, 3.123);
//! let e = Point::new(0.123, 0.123);
//! let f = Point::new(1.123, 1.123);
//!
//! let points = vec![a, b, c, d, e, f];
//!
//! let k = PointPlane::new(points, 100);
//!
//! println!("{:#?}", k);
//!
pub mod connection;
pub mod graph;
pub mod point;
pub mod pointplane;
pub use connection::Connection;
pub use graph::Graph;
pub use point::Point;
pub use pointplane::PointPlane;
