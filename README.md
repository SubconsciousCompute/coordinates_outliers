# location_outliers

-----------------------------------------------------------------------

Find relationships in a series of location assuming `[A,B,A]` implies `A->B->A` and `A,B` are of 
type `Point`.

Simple lib to find outliers or path taken less or more frequently than others.

We store location in form of `Point(x: f64, y: f64)`, they are automatically rounded off to 3 decimal places when using 
`new` method on `Point`. We assume `x,y` are `latitude and longitude` and don't need more than
[3 decimal places of precision](https://gis.stackexchange.com/questions/8650/measuring-accuracy-of-latitude-and-longitude#:~:text=The%20first%20decimal%20place%20is,one%20village%20from%20the%20next.). 

Use `cargo doc --open` for documentation or view it [here](https://docs.rs/location_outliers/latest/location_outliers/index.html)

Relevant [xkcd](https://xkcd.com/2170/)

![location precision](https://imgs.xkcd.com/comics/coordinate_precision.png)

Usage:

```rust
use location_outliers::{Point, PointPlane};

fn main(){
    let a = Point::new(0.123, 0.123);
    let b = Point::new(1.123, 1.123);
    let c = Point::new(2.123, 2.123);
    let d = Point::new(3.123, 3.123);
    let e = Point::new(0.123, 0.123);
    let f = Point::new(1.123, 1.123);

    let points = vec![a, b, c, d, e, f];

    let k = PointPlane::new(points, 100);

    println!("{:#?}", k);
}
```

Output:
```
PointPlane {
    points: {
        "3.123 3.123": 1,
        "1.123 1.123": 2,
        "0.123 0.123": 2,
        "2.123 2.123": 1,
    },
    graph: Graph(
        {
            Connection(
                "3.123-3.123",
                "0.123-0.123",
                Cell {
                    value: 1,
                },
            ),
            Connection(
                "1.123-1.123",
                "2.123-2.123",
                Cell {
                    value: 1,
                },
            ),
            Connection(
                "2.123-2.123",
                "3.123-3.123",
                Cell {
                    value: 1,
                },
            ),
            Connection(
                "0.123-0.123",
                "1.123-1.123",
                Cell {
                    value: 2,
                },
            ),
        },
    ),
    accumulate_xaxis: SimpleAccumulator {
        vec: [
            0.123,
            1.123,
            2.123,
            3.123,
            0.123,
            1.123,
        ],
        mean: 1.289666666666667,
        population_variance: 1.138888888888889,
        min: 0.123,
        min_: 0.123,
        max: 3.123,
        max_: 2.123,
        median: 1.123,
        len: 6,
        capacity: 100,
        fixed_capacity: true,
        last_write_position: 5,
        accumulate: true,
    },
    accumulate_yaxis: SimpleAccumulator {
        vec: [
            0.123,
            1.123,
            2.123,
            3.123,
            0.123,
            1.123,
        ],
        mean: 1.289666666666667,
        population_variance: 1.138888888888889,
        min: 0.123,
        min_: 0.123,
        max: 3.123,
        max_: 2.123,
        median: 1.123,
        len: 6,
        capacity: 100,
        fixed_capacity: true,
        last_write_position: 5,
        accumulate: true,
    },
    capacity: 100,
}
```