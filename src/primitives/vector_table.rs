use crate::primitives::vector::{Metric, Vector};
use std::collections::BinaryHeap;

use std::cmp::Reverse;

#[derive(Debug, PartialEq)]
pub struct VectorTable {
    pub vectors: Vec<Vector>,
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
    pub fn new() -> VectorTable {
        VectorTable {
            vectors: Vec::<Vector>::new(),
        }
    }

    pub fn insert(&mut self, vector: Vector) {
        self.vectors.extend(vec![vector])
    }

    pub fn insert_many(&mut self, vectors: &[Vector]) {
        self.vectors.extend(vectors.to_vec())
    }

    pub fn top_k_by_metric(
        &self,
        metric_fn: &dyn Fn(&Vector, &Vector) -> Metric,
        vector: &Vector,
        k: usize,
    ) -> Vec<(Metric, usize)> {
        let mut heap: BinaryHeap<(Metric, usize)> = BinaryHeap::with_capacity(k);

        for (pos, index_vector) in self.vectors.iter().enumerate() {
            let result: Metric = metric_fn(&vector, index_vector);
            heap.push((result, pos));
        }
        heap.into_sorted_vec()
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
        let mut vector: Vector = random_vector(128);

        let vectors: [Vector; 1] = [vector];

        let mut table: VectorTable = VectorTable::new();
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
