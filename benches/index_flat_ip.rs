extern crate latus;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use latus::indexes::index_flat_ip::IndexFlatIP;
use latus::primitives::vector::random_vector;

fn index_flat_ip_query(index: &mut IndexFlatIP, k: usize, dim: usize) {
    let query_vector = random_vector(dim);
    let results = index.query(&query_vector, k);

    assert!(
        results.len() == k,
        "results len={} doesn't match k={}",
        results.len(),
        k,
    );
}

fn index_flat_ip_matrix_query(index: &mut IndexFlatIP, k: usize, dim: usize) {
    let query_vector = random_vector(dim);
    let results = index.matrix_query(&query_vector, k);

    assert!(
        results.len() == k,
        "results len={} doesn't match k={}",
        results.len(),
        k,
    );
}

fn index_flat_ip_chunked_matrix_query(index: &mut IndexFlatIP, k: usize, dim: usize) {
    let query_vector = random_vector(dim);
    let results = index.matrix_query(&query_vector, k);

    assert!(
        results.len() == k,
        "results len={} doesn't match k={}",
        results.len(),
        k,
    );
}

fn query_benchmark(c: &mut Criterion) {
    let index_size = 1_183_514;
    let dim = 100;
    let k = 10;

    let mut index_ip: IndexFlatIP = IndexFlatIP::new(dim, false);
    // let mut index_ip_matrix: IndexFlatIP = IndexFlatIP::new(dim, false);
    let mut index_ip_chunked: IndexFlatIP = IndexFlatIP::new(dim, true);

    for _ in 0..index_size {
        let vector = random_vector(dim);
        index_ip.insert(&vector.clone());
        // index_ip_matrix.insert(&vector.clone());
        index_ip_chunked.insert(&vector.clone());
    }

    c.bench_function("index_flat_ip query", |b| {
        b.iter(|| index_flat_ip_query(black_box(&mut index_ip), black_box(k), black_box(dim)))
    });

    // c.bench_function("index_flat_ip matrix query", |b| {
    //     b.iter(|| {
    //         index_flat_ip_matrix_query(
    //             black_box(&mut index_ip_matrix),
    //             black_box(k),
    //             black_box(dim),
    //         )
    //     })
    // });

    c.bench_function("index_flat_ip chunked matrix query", |b| {
        b.iter(|| {
            index_flat_ip_chunked_matrix_query(
                black_box(&mut index_ip_chunked),
                black_box(k),
                black_box(dim),
            )
        })
    });
}

criterion_group!(benches, query_benchmark);
criterion_main!(benches);
