use crate::primitives::vector::{Matrix, Metric, Vector};

use ndarray::Array;

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::iter::zip;

use min_max_heap::MinMaxHeap;

const CHUNK_SIZE: usize = 2;

#[derive(Debug, PartialEq)]
pub struct VectorTable {
    pub dim: usize,
    pub chunking: bool,
    vectors: Vec<Vector>,
    chunks: Vec<Matrix>,
}

// TODO: Add distance computation functionality to this struct
// Right now we're letting other structs reach into this one's data
// instead of encapsulating the vectors and bringing the computation
// to the data

// TODO: Once we've improved the interface to bring the computation inside,
// then we can convert the internal representation to contain a vec of matrices
// and a vec of vectors, converting the vec of vectors to a matrix block when it
// hits a certain size. Then distance computations can be done on the matrix blocks

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
            // self.vectors = self.vectors[chunk_size..].to_vec()
        }
    }

    fn create_chunk(&mut self, chunk_size: usize, drain: bool) -> Matrix {
        assert!(self.vectors.len() > 0, "no vectors to create chunk with");
        assert!(
            chunk_size <= self.vectors.len(),
            "Chunk size is larger than vector table"
        );
        let flat: Vec<f32> = if drain {
            self.vectors.drain(0..chunk_size).flatten().collect()
        } else {
            self.vectors[0..chunk_size]
                .iter()
                .cloned()
                .flatten()
                .collect()
        };

        assert!(flat.len() > 0, "no floats drained from vectors to chunk");
        let shape = (chunk_size, self.dim);

        assert!(
            flat.len() == shape.0 * shape.1,
            "flat len {} doesn't match shape ({},{})",
            flat.len(),
            shape.0,
            shape.1
        );
        Matrix::from_shape_vec(shape, flat).unwrap()
    }

    fn create_temp_chunk(&self, chunk_size: usize) -> Matrix {
        let flat: Vec<f32> = self.vectors[0..chunk_size]
            .iter()
            .cloned()
            .flatten()
            .collect();

        let shape = (chunk_size, self.dim);

        assert!(
            flat.len() == shape.0 * shape.1,
            "flat len {} doesn't match shape ({},{})",
            flat.len(),
            shape.0,
            shape.1
        );
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
        metric_fn: &dyn Fn(&Vector, &Matrix) -> Vec<Metric>,
        vector: &Vector,
        k: usize,
    ) -> Vec<(Metric, usize)> {
        let mut heap: MinMaxHeap<(Metric, usize)> = MinMaxHeap::with_capacity(k);

        assert!(
            self.vectors.len() > 0 || self.chunks.len() > 0,
            "nothing in the index"
        );

        let mut chunks: Vec<&Matrix> = Vec::new();
        for reference in self.chunks.iter() {
            chunks.push(reference);
        }
        let num_vectors = self.vectors.len();

        let vector_chunk = self.create_temp_chunk(num_vectors);
        if self.vectors.len() > 0 {
            chunks.push(&vector_chunk);

            assert!(chunks.len() >= 1, "no chunks found");
            assert!(
                chunks.last().unwrap().shape()[0] == num_vectors,
                "unexpected value for chunk size {} (expected {})",
                chunks.last().unwrap().shape()[0],
                num_vectors
            );
        }

        // Iterate through the chunks computing distances and inserting into the heap
        for (chunk_index, chunk) in chunks.iter().enumerate() {
            assert!(chunk.shape()[0] > 0, "chunk is empty");

            let results: Vec<Metric> = metric_fn(&vector, &chunk);

            assert!(results.len() > 0, "no results found");

            let chunk_pos = chunk_index * CHUNK_SIZE;
            let result_positions = Array::from_iter(0..chunk.shape()[0]) + chunk_pos as usize;

            assert!(result_positions.len() > 0, "no result positions found");
            assert!(
                results.len() == result_positions.len(),
                "results len={} not equal to positions len={}",
                results.len(),
                result_positions.len()
            );

            let result_pairs = zip(results, result_positions);

            assert!(result_pairs.len() > 0, "no result pairs found");

            for (result, pos) in result_pairs {
                if heap.len() >= k {
                    heap.push_pop_min((result, pos));
                } else {
                    heap.push((result, pos));
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
