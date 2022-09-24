use crate::prelude::*;

extern crate ndarray;
extern crate ndarray_rand;

use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;

pub fn random_vector(dim: usize) -> Vector {
    let dist = Uniform::new(0., 1.);
    let vector = Vector::random((dim,), dist);
    vector
}
