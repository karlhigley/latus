use crate::prelude::*;
use crate::primitives::distances::{inner_product, matrix_inner_product, Metric};
use crate::primitives::vector_table::VectorTable;

// TODO: Refactor this and IndexFlatL2 to implement trait(s)

#[derive(Debug, PartialEq)]
pub struct IndexFlatIP {
    pub table: VectorTable,
}

impl IndexFlatIP {
    pub fn new(dim: usize, chunking: bool) -> IndexFlatIP {
        IndexFlatIP {
            table: VectorTable::new(dim, chunking),
        }
    }

    pub fn insert(&mut self, vector: &Vector) {
        self.table.insert(vector)
    }

    pub fn insert_many(&mut self, vectors: &[Vector]) {
        self.table.insert_many(vectors)
    }

    pub fn query(&mut self, vector: &Vector, k: usize) -> Vec<(Metric, usize)> {
        self.table.top_k_by_metric(&inner_product, vector, k)
    }

    pub fn matrix_query(&mut self, vector: &Vector, k: usize) -> Vec<(Metric, usize)> {
        self.table
            .matrix_top_k_by_metric(&matrix_inner_product, vector, k, false)
    }

    pub fn query_many(&mut self, vectors: &[Vector], k: usize) -> Vec<Vec<(Metric, usize)>> {
        let mut results = Vec::new();
        for query_vector in vectors {
            results.push(self.query(query_vector, k))
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::{IndexFlatIP, Vector};
    use crate::primitives::vector::random_vector;

    #[test]
    fn insert_many() {
        let dim = 128;
        let num_vectors = 1000;

        let vectors: Vec<Vector> = (0..num_vectors).map(|_| random_vector(dim)).collect();
        let mut index: IndexFlatIP = IndexFlatIP::new(dim, false);
        index.insert_many(&vectors);
    }

    #[test]
    fn query() {
        let dim = 128;
        let num_vectors = 1000;
        let k = 10;

        let vectors: Vec<Vector> = (0..num_vectors).map(|_| random_vector(dim)).collect();
        let mut index: IndexFlatIP = IndexFlatIP::new(dim, false);
        index.insert_many(&vectors);

        let query_vector = random_vector(dim);
        let result = index.query(&query_vector, k);
    }
}
