use location_outliers::{Point, PointPlane};
use rand::{thread_rng, Rng};

fn main() {
    let mut rng = thread_rng();

    let mut points = Vec::with_capacity(100);

    for _ in 0..100 {
        points.push(Point::new(
            rng.gen_range(1.12345..1.12350),
            rng.gen_range(1.12345..1.12350),
        ));
    }

    let a = Point::new(0.0, 0.0);
    let b = Point::new(1.0, 1.0);
    let c = Point::new(2.0, 2.0);
    let d = Point::new(3.0, 3.0);
    let e = Point::new(0.0, 0.0);
    let f = Point::new(1.0, 1.0);

    let points = vec![a, b, c, d, e, f];

    let k = PointPlane::new(points, 100);

    println!("{:#?}", k);
}
