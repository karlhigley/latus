use crate::distances::Distance;
use crate::prelude::*;

#[derive(Debug, PartialEq)]
pub struct InnerProduct {}

impl Distance for InnerProduct {
    fn vector_dist(&self, a: &Vector, b: &Vector) -> f32 {
        a.dot(b)
    }

    fn matrix_dist(&self, a: &Vector, b: &Matrix) -> Vector {
        b.dot(a)
    }
}
