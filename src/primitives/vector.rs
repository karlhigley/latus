extern crate ndarray;
extern crate ndarray_rand;

use ndarray::Array1;
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;
use ordered_float::OrderedFloat;

use std::cmp::Reverse;

pub type Metric = OrderedFloat<f32>;
pub type Vector = Array1<f32>;

pub fn random_vector(dim: usize) -> Vector {
    let dist = Uniform::new(0., 1.);
    let vector = Vector::random((dim,), dist);
    vector
}

pub fn l2_distance(a: &Vector, b: &Vector) -> Metric {
    let sub = a - b;
    OrderedFloat::<f32>((&sub * &sub).sum().sqrt())
}

pub fn inner_product(a: &Vector, b: &Vector) -> Metric {
    OrderedFloat::<f32>((a * b).sum())
}

#[cfg(test)]
mod tests {
    use super::{inner_product, l2_distance, Metric};
    use super::{random_vector, Vector};

    #[test]
    fn ordered_float() {
        use ordered_float::OrderedFloat;
        use std::f32::NAN;

        let mut v = [OrderedFloat(NAN), OrderedFloat(2.0), OrderedFloat(1.0)];
        v.sort();
        assert_eq!(v, [OrderedFloat(1.0), OrderedFloat(2.0), OrderedFloat(NAN)]);
        v.sort();
        v.reverse();
        assert_eq!(v, [OrderedFloat(NAN), OrderedFloat(2.0), OrderedFloat(1.0)]);
    }

    // #[test]
    // fn l2_distance() {}

    // #[test]
    // fn inner_product() {}

    // #[test]
    // fn insert_many() {
    //     let mut vector: Vector = random_vector(128);

    //     let vectors: [Vector; 1] = [vector];

    //     let mut table: VectorTable = VectorTable::new();
    //     table.insert_many(&vectors);

    //     let mut expected: Option<&Vector> = vectors.get(0);

    //     assert_eq!(table.vectors.get(0), expected)
    // }
}
