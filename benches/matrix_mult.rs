use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use matrixmultiply::sgemm;
use nalgebra::DMatrix;
use ndarray::Array2;
use rulinalg::matrix::Matrix;

fn bench_matrixmultiply(n: usize) {
    let a: Vec<f32> = (0..n*n).map(|i| i as f32).collect();
    let b: Vec<f32> = (0..n*n).map(|i| i as f32).collect();
    let mut c = vec![0.0f32; n * n];

    unsafe {
        sgemm(
            n, n, n,
            1.0,
            a.as_ptr(), n as isize, 1,
            b.as_ptr(), n as isize, 1,
            0.0,
            c.as_mut_ptr(), n as isize, 1,
        );
    }
    black_box(c);
}

fn bench_ndarray(n: usize) {
    let a = Array2::from_shape_fn((n, n), |(i, j)| (i * n + j) as f32);
    let b = Array2::from_shape_fn((n, n), |(i, j)| (i * n + j) as f32);
    let c = a.dot(&b);
    black_box(c);
}

fn bench_nalgebra(n: usize) {
    let a = DMatrix::from_fn(n, n, |i, j| (i * n + j) as f32);
    let b = DMatrix::from_fn(n, n, |i, j| (i * n + j) as f32);
    let c = a * b;
    black_box(c);
}

fn bench_rulinalg(n: usize) {
    let data: Vec<f32> = (0..n*n).map(|i| i as f32).collect();
    let a = Matrix::new(n, n, data.clone());
    let b = Matrix::new(n, n, data);
    let c = a * b;
    black_box(c);
}

fn matrix_mult_small(c: &mut Criterion) {
    let mut group = c.benchmark_group("matrix_mult_small");

    group.bench_function("nalgebra", |b| {
        b.iter(|| bench_nalgebra(4))
    });

    group.bench_function("ndarray", |b| {
        b.iter(|| bench_ndarray(4))
    });

    group.bench_function("rulinalg", |b| {
        b.iter(|| bench_rulinalg(4))
    });

    group.bench_function("matrixmultiply", |b| {
        b.iter(|| bench_matrixmultiply(4))
    });

    group.finish();
}

fn matrix_mult_medium(c: &mut Criterion) {
    let mut group = c.benchmark_group("matrix_mult_medium");

    group.bench_function("nalgebra", |b| {
        b.iter(|| bench_nalgebra(64))
    });

    group.bench_function("ndarray", |b| {
        b.iter(|| bench_ndarray(64))
    });

    group.bench_function("rulinalg", |b| {
        b.iter(|| bench_rulinalg(64))
    });

    group.bench_function("matrixmultiply", |b| {
        b.iter(|| bench_matrixmultiply(64))
    });

    group.finish();
}

fn matrix_mult_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("matrix_mult_large");

    group.bench_function("nalgebra", |b| {
        b.iter(|| bench_nalgebra(256))
    });

    group.bench_function("ndarray", |b| {
        b.iter(|| bench_ndarray(256))
    });

    group.bench_function("rulinalg", |b| {
        b.iter(|| bench_rulinalg(256))
    });

    group.bench_function("matrixmultiply", |b| {
        b.iter(|| bench_matrixmultiply(256))
    });

    group.finish();
}


criterion_group!(benches, matrix_mult_small, matrix_mult_medium, matrix_mult_large);
criterion_main!(benches);