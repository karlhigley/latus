extern crate latus;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use latus::prelude::*;
use latus::primitives::distances::{half_plane_distance, l2_distance, matrix_inner_product};
use latus::primitives::vector::random_vector;
use latus::primitives::vector_table::VectorTable;

fn inner_product_top_k(table: &mut VectorTable, vector: &Vector, k: usize) {
    table.matrix_top_k_by_metric(&matrix_inner_product, vector, k);
}

fn l2_distance_bottom_k(table: &VectorTable, vector: &Vector, k: usize) {
    table.bottom_k_by_metric(&l2_distance, vector, k);
}

fn half_plane_dist_bottom_k(table: &VectorTable, vector: &Vector, k: usize) {
    table.bottom_k_by_metric(&half_plane_distance, vector, k);
}

fn distances_benchmark(c: &mut Criterion) {
    let index_size = 1_048_576;
    let dim = 128;
    let k = 100;

    let vectors: Vec<Vector> = (0..index_size).map(|_| random_vector(dim)).collect();

    let mut vector_table = VectorTable::new(dim, false);
    vector_table.insert_many(&vectors);

    let query_vector = random_vector(dim);

    c.bench_function("inner_product top-k", |b| {
        b.iter(|| {
            inner_product_top_k(
                black_box(&mut vector_table),
                black_box(&query_vector),
                black_box(k),
            )
        })
    });

    c.bench_function("l2_distance bottom-k", |b| {
        b.iter(|| {
            l2_distance_bottom_k(
                black_box(&vector_table),
                black_box(&query_vector),
                black_box(k),
            )
        })
    });

    c.bench_function("half_plane_distance bottom-k", |b| {
        b.iter(|| {
            half_plane_dist_bottom_k(
                black_box(&vector_table),
                black_box(&query_vector),
                black_box(k),
            )
        })
    });
}

criterion_group!(benches, distances_benchmark);
criterion_main!(benches);
