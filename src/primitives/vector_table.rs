use crate::prelude::*;
use crate::primitives::distances::Metric;

use ndarray::Array;

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::iter::zip;

use min_max_heap::MinMaxHeap;
use ordered_float::OrderedFloat;

const CHUNK_SIZE: usize = 4096;

#[derive(Debug, PartialEq)]
pub struct VectorTable {
    pub dim: usize,
    pub chunking: bool,
    vectors: Vec<Vector>,
    chunks: Vec<Matrix>,
}

impl VectorTable {
    pub fn new(dim: usize, chunking: bool) -> VectorTable {
        VectorTable {
            dim,
            chunking,
            vectors: Vec::<Vector>::new(),
            chunks: Vec::<Matrix>::new(),
        }
    }

    pub fn insert(&mut self, vector: &Vector) {
        self.check_dims(vector);
        self.vectors.push(vector.clone());
        if self.chunking {
            self.condense_vectors(CHUNK_SIZE);
        }
    }

    pub fn insert_many(&mut self, vectors: &[Vector]) {
        for vector in vectors {
            self.check_dims(vector);
        }

        self.vectors.extend_from_slice(vectors);
        if self.chunking {
            self.condense_vectors(CHUNK_SIZE);
        }
    }

    fn condense_vectors(&mut self, chunk_size: usize) {
        while self.vectors.len() >= chunk_size {
            let new_chunk = self.create_chunk(chunk_size, true);

            self.chunks.push(new_chunk);
        }
    }

    fn create_chunk(&mut self, chunk_size: usize, drain: bool) -> Matrix {
        let flat: Vec<f32> = if drain {
            self.vectors.drain(0..chunk_size).flatten().collect()
        } else {
            self.vectors[0..chunk_size]
                .iter()
                .flatten()
                .cloned()
                .collect()
        };

        let shape = (chunk_size, self.dim);

        Matrix::from_shape_vec(shape, flat).unwrap()
    }

    fn check_dims(&mut self, vector: &Vector) {
        assert!(
            vector.len() == self.dim,
            "Vector dim doesn't match table dim"
        );
    }

    pub fn top_k_by_metric(
        &self,
        metric_fn: &dyn Fn(&Vector, &Vector) -> Metric,
        vector: &Vector,
        k: usize,
    ) -> Vec<(Metric, usize)> {
        let mut heap: MinMaxHeap<(Metric, usize)> = MinMaxHeap::with_capacity(k);

        for (pos, index_vector) in self.vectors.iter().enumerate() {
            let result: Metric = metric_fn(&vector, index_vector);
            if heap.len() >= k {
                heap.push_pop_min((result, pos));
            } else {
                heap.push((result, pos));
            }
        }
        heap.into_vec_desc()
    }

    pub fn matrix_top_k_by_metric(
        &mut self,
        metric_fn: &dyn Fn(&Vector, &Matrix) -> Vector,
        vector: &Vector,
        k: usize,
    ) -> Vec<(Metric, usize)> {
        let mut heap: MinMaxHeap<(Metric, usize)> = MinMaxHeap::with_capacity(k);

        // This is fine as long as the number of unchunked vectors is small,
        // but it would be even better to delegate to the other implementation here.
        // For that we'd need references to both distance functions though
        let num_vectors = self.vectors.len();
        let vector_chunk = self.create_chunk(num_vectors, false);

        let mut chunks: Vec<&Matrix> = Vec::new();
        for reference in self.chunks.iter() {
            chunks.push(reference);
        }

        if self.vectors.len() > 0 {
            chunks.push(&vector_chunk);
        }

        // Iterate through the chunks computing distances and inserting into the heap
        for (chunk_index, chunk) in chunks.iter().enumerate() {
            let results: Vector = metric_fn(&vector, &chunk);

            let chunk_pos = chunk_index * CHUNK_SIZE;
            let result_positions = Array::from_iter(0..chunk.shape()[0]) + chunk_pos as usize;
            let result_pairs = zip(results, result_positions);

            for (result, pos) in result_pairs {
                if heap.len() >= k {
                    heap.push_pop_min((OrderedFloat(result), pos));
                } else {
                    heap.push((OrderedFloat(result), pos));
                }
            }
        }

        heap.into_vec_desc()
    }

    pub fn bottom_k_by_metric(
        &self,
        metric_fn: &dyn Fn(&Vector, &Vector) -> Metric,
        vector: &Vector,
        k: usize,
    ) -> Vec<(Metric, usize)> {
        let mut heap: BinaryHeap<(Reverse<Metric>, usize)> = BinaryHeap::with_capacity(k);

        for (pos, index_vector) in self.vectors.iter().enumerate() {
            let result: Metric = metric_fn(&vector, index_vector);
            heap.push((Reverse(result), pos));
        }
        heap.into_sorted_vec()
            .iter()
            .map(|r| (r.0 .0, r.1))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{Vector, VectorTable};
    use crate::primitives::vector::random_vector;

    #[test]
    fn insert_many() {
        let dim = 128;
        let mut vector: Vector = random_vector(dim);

        let vectors: [Vector; 1] = [vector];

        let mut table: VectorTable = VectorTable::new(dim, false);
        table.insert_many(&vectors);

        let mut expected: Option<&Vector> = vectors.get(0);

        assert_eq!(table.vectors.get(0), expected)
    }

    #[test]
    fn top_k_by_metric() {
        // TODO: Test with inner product
    }

    #[test]
    fn bottom_k_by_metric() {
        // TODO: Test with l2 distance
    }
}
