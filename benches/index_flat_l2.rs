extern crate latus;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use latus::indexes::index_flat_l2::IndexFlatL2;
use latus::primitives::vector::random_vector;

fn index_flat_l2_query(index: &IndexFlatL2, k: usize, dim: usize) {
    let query_vector = random_vector(dim);
    index.query(&query_vector, k);
}

fn query_benchmark(c: &mut Criterion) {
    let index_size = 1_183_514;
    let dim = 100;
    let k = 10;

    let mut index_l2: IndexFlatL2 = IndexFlatL2::new(dim);
    // let mut index_l2_matrix: IndexFlatL2 = IndexFlatL2::new(dim, false);
    // let mut index_l2_chunked: IndexFlatL2 = IndexFlatL2::new(dim);

    for _ in 0..index_size {
        let vector = random_vector(dim);
        index_l2.insert(vector.clone());
        // index_l2_matrix.insert(&vector.clone());
        // index_l2_chunked.insert(&vector.clone());
    }

    c.bench_function("index_flat_l2 query", |b| {
        b.iter(|| index_flat_l2_query(black_box(&mut index_l2), black_box(k), black_box(dim)))
    });

    // c.bench_function("index_flat_l2 matrix query", |b| {
    //     b.iter(|| {
    //         index_flat_l2_matrix_query(
    //             black_box(&mut index_l2_matrix),
    //             black_box(k),
    //             black_box(dim),
    //         )
    //     })
    // });

    // c.bench_function("index_flat_l2 chunked matrix query", |b| {
    //     b.iter(|| {
    //         index_flat_l2_chunked_matrix_query(
    //             black_box(&mut index_l2_chunked),
    //             black_box(k),
    //             black_box(dim),
    //         )
    //     })
    // });
}

criterion_group!(benches, query_benchmark);
criterion_main!(benches);
