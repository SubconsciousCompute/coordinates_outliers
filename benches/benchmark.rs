use coordinates_outliers::{Point, PointPlane};
use criterion::{criterion_group, criterion_main, Criterion, SamplingMode};
use rand::{thread_rng, Rng};

pub fn bench_point_plane_new(c: &mut Criterion) {
    let mut rng = thread_rng();

    let mut group = c.benchmark_group("PointPlane-flat-sampling");
    group.sampling_mode(SamplingMode::Flat);

    macro_rules! point_plane_new {
        ($id: tt, $capacity: expr) => {
            group.bench_function($id, |b| {
                b.iter(|| {
                    let mut points = Vec::with_capacity($capacity);

                    for _ in 0..$capacity {
                        points.push(Point::new(
                            rng.gen_range(1.123..4.123),
                            rng.gen_range(2.123..5.123),
                        ));
                    }

                    PointPlane::new(points, $capacity)
                })
            });
        };
    }
    point_plane_new!("with 100", 100);
    point_plane_new!("with 1000", 1000);
    point_plane_new!("with 10000", 10000);

    group.finish();
}

criterion_group!(benches, bench_point_plane_new);
criterion_main!(benches);
