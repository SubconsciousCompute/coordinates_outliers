use coordinates_outliers::{Point, PointPlane};
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

    let a = Point::new(0.123, 0.123);
    let b = Point::new(1.123, 1.123);
    let c = Point::new(2.123, 2.123);
    let d = Point::new(3.123, 3.123);
    let e = Point::new(0.123, 0.123);
    let f = Point::new(1.123, 1.123);

    let points = vec![a, b, c, d, e, f];

    let mut k = PointPlane::new(points, 100);

    println!("{:#?}", k);

    println!("{} {}", k.get_point_frequency(&a), k.get_point_weight(&a));

    k.retain(2);

    println!("{:#?}", k);
}
