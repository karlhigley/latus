extern crate ndarray;
extern crate ndarray_rand;

use ndarray::{Array1, Array2};
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;
use ordered_float::OrderedFloat;

pub type Metric = OrderedFloat<f32>;
pub type Vector = Array1<f32>;
pub type Matrix = Array2<f32>;

pub fn random_vector(dim: usize) -> Vector {
    let dist = Uniform::new(0., 1.);
    let vector = Vector::random((dim,), dist);
    vector
}

pub fn l2_distance(a: &Vector, b: &Vector) -> Metric {
    let sub = b - a;
    OrderedFloat::<f32>(sub.dot(&sub).sqrt())
}

pub fn inner_product(a: &Vector, b: &Vector) -> Metric {
    OrderedFloat::<f32>(a.dot(b))
}

pub fn matrix_inner_product(a: &Vector, b: &Matrix) -> Vec<Metric> {
    let result = b.dot(a);
    result.iter().map(|r| OrderedFloat::<f32>(*r)).collect()
}

// "y" is the half-plane dimension
// "x" is all the rest of the dimensions
pub fn half_plane_distance(a: &Vector, b: &Vector) -> Metric {
    // Extract the half-plane dimension and
    // form vectors from the remaining dimensions
    let (a_y, a_x) = split_dims(&a);
    let (b_y, b_x) = split_dims(&b);

    assert!(a_y >= &0., "Half plane dimension is negative");
    assert!(b_y >= &0., "Half plane dimension is negative");

    let a_x = Vector::from_vec(a_x.to_vec());
    let b_x = Vector::from_vec(b_x.to_vec());

    // Compute diffs between points along
    // the half-plane and remaining dimensions
    let y_diff = b_y - a_y;
    let y_diff_ref = b_y + a_y;

    let x_diff = b_x - a_x;
    let x_diff_squared = x_diff.dot(&x_diff);

    // Magnitude of the diffs with raw y coord
    // and y coord reflected across the plane
    let diff_mag = (x_diff_squared + y_diff * y_diff).sqrt();
    let diff_mag_ref = (x_diff_squared + y_diff_ref * y_diff_ref).sqrt();

    // Put it all together to compute the distance
    let numerator = diff_mag + diff_mag_ref;
    let denominator = 2. * (a_y * b_y).sqrt();
    let dist = 2. * (numerator / denominator).ln();

    OrderedFloat::<f32>(dist)
}

fn split_dims<'a>(v: &'a Vector) -> (&'a f32, &'a [f32]) {
    v.as_slice()
        .unwrap()
        .split_last()
        .expect("couldn't slice vector")
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
