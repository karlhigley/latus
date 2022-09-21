use crate::indexes::traits::{Indexable, Queryable};
use crate::primitives::vector::{inner_product, Metric, Vector};
use crate::primitives::vector_table::VectorTable;

// TODO: Refactor this and IndexFlatL2 to implement trait(s)

#[derive(Debug, PartialEq)]
pub struct IndexFlatIP {
    pub table: VectorTable,
}

impl IndexFlatIP {
    pub fn new() -> IndexFlatIP {
        IndexFlatIP {
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
        self.table.top_k_by_metric(&inner_product, vector, k)
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
    use super::{IndexFlatIP, Vector};
    use crate::primitives::vector::random_vector;

    #[test]
    fn insert_many() {
        let vectors: Vec<Vector> = (0..1000).map(|_| random_vector(128)).collect();
        let mut index: IndexFlatIP = IndexFlatIP::new();
        index.insert_many(&vectors);
    }

    #[test]
    fn query() {
        let vectors: Vec<Vector> = (0..1000).map(|_| random_vector(128)).collect();
        let mut index: IndexFlatIP = IndexFlatIP::new();
        index.insert_many(&vectors);

        let query_vector = random_vector(128);
        let result = index.query(&query_vector, 10);
    }
}
