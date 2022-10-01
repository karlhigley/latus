use crate::prelude::*;
use crate::primitives::distances::l2_distance;
use crate::primitives::vector_table::{Metric, VectorTable};

#[derive(Debug, PartialEq)]
pub struct IndexFlatL2 {
    pub table: VectorTable,
}

impl IndexFlatL2 {
    pub fn new(dim: usize) -> IndexFlatL2 {
        IndexFlatL2 {
            table: VectorTable::new(dim, false),
        }
    }

    pub fn insert(&mut self, vector: Vector) {
        self.table.insert(&vector)
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
    use super::{IndexFlatL2, Vector};
    use crate::primitives::vector::random_vector;

    #[test]
    fn insert_many() {
        let num_vectors = 1000;
        let dim = 128;

        let vectors: Vec<Vector> = (0..num_vectors).map(|_| random_vector(dim)).collect();
        let mut index: IndexFlatL2 = IndexFlatL2::new(dim);
        index.insert_many(&vectors);
    }

    #[test]
    fn query() {
        let num_vectors = 1000;
        let dim = 128;
        let k = 10;

        let vectors: Vec<Vector> = (0..num_vectors).map(|_| random_vector(dim)).collect();
        let mut index: IndexFlatL2 = IndexFlatL2::new(dim);
        index.insert_many(&vectors);

        let query_vector = random_vector(dim);
        let result = index.query(&query_vector, k);
    }
}
