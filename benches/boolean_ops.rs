//! Benchmark for boolean operations
//!
//! This module benchmarks the performance of boolean operations
//! on various geometric shapes.

use breprs::foundation::handle::Handle;
use breprs::modeling::boolean_operations::BooleanOperations;
use breprs::modeling::primitives;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_boolean_union(c: &mut Criterion) {
    c.bench_function("boolean_union", |b| {
        b.iter(|| {
            let box1 = primitives::make_box(
                black_box(1.0),
                black_box(1.0),
                black_box(1.0),
                Some(black_box(breprs::geometry::Point::new(0.0, 0.0, 0.0))),
            );

            let box2 = primitives::make_box(
                black_box(1.0),
                black_box(1.0),
                black_box(1.0),
                Some(black_box(breprs::geometry::Point::new(0.5, 0.5, 0.5))),
            );

            let shape1 = Handle::new(std::sync::Arc::new(box1.shape().clone()));
            let shape2 = Handle::new(std::sync::Arc::new(box2.shape().clone()));

            let boolean_ops = BooleanOperations::new();
            let result = boolean_ops.fuse(&shape1, &shape2);

            black_box(result.shape().clone())
        })
    });
}

fn bench_boolean_intersection(c: &mut Criterion) {
    c.bench_function("boolean_intersection", |b| {
        b.iter(|| {
            let box1 = primitives::make_box(
                black_box(1.0),
                black_box(1.0),
                black_box(1.0),
                Some(black_box(breprs::geometry::Point::new(0.0, 0.0, 0.0))),
            );

            let box2 = primitives::make_box(
                black_box(1.0),
                black_box(1.0),
                black_box(1.0),
                Some(black_box(breprs::geometry::Point::new(0.5, 0.5, 0.5))),
            );

            let shape1 = Handle::new(std::sync::Arc::new(box1.shape().clone()));
            let shape2 = Handle::new(std::sync::Arc::new(box2.shape().clone()));

            let boolean_ops = BooleanOperations::new();
            let result = boolean_ops.common(&shape1, &shape2);

            black_box(result.shape().clone())
        })
    });
}

fn bench_boolean_difference(c: &mut Criterion) {
    c.bench_function("boolean_difference", |b| {
        b.iter(|| {
            let box1 = primitives::make_box(
                black_box(2.0),
                black_box(2.0),
                black_box(2.0),
                Some(black_box(breprs::geometry::Point::new(0.0, 0.0, 0.0))),
            );

            let box2 = primitives::make_box(
                black_box(1.0),
                black_box(1.0),
                black_box(1.0),
                Some(black_box(breprs::geometry::Point::new(0.5, 0.5, 0.5))),
            );

            let shape1 = Handle::new(std::sync::Arc::new(box1.shape().clone()));
            let shape2 = Handle::new(std::sync::Arc::new(box2.shape().clone()));

            let boolean_ops = BooleanOperations::new();
            let result = boolean_ops.cut(&shape1, &shape2);

            black_box(result.shape().clone())
        })
    });
}

criterion_group!(
    benches,
    bench_boolean_union,
    bench_boolean_intersection,
    bench_boolean_difference
);
criterion_main!(benches);
