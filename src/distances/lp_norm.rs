use crate::distances::Distance;
use crate::prelude::*;

#[derive(Debug, PartialEq)]
pub struct L2 {}

impl Distance for L2 {
    fn vector_dist(&self, a: &Vector, b: &Vector) -> f32 {
        let sub = b - a;
        sub.dot(&sub).sqrt()
    }

    fn matrix_dist(&self, a: &Vector, b: &Matrix) -> Vector {
        let sub = b - a;
        sub.dot(&sub).iter().map(|s| s.sqrt()).collect()
    }
}
