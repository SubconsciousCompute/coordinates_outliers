use coordinates_outliers::{Point, PointPlane};
use criterion::{criterion_group, criterion_main, Criterion, SamplingMode};
use rand::{thread_rng, Rng};

pub fn bench_point_plane(c: &mut Criterion) {
    let mut rng = thread_rng();

    let mut group = c.benchmark_group("PointPlane-flat-sampling");
    group.sampling_mode(SamplingMode::Flat);

    macro_rules! point_plane_new {
        ($id: tt, $capacity: expr) => {
            group.bench_function($id, |b| {
                let mut points = Vec::with_capacity($capacity);

                for _ in 0..$capacity {
                    points.push(Point::new(
                        rng.gen_range(1.123..4.123),
                        rng.gen_range(2.123..5.123),
                    ));
                }
                b.iter(|| PointPlane::new(points.clone(), $capacity))
            });
        };
    }
    point_plane_new!("PointPlane::new with 100", 100);
    point_plane_new!("PointPlane::new with 1000", 1000);
    point_plane_new!("PointPlane::new with 10000", 10000);

    macro_rules! point_plane_push {
        ($id: tt, $capacity: expr) => {
            group.bench_function($id, |b| {
                let mut points = Vec::with_capacity($capacity);
                for _ in 0..$capacity {
                    points.push(Point::new(
                        rng.gen_range(1.123..4.123),
                        rng.gen_range(2.123..5.123),
                    ));
                }
                let mut k = PointPlane::new(points, $capacity);

                let mut points = Vec::with_capacity($capacity);

                for _ in 0..$capacity {
                    points.push(Point::new(
                        rng.gen_range(1.12345..1.12350),
                        rng.gen_range(1.12345..1.12350),
                    ));
                }

                b.iter(|| k.push(points.clone()))
            });
        };
    }

    point_plane_push!("PointPlane::push with 100", 100);
    point_plane_push!("PointPlane::push with 1000", 1000);
    point_plane_push!("PointPlane::push with 10000", 10000);

    macro_rules! point_plane_retain {
        ($id: tt, $capacity: expr, $frequency: expr) => {
            group.bench_function($id, |b| {
                let mut points = Vec::with_capacity($capacity);

                for _ in 0..$capacity {
                    points.push(Point::new(
                        rng.gen_range(1.123..4.123),
                        rng.gen_range(2.123..5.123),
                    ));
                }

                let mut k = PointPlane::new(points, $capacity);
                b.iter(|| k.retain($frequency))
            });
        };
    }

    point_plane_retain!("PointPlane::retain with 100 remove <=1", 100, 1);
    point_plane_retain!("PointPlane::retain with 100 remove <=2", 100, 2);
    point_plane_retain!("PointPlane::retain with 100 remove <=3", 100, 3);
    point_plane_retain!("PointPlane::retain with 1000 remove <=1", 1000, 1);
    point_plane_retain!("PointPlane::retain with 1000 remove <=2", 1000, 2);
    point_plane_retain!("PointPlane::retain with 1000 remove <=3", 1000, 3);
    point_plane_retain!("PointPlane::retain with 10000 remove <=1", 10000, 1);
    point_plane_retain!("PointPlane::retain with 10000 remove <=2", 10000, 2);
    point_plane_retain!("PointPlane::retain with 10000 remove <=3", 10000, 3);

    group.finish();
}

criterion_group!(benches, bench_point_plane);
criterion_main!(benches);
