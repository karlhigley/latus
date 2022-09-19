use std::cmp::Reverse;
use std::collections::BinaryHeap;

use super::super::primitives::vector::{l2_distance, Metric, Vector};
use super::super::primitives::vector_table::VectorTable;

#[derive(Debug, PartialEq)]
pub struct IndexFlatL2 {
    pub table: VectorTable,
}

impl IndexFlatL2 {
    pub fn new() -> IndexFlatL2 {
        IndexFlatL2 {
            table: VectorTable::new(),
        }
    }

    pub fn insert(&mut self, vector: Vector) {
        self.table.insert(vector)
    }

    pub fn insert_many(&mut self, vectors: &[Vector]) {
        self.table.insert_many(vectors)
    }

    pub fn query(&self, vector: &Vector, k: usize) -> Vec<(Metric, usize)> {
        self.table.bottom_k_by_metric(&l2_distance, vector, k)
    }

    pub fn query_many(&self, vectors: &[Vector], k: usize) -> Vec<Vec<(Metric, usize)>> {
        let mut results = Vec::new();
        for query_vector in vectors {
            results.push(self.query(query_vector, k))
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::primitives::vector::random_vector;
    use super::{IndexFlatL2, Vector};

    #[test]
    fn insert_many() {
        let vectors: Vec<Vector> = (0..1000).map(|_| random_vector(128)).collect();
        let mut index: IndexFlatL2 = IndexFlatL2::new();
        index.insert_many(&vectors);
    }

    #[test]
    fn query() {
        let vectors: Vec<Vector> = (0..1000).map(|_| random_vector(128)).collect();
        let mut index: IndexFlatL2 = IndexFlatL2::new();
        index.insert_many(&vectors);

        let query_vector = random_vector(128);
        let result = index.query(&query_vector, 10);
    }
}
