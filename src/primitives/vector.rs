extern crate ndarray;
extern crate ndarray_rand;

use ndarray::Array1;
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;

use std::cmp::Ordering;

#[derive(PartialOrd, PartialEq)]
pub struct Metric(f32);

impl Eq for Metric {}

impl Ord for Metric {
    fn cmp(&self, other: &Self) -> Ordering {
        if let Some(ordering) = self.partial_cmp(other) {
            ordering
        } else {
            Ordering::Less
        }
    }
}

pub type Vector = Array1<f32>;

pub fn random_vector(dim: usize) -> Vector {
    let dist = Uniform::new(0., 1.);
    let vector = Vector::random((dim,), dist);
    vector
}

pub fn l2_distance(a: &Vector, b: &Vector) -> Metric {
    let sub = a - b;
    Metric((&sub * &sub).sum().sqrt())
}

pub fn inner_product(a: &Vector, b: &Vector) -> Metric {
    Metric((a * b).sum())
}

// TODO: Write tests for these methods
