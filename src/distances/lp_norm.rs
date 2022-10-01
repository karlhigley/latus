use crate::prelude::*;

extern crate ndarray;
extern crate ndarray_rand;

use ndarray::Axis;

use std::ops::Mul;

pub fn l2_distance(a: &Vector, b: &Vector) -> f32 {
    let sub = b - a;
    sub.dot(&sub).sqrt()
}
