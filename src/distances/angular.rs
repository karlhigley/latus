use crate::prelude::*;

extern crate ndarray;
extern crate ndarray_rand;

use ndarray::Axis;

use std::ops::Mul;

pub fn inner_product(a: &Vector, b: &Vector) -> f32 {
    a.dot(b)
}

pub fn matrix_inner_product(a: &Vector, b: &Matrix) -> Vector {
    b.dot(a)
}
