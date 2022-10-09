pub mod angular;
pub mod base;
pub mod hyperbolic;
pub mod lp_norm;

use crate::prelude::*;

pub trait Distance {
    fn vector_dist(&self, a: &Vector, b: &Vector) -> f32;

    fn matrix_dist(&self, a: &Vector, b: &Matrix) -> Vector;
}
